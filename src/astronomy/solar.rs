// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

use chrono::{Date, DateTime, Datelike, TimeZone, Utc};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::astronomy::ops;
//use crate::astronomy::unit::Stride;
use crate::astronomy::unit::{Angle, Coordinates};

#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SolarCoordinates {
    // The declination of the sun, the angle between
    // the rays of the Sun and the plane of the Earth's equator.
    declination: Angle,

    // Right ascension of the Sun, the angular distance on the
    // celestial equator from the vernal equinox to the hour circle.
    right_ascension: Angle,

    // Apparent sidereal time, the hour angle of the vernal equinox.
    apparent_sidereal_time: Angle,
}

impl SolarCoordinates {
    fn new<Tz: TimeZone>(date: Date<Tz>) -> Self {
        let julian_day = ops::julian_day(&date.and_hms(0, 0, 0));
        let julian_century = ops::julian_century(julian_day);
        let mean_solar_longitude = ops::mean_solar_longitude(julian_century);
        let mean_lunar_longitude = ops::mean_lunar_longitude(julian_century);
        let ascending_lunar_node = ops::ascending_lunar_node_longitude(julian_century);
        let apparent_solar_longitude =
            ops::apparent_solar_longitude(julian_century, mean_solar_longitude).radians();

        let mean_sidereal_time = ops::mean_sidereal_time(julian_century);
        let nutation_longitude = ops::nutation_in_longitude(
            mean_solar_longitude,
            mean_lunar_longitude,
            ascending_lunar_node,
        );
        let nutation_obliq = ops::nutation_in_obliquity(
            mean_solar_longitude,
            mean_lunar_longitude,
            ascending_lunar_node,
        );

        let mean_obliq_ecliptic = ops::mean_obliquity_of_the_ecliptic(julian_century);
        let apparent_obliq_ecliptic =
            ops::apparent_obliquity_of_the_ecliptic(julian_century, mean_obliq_ecliptic).radians();

        // Equation from Astronomical Algorithms page 165
        let declination = Angle::from_radians(
            (apparent_obliq_ecliptic.sin() * apparent_solar_longitude.sin()).asin(),
        );

        // Equation from Astronomical Algorithms page 165
        let right_ascension = Angle::from_radians(
            (apparent_obliq_ecliptic.cos() * apparent_solar_longitude.sin())
                .atan2(apparent_solar_longitude.cos()),
        )
        .unwound();

        // Equation from Astronomical Algorithms page 88
        let apparent_sidereal_time = Angle::new(
            mean_sidereal_time.degrees
                + ((nutation_longitude * 3600.0)
                    * Angle::new(mean_obliq_ecliptic.degrees + nutation_obliq)
                        .radians()
                        .cos())
                    / 3600.0,
        );

        SolarCoordinates {
            declination,
            right_ascension,
            apparent_sidereal_time,
        }
    }
}

// Solar Time
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SolarTime {
    date: Date<Utc>,
    observer: Coordinates,
    solar: SolarCoordinates,
    pub transit: DateTime<Utc>,
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    prev_solar: SolarCoordinates,
    next_solar: SolarCoordinates,
    approx_transit: f64,
}

