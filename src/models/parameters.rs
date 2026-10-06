// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::adjustments::TimeAdjustment;
use super::high_altitude_rule::HighLatitudeRule;
use super::madhab::Madhab;
use super::method::Method;
use super::prayer::Prayer;
use super::rounding::Rounding;
use super::shafaq::Shafaq;

/// Complete settings used to determine prayer times.
///
/// Use [`Configuration`] or [`Configuration::with`] to construct values with
/// the method defaults and adjustments applied.
///
/// # Examples
///
/// ```
/// use salah::{Madhab, Method, Parameters};
///
/// let mut parameters = Method::NorthAmerica.parameters();
/// parameters.madhab = Madhab::Hanafi;
/// assert_eq!(parameters.fajr_angle, 15.0);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Parameters {
    /// Preset that supplied these values.
    pub method: Method,
    /// Sun depression angle for Fajr, in degrees.
    pub fajr_angle: f64,
    /// Sun depression angle for Maghrib, in degrees, when used by a method.
    pub maghrib_angle: f64,
    /// Sun depression angle for Isha, in degrees.
    pub isha_angle: f64,
    /// Minutes after Maghrib for interval-based Isha methods.
    pub isha_interval: i32,
    /// Asr shadow-length convention.
    pub madhab: Madhab,
    /// Approximation used when high-latitude twilight is unavailable.
    pub high_latitude_rule: HighLatitudeRule,
    /// User-supplied minute adjustments.
    pub adjustments: TimeAdjustment,
    /// Adjustments bundled with the selected method.
    pub method_adjustments: TimeAdjustment,
    /// Minute rounding policy.
    pub rounding: Rounding,
    /// Twilight interpretation used by the Moonsighting Committee method.
    pub shafaq: Shafaq,
}

impl Parameters {
    /// Create custom parameters with Fajr and Isha solar angles.
    ///
    /// Angles are positive magnitudes in degrees below the horizon. This
    /// constructor starts with [`Method::Other`] and zero method adjustments.
    ///
    /// # Examples
    ///
    /// ```
    /// use salah::Parameters;
    ///
    /// let parameters = Parameters::new(18.0, 18.0);
    /// assert_eq!(parameters.fajr_angle, 18.0);
    /// assert_eq!(parameters.isha_angle, 18.0);
    /// ```
    pub fn new(fajr_angle: f64, isha_angle: f64) -> Parameters {
        Parameters {
            fajr_angle: fajr_angle,
            maghrib_angle: 0.0,
            isha_angle: isha_angle,
            method: Method::Other,
            isha_interval: 0,
            madhab: Madhab::Shafi,
            high_latitude_rule: HighLatitudeRule::MiddleOfTheNight,
            adjustments: TimeAdjustment::default(),
            method_adjustments: TimeAdjustment::default(),
            rounding: Rounding::Nearest,
            shafaq: Shafaq::General,
        }
    }

    /// Return the fractions of night allocated to Fajr and Isha fallbacks.
    pub fn night_portions(&self) -> (f64, f64) {
        match self.high_latitude_rule {
            HighLatitudeRule::MiddleOfTheNight => (1.0 / 2.0, 1.0 / 2.0),
            HighLatitudeRule::SeventhOfTheNight => (1.0 / 7.0, 1.0 / 7.0),
            HighLatitudeRule::TwilightAngle => (self.fajr_angle / 60.0, self.isha_angle / 60.0),
        }
    }

