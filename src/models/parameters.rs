// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2021 Farhan Ahmed. All rights reserved.
//

use super::adjustments::TimeAdjustment;
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
pub struct Parameters {
    pub method: Method,
    pub fajr_angle: f64,
    pub isha_angle: f64,
    pub isha_interval: i32,
    pub madhab: Madhab,
    pub high_latitude_rule: HighLatitudeRule,
    pub adjustments: TimeAdjustment,
    pub method_adjustments: TimeAdjustment,
}

impl Parameters {
    pub fn new(fajr_angle: f64, isha_angle: f64) -> Parameters {
        Parameters {
            fajr_angle: fajr_angle,
            isha_angle: isha_angle,
            method: Method::Other,
            isha_interval: 0,
            madhab: Madhab::Shafi,
            high_latitude_rule: HighLatitudeRule::MiddleOfTheNight,
            adjustments: TimeAdjustment::default(),
            method_adjustments: TimeAdjustment::default(),
        }
    }

    pub fn night_portions(&self) -> (f64, f64) {
        match self.high_latitude_rule {
            HighLatitudeRule::MiddleOfTheNight => (1.0 / 2.0, 1.0 / 2.0),
            HighLatitudeRule::SeventhOfTheNight => (1.0 / 7.0, 1.0 / 7.0),
            HighLatitudeRule::TwilightAngle => (self.fajr_angle / 60.0, self.isha_angle / 60.0),
        }
    }

    pub fn time_adjustments(&self, prayer: Prayer) -> i64 {
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

/// A builder for the the [Parameters](struct.Parameters.html).
///
/// It is recommended that this is used for setting
/// all parameters that are needed.
pub struct Configuration {
    method: Method,
    fajr_angle: f64,
    isha_angle: f64,
    isha_interval: i32,
    madhab: Madhab,
    high_latitude_rule: HighLatitudeRule,
    adjustments: TimeAdjustment,
    method_adjustments: TimeAdjustment,
}

impl Configuration {
    pub fn new(fajr_angle: f64, isha_angle: f64) -> Configuration {
        Configuration {
            fajr_angle: fajr_angle,
            isha_angle: isha_angle,
            method: Method::Other,
            isha_interval: 0,
            madhab: Madhab::Shafi,
            high_latitude_rule: HighLatitudeRule::MiddleOfTheNight,
            adjustments: TimeAdjustment::default(),
            method_adjustments: TimeAdjustment::default(),
        }
    }

    pub fn with(method: Method, madhab: Madhab) -> Parameters {
        let mut params = method.parameters();
        params.madhab = madhab;

        params
    }

    pub fn method<'a>(&'a mut self, method: Method) -> &'a mut Configuration {
        self.method = method;
        self
    }

    pub fn method_adjustments<'a>(
        &'a mut self,
        method_adjustments: TimeAdjustment,
    ) -> &'a mut Configuration {
        self.method_adjustments = method_adjustments;
        self
    }

    pub fn high_latitude_rule<'a>(
        &'a mut self,
        high_latitude_rule: HighLatitudeRule,
    ) -> &'a mut Configuration {
        self.high_latitude_rule = high_latitude_rule;
        self
    }

    pub fn madhab<'a>(&'a mut self, madhab: Madhab) -> &'a mut Configuration {
        self.madhab = madhab;
        self
    }

    pub fn isha_interval<'a>(&'a mut self, isha_interval: i32) -> &'a mut Configuration {
        self.isha_angle = 0.0;
        self.isha_interval = isha_interval;
        self
    }

    pub fn done(&self) -> Parameters {
        Parameters {
            fajr_angle: self.fajr_angle,
            isha_angle: self.isha_angle,
            method: self.method,
            isha_interval: self.isha_interval,
            madhab: self.madhab,
            high_latitude_rule: self.high_latitude_rule,
            adjustments: self.adjustments,
            method_adjustments: self.method_adjustments,
        }
    }
}

#[cfg(test)]
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
        let params = Configuration::new(18.0, 18.0)
            .high_latitude_rule(HighLatitudeRule::SeventhOfTheNight)
            .done();

        assert_eq!(params.night_portions().0, 1.0 / 7.0);
        assert_eq!(params.night_portions().1, 1.0 / 7.0);
    }

    #[test]
    fn calculated_night_portions_twilight_angle() {
        let params = Configuration::new(10.0, 15.0)
            .high_latitude_rule(HighLatitudeRule::TwilightAngle)
            .done();

        assert_eq!(params.night_portions().0, 10.0 / 60.0);
        assert_eq!(params.night_portions().1, 15.0 / 60.0);
    }

    #[test]
    fn parameters_using_method_and_madhab() {
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);

        assert_eq!(params.method, Method::NorthAmerica);
        assert_eq!(params.fajr_angle, 15.0);
        assert_eq!(params.isha_angle, 15.0);
        assert_eq!(params.isha_interval, 0);
        assert_eq!(params.madhab, Madhab::Hanafi);
    }
}