impl SolarTime {
    pub fn new(date: Date<Utc>, coordinates: Coordinates) -> SolarTime {
        // All calculation need to occur at 0h0m UTC
        let today = Utc.ymd(date.year(), date.month(), date.day());
        let tomorrow = today.succ();
        let yesterday = today.pred();
        let prev_solar = SolarCoordinates::new(yesterday);
        let solar = SolarCoordinates::new(today);
        let next_solar = SolarCoordinates::new(tomorrow);
        let solar_altitude = Angle::new(-50.0 / 60.0);
        let approx_transit = ops::approximate_transit(
            coordinates.longitude_angle(),
            solar.apparent_sidereal_time,
            solar.right_ascension,
        );
        let transit_time = ops::corrected_transit(
            approx_transit,
            coordinates.longitude_angle(),
            solar.apparent_sidereal_time,
            solar.right_ascension,
            prev_solar.right_ascension,
            next_solar.right_ascension,
        );
        let sunrise_time = ops::corrected_hour_angle(
            approx_transit,
            solar_altitude,
            coordinates,
            false,
            solar.apparent_sidereal_time,
            solar.right_ascension,
            prev_solar.right_ascension,
            next_solar.right_ascension,
            solar.declination,
            prev_solar.declination,
            next_solar.declination,
        );
        let sunset_time = ops::corrected_hour_angle(
            approx_transit,
            solar_altitude,
            coordinates,
            true,
            solar.apparent_sidereal_time,
            solar.right_ascension,
            prev_solar.right_ascension,
            next_solar.right_ascension,
            solar.declination,
            prev_solar.declination,
            next_solar.declination,
        );

        SolarTime {
            date,
            observer: coordinates,
            solar,
            transit: SolarTime::setting_hour(transit_time, &date).unwrap(),
            sunrise: SolarTime::setting_hour(sunrise_time, &date).unwrap(),
            sunset: SolarTime::setting_hour(sunset_time, &date).unwrap(),
            prev_solar,
            next_solar,
            approx_transit,
        }
    }

    pub fn time_for_solar_angle(&self, angle: Angle, after_transit: bool) -> DateTime<Utc> {
        let hours = ops::corrected_hour_angle(
            self.approx_transit,
            angle,
            self.observer,
            after_transit,
            self.solar.apparent_sidereal_time,
            self.solar.right_ascension,
            self.prev_solar.right_ascension,
            self.next_solar.right_ascension,
            self.solar.declination,
            self.prev_solar.declination,
            self.next_solar.declination,
        );

        SolarTime::setting_hour(hours, &self.date).unwrap()
    }

    pub fn afternoon(&self, shadow_length: f64) -> DateTime<Utc> {
        let absolute_degrees = (self.observer.latitude - self.solar.declination.degrees).abs();
        let tangent = Angle::new(absolute_degrees);
        let inverse = shadow_length + tangent.radians().tan();
        let angle = Angle::from_radians((1.0 / inverse).atan());

        self.time_for_solar_angle(angle, true)
    }

