// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::adjustments::TimeAdjustments;
use super::high_altitude_rule::HighLatitudeRule;
use super::madhab::Madhab;
use super::method::Method;
use super::prayer::Prayer;

/// Settings that are used for determining the
/// the correct prayer time.
///
/// It is recommended to use [Configuration](struct.Configuration.html) to build
/// the parameters that are need.
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Parameters {
    pub method: Method,
    pub fajr_angle: f64,
    pub isha_angle: f64,
    pub isha_interval: i32,
    pub madhab: Madhab,
    pub high_latitude_rule: HighLatitudeRule,
    pub adjustments: TimeAdjustments,
    pub method_adjustments: TimeAdjustments,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            fajr_angle: 0.,
            isha_angle: 0.,
            method: Method::Other,
            isha_interval: 0,
            madhab: Madhab::Shafi,
            high_latitude_rule: HighLatitudeRule::MiddleOfTheNight,
            adjustments: TimeAdjustments::default(),
            method_adjustments: TimeAdjustments::default(),
        }
    }
}

impl Parameters {
    pub fn new(fajr_angle: f64, isha_angle: f64) -> Parameters {
        Parameters {
            fajr_angle,
            isha_angle,
            ..Default::default()
        }
    }

    pub fn with(method: Method, madhab: Madhab) -> Parameters {
        let mut params = method.parameters();
        params.madhab = madhab;
        params
    }

    pub fn night_portions(&self) -> (f64, f64) {
        match self.high_latitude_rule {
            HighLatitudeRule::MiddleOfTheNight => (1.0 / 2.0, 1.0 / 2.0),
            HighLatitudeRule::SeventhOfTheNight => (1.0 / 7.0, 1.0 / 7.0),
            HighLatitudeRule::TwilightAngle => (self.fajr_angle / 60.0, self.isha_angle / 60.0),
        }
    }

    pub fn time_adjustment(&self, prayer: Prayer) -> i64 {
        match prayer {
            Prayer::Fajr => self.adjustments.fajr + self.method_adjustments.fajr,
            Prayer::Sunrise => self.adjustments.sunrise + self.method_adjustments.sunrise,
            Prayer::Dhuhr => self.adjustments.dhuhr + self.method_adjustments.dhuhr,
            Prayer::Asr => self.adjustments.asr + self.method_adjustments.asr,
            Prayer::Maghrib => self.adjustments.maghrib + self.method_adjustments.maghrib,
            Prayer::Isha => self.adjustments.isha + self.method_adjustments.isha,
            _ => 0,
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn calculate_parameters_with_fajr_and_isha_angles() {
        let params = Parameters::new(18.0, 18.0);

        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 18.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn calculated_night_portions_middle_of_the_night() {
        let params = Parameters::new(18.0, 18.0);

        assert_eq!(params.night_portions().0, 1.0 / 2.0);
        assert_eq!(params.night_portions().1, 1.0 / 2.0);
    }

    #[test]
    fn calculated_night_portions_seventh_of_the_night() {
        let params = Parameters {
            fajr_angle: 18.,
            isha_angle: 18.,
            high_latitude_rule: HighLatitudeRule::SeventhOfTheNight,
            ..Default::default()
        };

        assert_eq!(params.night_portions().0, 1.0 / 7.0);
        assert_eq!(params.night_portions().1, 1.0 / 7.0);
    }

    #[test]
    fn calculated_night_portions_twilight_angle() {
        let params = Parameters {
            fajr_angle: 10.,
            isha_angle: 15.,
            high_latitude_rule: HighLatitudeRule::TwilightAngle,
            ..Default::default()
        };

        assert_eq!(params.night_portions().0, 10.0 / 60.0);
        assert_eq!(params.night_portions().1, 15.0 / 60.0);
    }

    #[test]
    fn parameters_using_method_and_madhab() {
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);

        assert_eq!(params.method, Method::NorthAmerica);
        assert_eq!(params.fajr_angle, 15.0);
        assert_eq!(params.isha_angle, 15.0);
        assert_eq!(params.isha_interval, 0);
        assert_eq!(params.madhab, Madhab::Hanafi);
    }
}