    /// Return the combined user and method adjustment for a prayer.
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

/// A builder for [`Parameters`] with chainable customization methods.
///
/// # Examples
///
/// ```
/// use salah::{Configuration, HighLatitudeRule, Madhab, Rounding, Shafaq};
///
/// let parameters = Configuration::new(18.0, 18.0)
///     .madhab(Madhab::Hanafi)
///     .high_latitude_rule(HighLatitudeRule::SeventhOfTheNight)
///     .rounding(Rounding::Up)
///     .shafaq(Shafaq::Abyad)
///     .done();
///
/// assert_eq!(parameters.madhab, Madhab::Hanafi);
/// ```
pub struct Configuration {
    method: Method,
    fajr_angle: f64,
    maghrib_angle: f64,
    isha_angle: f64,
    isha_interval: i32,
    madhab: Madhab,
    high_latitude_rule: HighLatitudeRule,
    adjustments: TimeAdjustment,
    method_adjustments: TimeAdjustment,
    rounding: Rounding,
    shafaq: Shafaq,
}

impl Configuration {
    /// Create a custom configuration with Fajr and Isha solar angles.
    ///
    /// # Examples
    ///
    /// ```
    /// use salah::Configuration;
    ///
    /// let configuration = Configuration::new(18.0, 18.0);
    /// assert_eq!(configuration.done().fajr_angle, 18.0);
    /// ```
    pub fn new(fajr_angle: f64, isha_angle: f64) -> Configuration {
        Configuration {
            fajr_angle: fajr_angle,
            maghrib_angle: 0.0,
            isha_angle: isha_angle,
            method: Method::Other,
            isha_interval: 0,
            madhab: Madhab::Shafi,
            high_latitude_rule: HighLatitudeRule::MiddleOfTheNight,
            adjustments: TimeAdjustment::default(),
            method_adjustments: TimeAdjustment::default(),
            rounding: Rounding::Nearest,
            shafaq: Shafaq::General,
        }
    }

    /// Start with a named method preset and choose the Asr convention.
    ///
    /// This is the shortest path for the usual case. The returned
    /// [`Parameters`] value can still be customized through its public fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use salah::{Configuration, Madhab, Method};
    ///
    /// let parameters = Configuration::with(Method::MuslimWorldLeague, Madhab::Shafi);
    /// assert_eq!(parameters.method, Method::MuslimWorldLeague);
    /// assert_eq!(parameters.madhab, Madhab::Shafi);
    /// ```
    pub fn with(method: Method, madhab: Madhab) -> Parameters {
        let mut params = method.parameters();
        params.madhab = madhab;

        params
    }

    /// Select the named method preset.
    pub fn method<'a>(&'a mut self, method: Method) -> &'a mut Configuration {
        self.method = method;
        self
    }

    /// Add method-specific minute adjustments to the calculated times.
    pub fn method_adjustments<'a>(
        &'a mut self,
        method_adjustments: TimeAdjustment,
    ) -> &'a mut Configuration {
        self.method_adjustments = method_adjustments;
        self
    }

    /// Select the fallback rule for high-latitude Fajr and Isha.
    pub fn high_latitude_rule<'a>(
        &'a mut self,
        high_latitude_rule: HighLatitudeRule,
    ) -> &'a mut Configuration {
        self.high_latitude_rule = high_latitude_rule;
        self
    }

    /// Select the Asr shadow-length convention.
    pub fn madhab<'a>(&'a mut self, madhab: Madhab) -> &'a mut Configuration {
        self.madhab = madhab;
        self
    }

    /// Use an interval after Maghrib for Isha instead of an Isha angle.
    pub fn isha_interval<'a>(&'a mut self, isha_interval: i32) -> &'a mut Configuration {
        self.isha_angle = 0.0;
        self.isha_interval = isha_interval;
        self
    }

    /// Set the solar depression angle used for Maghrib by applicable methods.
    pub fn maghrib_angle<'a>(&'a mut self, angle: f64) -> &'a mut Configuration {
        self.maghrib_angle = angle;
        self
    }

    /// Select how calculated times are rounded to minutes.
    pub fn rounding<'a>(&'a mut self, value: Rounding) -> &'a mut Configuration {
        self.rounding = value;
        self
    }

    /// Select the twilight interpretation used by the Moonsighting Committee method.
    pub fn shafaq<'a>(&'a mut self, value: Shafaq) -> &'a mut Configuration {
        self.shafaq = value;
        self
    }

    /// Finish the builder and return immutable calculation parameters.
    pub fn done(&self) -> Parameters {
        Parameters {
            fajr_angle: self.fajr_angle,
            maghrib_angle: self.maghrib_angle,
            isha_angle: self.isha_angle,
            method: self.method,
            isha_interval: self.isha_interval,
            madhab: self.madhab,
            high_latitude_rule: self.high_latitude_rule,
            adjustments: self.adjustments,
            method_adjustments: self.method_adjustments,
            rounding: self.rounding,
            shafaq: self.shafaq,
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