    fn setting_hour(value: f64, date: &Date<Utc>) -> Option<DateTime<Utc>> {
        if value.is_normal() {
            let calculated_hours = value.floor();
            let calculated_minutes = ((value - calculated_hours) * 60.0).floor();
            let calculated_seconds =
                ((value - (calculated_hours + calculated_minutes / 60.0)) * 60.0 * 60.0).floor();

            // Adjust the hour to be within 0..=23,
            // wrapping around as needed; otherwise
            // chrono method will panic.
            let (adjusted_hour, adjusted_date) = if calculated_hours < 0.0 {
                ((calculated_hours + 24.0) as u32, date.pred())
            } else if calculated_hours >= 24.0 {
                ((calculated_hours - 24.0) as u32, date.succ())
            } else {
                (calculated_hours as u32, *date)
            };

            // Round to the nearest minute
            let adjusted_mins = (calculated_minutes + calculated_seconds / 60.0).round() as u32;
            let adjusted_secs: u32 = 0;

            Some(adjusted_date.and_hms(adjusted_hour, adjusted_mins, adjusted_secs))
        } else {
            None
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::astronomy::ops;
    use chrono::{Datelike, Local, TimeZone, Utc};

    #[test]
    fn solar_coordinates() {
        let solar = SolarCoordinates::new(Utc.ymd(1992, 10, 13));

        assert_eq!(solar.declination.degrees, -7.7850685152648795);
        assert_eq!(solar.right_ascension.degrees, 198.380_822_142_518_8);
        assert_eq!(
            solar.right_ascension.unwound().degrees,
            198.380_822_142_518_8
        );
    }

    #[test]
    fn zero_out_time_for_a_date() {
        // Local date below is 2019-01-11T04:41:19Z in UTC
        let utc_date = Utc.ymd(2019, 1, 11).and_hms(23, 41, 19);
        let updated = Utc
            .ymd(utc_date.year(), utc_date.month(), utc_date.day())
            .and_hms(0, 0, 0);

        assert_eq!(updated, Utc.ymd(2019, 1, 11).and_hms(0, 0, 0));
    }

    #[test]
    fn calculate_date_for_tomorrow() {
        let date = Local.ymd(2019, 1, 10);
        let tomorrow = date.succ();

        assert_eq!(tomorrow.year(), 2019);
        assert_eq!(tomorrow.month(), 1);
        assert_eq!(tomorrow.day(), 11);
    }

    #[test]
    fn calculate_julian_date() {
        let local = Utc.ymd(1992, 10, 13).and_hms(0, 0, 0);
        let utc = local.with_timezone(&Utc);
        let julian_day = ops::julian_day_ymdh(1992, 10, 13, 0.0);

        assert_eq!(ops::julian_day(&utc), julian_day);
    }

    #[test]
    fn calculate_solar_time() {
        let coordinates = Coordinates::new(35.0 + 47.0 / 60.0, -78.0 - 39.0 / 60.0);
        let date = Utc.ymd(2015, 7, 12);
        let solar = SolarTime::new(date, coordinates);
        let transit_date = Utc.ymd(2015, 7, 12).and_hms(17, 20, 0);
        let sunrise_date = Utc.ymd(2015, 7, 12).and_hms(10, 8, 0);
        let sunset_date = Utc.ymd(2015, 7, 13).and_hms(0, 32, 0);

        assert_eq!(solar.transit, transit_date);
        assert_eq!(solar.sunrise, sunrise_date);
        assert_eq!(solar.sunset, sunset_date);
    }

    #[test]
    fn calculate_time_for_solar_angle() {
        let coordinates = Coordinates::new(35.0 + 47.0 / 60.0, -78.0 - 39.0 / 60.0);
        let date = Utc.ymd(2015, 7, 12);
        let solar = SolarTime::new(date, coordinates);
        let angle = Angle::new(-6.0);
        let twilight_start = solar.time_for_solar_angle(angle, false);
        let twilight_end = solar.time_for_solar_angle(angle, true);

        assert_eq!(twilight_start.format("%-k:%M").to_string(), "9:38");
        assert_eq!(twilight_end.format("%-k:%M").to_string(), "1:02");
    }

    #[test]
    fn calculate_corrected_hour_angle() {
        let coordinates = Coordinates::new(35.0 + 47.0 / 60.0, -78.0 - 39.0 / 60.0);
        let date = Utc.ymd(2015, 7, 12).and_hms(0, 0, 0);
        let today = Utc.ymd(date.year(), date.month(), date.day());
        let tomorrow = today.succ();
        let yesterday = today.pred();
        let prev_solar = SolarCoordinates::new(yesterday);
        let solar = SolarCoordinates::new(today);
        let next_solar = SolarCoordinates::new(tomorrow);
        let solar_altitude = Angle::new(-50.0 / 60.0);
        let approx_transit = ops::approximate_transit(
            coordinates.longitude_angle(),
            solar.apparent_sidereal_time,
            solar.right_ascension,
        );
        let _transit_time = ops::corrected_transit(
            approx_transit,
            coordinates.longitude_angle(),
            solar.apparent_sidereal_time,
            solar.right_ascension,
            prev_solar.right_ascension,
            next_solar.right_ascension,
        );
        let sunrise_time = ops::corrected_hour_angle(
            approx_transit,
            solar_altitude,
            coordinates,
            false,
            solar.apparent_sidereal_time,
            solar.right_ascension,
            prev_solar.right_ascension,
            next_solar.right_ascension,
            solar.declination,
            prev_solar.declination,
            next_solar.declination,
        );

        assert_eq!(sunrise_time, 10.131_800_480_632_85);
    }
}
