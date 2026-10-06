// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

//! Calculate Islamic prayer times and the Qibla bearing from a location and date.
//!
//! Salah uses astronomical solar-position calculations. Fajr and Isha can be
//! defined by the sun's depression angle below the horizon (for example, 18°
//! means the sun is 18° below the horizon), or Isha can be defined as a
//! number of minutes after Maghrib by selecting a method that uses an
//! interval. Maghrib and sunrise use the standard solar event calculations
//! and Dhuhr is solar transit. Asr is derived from the selected [`Madhab`]:
//! the Hanafi shadow-length rule produces a later time than the Shafi rule.
//!
//! Results are returned as [`DateTime<Utc>`]. The input [`NaiveDate`] is a
//! calendar date in the observer's intended local calendar, while the
//! astronomical calculation itself is performed in UTC. Convert each result
//! to the observer's timezone with `with_timezone` before displaying it; the
//! crate does not infer a timezone from longitude.
//!
//! At high latitudes, twilight may not reach the configured solar angle.
//! [`HighLatitudeRule`] supplies a night-fraction approximation for Fajr and
//! Isha in that case. The [`Shafaq`] setting selects which twilight
//! interpretation the Moonsighting Committee method uses for Isha.
//!
//! The [`Qiblah`] type provides the initial great-circle bearing from the
//! supplied coordinates to the Kaaba in Makkah, measured clockwise from true
//! north in degrees.
//!
//! This implementation is based on the [Adhan](https://github.com/batoulapps/Adhan)
//! library by Batoul Apps and uses equations from Jean Meeus's
//! *Astronomical Algorithms*.
//!
//! # Examples
//!
//! Calculate a schedule with a named method and format one result in a local
//! timezone:
//!
//! ```
//! use salah::prelude::*;
//!
//! let new_york_city = Coordinates::new(40.7128, -74.0059);
//! let date          = NaiveDate::from_ymd_opt(2019, 1, 25).expect("Invalid date provided");
//! let params        = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
//! let prayers       = PrayerSchedule::new()
//!                       .on(date)
//!                       .for_location(new_york_city)
//!                       .with_configuration(params)
//!                       .calculate()
//!                       .expect("all schedule inputs are present");
//!
//! let eastern = chrono::FixedOffset::west_opt(5 * 60 * 60).unwrap();
//! let fajr = prayers.time(Prayer::Fajr).with_timezone(&eastern);
//! assert_eq!(fajr.date_naive(), NaiveDate::from_ymd_opt(2019, 1, 25).unwrap());
//! ```
//!
//! # Features
//!
//! Enable the `serde` feature to derive `Serialize` and `Deserialize` for the
//! public calculation and result models:
//!
//! ```toml
//! [dependencies]
//! salah = { version = "0.7.6", features = ["serde"] }
//! ```

mod astronomy;
mod models;
mod schedule;

pub use crate::astronomy::qiblah::Qiblah;
pub use crate::astronomy::unit::{Coordinates, Stride};
pub use crate::models::adjustments::{Adjustment, TimeAdjustment};
pub use crate::models::high_altitude_rule::HighLatitudeRule;
pub use crate::models::madhab::Madhab;
pub use crate::models::method::Method;
pub use crate::models::parameters::{Configuration, Parameters};
pub use crate::models::prayer::Prayer;
pub use crate::models::rounding::Rounding;
pub use crate::models::shafaq::Shafaq;
pub use crate::schedule::{PrayerSchedule, PrayerTimes};
pub use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDate, TimeZone, Timelike, Utc, Weekday,
};

/// A convenience module appropriate for glob imports (`use salah::prelude::*;`).
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::astronomy::qiblah::Qiblah;
    #[doc(no_inline)]
    pub use crate::astronomy::unit::{Coordinates, Stride};
    #[doc(no_inline)]
    pub use crate::models::adjustments::{Adjustment, TimeAdjustment};
    #[doc(no_inline)]
    pub use crate::models::high_altitude_rule::HighLatitudeRule;
    #[doc(no_inline)]
    pub use crate::models::madhab::Madhab;
    #[doc(no_inline)]
    pub use crate::models::method::Method;
    #[doc(no_inline)]
    pub use crate::models::parameters::{Configuration, Parameters};
    #[doc(no_inline)]
    pub use crate::models::prayer::Prayer;
    #[doc(no_inline)]
    pub use crate::models::rounding::Rounding;
    #[doc(no_inline)]
    pub use crate::models::shafaq::Shafaq;
    #[doc(no_inline)]
    pub use crate::schedule::{PrayerSchedule, PrayerTimes};
    #[doc(no_inline)]
    pub use chrono::{
        DateTime, Datelike, Duration, Local, NaiveDate, TimeZone, Timelike, Utc, Weekday,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn calculate_prayer_times() {
        let local_date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let schedule = PrayerTimes::new(local_date, coordinates, params);

        assert_eq!(
            schedule.time(Prayer::Fajr).format("%-l:%M %p").to_string(),
            "8:42 AM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Sunrise)
                .format("%-l:%M %p")
                .to_string(),
            "10:08 AM"
        );
        assert_eq!(
            schedule.time(Prayer::Dhuhr).format("%-l:%M %p").to_string(),
            "5:21 PM"
        );
        assert_eq!(
            schedule.time(Prayer::Asr).format("%-l:%M %p").to_string(),
            "10:22 PM"
        );
        assert_eq!(
            schedule
                .time(Prayer::Maghrib)
                .format("%-l:%M %p")
                .to_string(),
            "12:32 AM"
        );
        assert_eq!(
            schedule.time(Prayer::Isha).format("%-l:%M %p").to_string(),
            "1:57 AM"
        );
    }

    #[test]
    fn calculate_times_using_the_builder_successfully() {
        let date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let result = PrayerSchedule::new()
            .on(date)
            .for_location(coordinates)
            .with_configuration(params)
            .calculate();

        match result {
            Ok(schedule) => {
                assert_eq!(
                    schedule.time(Prayer::Fajr).format("%-l:%M %p").to_string(),
                    "8:42 AM"
                );
                assert_eq!(
                    schedule
                        .time(Prayer::Sunrise)
                        .format("%-l:%M %p")
                        .to_string(),
                    "10:08 AM"
                );
                assert_eq!(
                    schedule.time(Prayer::Dhuhr).format("%-l:%M %p").to_string(),
                    "5:21 PM"
                );
                assert_eq!(
                    schedule.time(Prayer::Asr).format("%-l:%M %p").to_string(),
                    "10:22 PM"
                );
                assert_eq!(
                    schedule
                        .time(Prayer::Maghrib)
                        .format("%-l:%M %p")
                        .to_string(),
                    "12:32 AM"
                );
                assert_eq!(
                    schedule.time(Prayer::Isha).format("%-l:%M %p").to_string(),
                    "1:57 AM"
                );
            }

            Err(_err) => assert!(false),
        }
    }

    #[test]
    fn calculate_times_using_the_builder_failure() {
        let date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let result = PrayerSchedule::new()
            .on(date)
            .with_configuration(params)
            .calculate();

        assert!(
            result.is_err(),
            "We were expecting an error, but received data."
        );
    }

    #[test]
    fn calculate_qiyam_times() {
        let date = NaiveDate::from_ymd_opt(2015, 7, 12).expect("Invalid date provided");
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let result = PrayerSchedule::new()
            .on(date)
            .for_location(coordinates)
            .with_configuration(params)
            .calculate();

        match result {
            Ok(schedule) => {
                // Today's Maghrib: 2015-07-13T00:32:00Z
                // Tomorrow's Fajr: 2015-07-13T08:43:00Z
                // Middle of Night: 2015-07-13T04:38:00Z
                // Last Third     : 2015-07-13T05:59:00Z
                assert_eq!(
                    schedule
                        .time(Prayer::Maghrib)
                        .format("%-l:%M %p")
                        .to_string(),
                    "12:32 AM"
                );
                assert_eq!(
                    schedule.time(Prayer::Qiyam).format("%-l:%M %p").to_string(),
                    "5:59 AM"
                );
            }
            Err(_err) => assert!(false),
        }
    }

    #[test]
    fn calculate_times_for_singapore() {
        let mut params = Configuration::with(Method::Singapore, Madhab::Shafi);

        params.high_latitude_rule = HighLatitudeRule::MiddleOfTheNight;

        let result = PrayerSchedule::new()
            .on(NaiveDate::from_ymd_opt(2021, 1, 13).expect("Invalid date provided"))
            .for_location(Coordinates::new(1.370844612058886, 103.80145644060552))
            .with_configuration(params)
            .calculate();

        match result {
            Ok(schedule) => {
                let hour = 3600;
                let sgt_offset = FixedOffset::east_opt(8 * hour).expect("Invalid offset provided");
                let sgt_fajr = schedule.time(Prayer::Fajr).with_timezone(&sgt_offset);
                let sgt_sunrise = schedule.time(Prayer::Sunrise).with_timezone(&sgt_offset);
                let sgt_dhuhr = schedule.time(Prayer::Dhuhr).with_timezone(&sgt_offset);
                let sgt_asr = schedule.time(Prayer::Asr).with_timezone(&sgt_offset);
                let sgt_maghrib = schedule.time(Prayer::Maghrib).with_timezone(&sgt_offset);
                let sgt_isha = schedule.time(Prayer::Isha).with_timezone(&sgt_offset);

                assert_eq!(sgt_fajr.format("%-l:%M %p").to_string(), "5:50 AM");
                assert_eq!(sgt_sunrise.format("%-l:%M %p").to_string(), "7:13 AM");
                assert_eq!(sgt_dhuhr.format("%-l:%M %p").to_string(), "1:15 PM");
                assert_eq!(sgt_asr.format("%-l:%M %p").to_string(), "4:39 PM");
                assert_eq!(sgt_maghrib.format("%-l:%M %p").to_string(), "7:16 PM");
                assert_eq!(sgt_isha.format("%-l:%M %p").to_string(), "8:30 PM");
            }
            Err(_err) => assert!(false),
        }
    }

    #[test]
    fn calculate_times_for_jakarta() {
        let mut params = Configuration::with(Method::Egyptian, Madhab::Shafi);

        // The adjustment below are based on the prayer times that are provided
        // on the website (https://www.jadwalsholat.org/). I don't know the exact
        // calculation that this site is using for the prayer times; therefore the
        // use of time adjustment.
        //
        // It would be a good idea to get some more information on how the Fajr
        // and Isha are calculated, since that's where the biggest variance is;
        // however, the other times are within the 2 minute variance.
        params.method_adjustments = Adjustment::new()
            .fajr(-10)
            .sunrise(-2)
            .dhuhr(2)
            .asr(1)
            .maghrib(2)
            .isha(4)
            .done();

        let result = PrayerSchedule::new()
            .on(NaiveDate::from_ymd_opt(2021, 1, 12).expect("Invalid date provided"))
            .for_location(Coordinates::new(-6.18233995, 106.84287154))
            .with_configuration(params)
            .calculate();

        match result {
            Ok(schedule) => {
                let hour = 3600;
                let wib_offset = FixedOffset::east_opt(7 * hour).expect("Invalid offset provided");
                let wib_fajr = schedule.time(Prayer::Fajr).with_timezone(&wib_offset);
                let wib_sunrise = schedule.time(Prayer::Sunrise).with_timezone(&wib_offset);
                let wib_dhuhr = schedule.time(Prayer::Dhuhr).with_timezone(&wib_offset);
                let wib_asr = schedule.time(Prayer::Asr).with_timezone(&wib_offset);
                let wib_maghrib = schedule.time(Prayer::Maghrib).with_timezone(&wib_offset);
                let wib_isha = schedule.time(Prayer::Isha).with_timezone(&wib_offset);

                assert_eq!(wib_fajr.format("%-l:%M %p").to_string(), "4:15 AM");
                assert_eq!(wib_sunrise.format("%-l:%M %p").to_string(), "5:45 AM");
                assert_eq!(wib_dhuhr.format("%-l:%M %p").to_string(), "12:03 PM");
                assert_eq!(wib_asr.format("%-l:%M %p").to_string(), "3:28 PM");
                assert_eq!(wib_maghrib.format("%-l:%M %p").to_string(), "6:16 PM");
                assert_eq!(wib_isha.format("%-l:%M %p").to_string(), "7:31 PM");
            }
            Err(_err) => assert!(false),
        }
    }

    #[test]
    fn calculate_time_for_kuala_lumpur() {
        let location = Coordinates::new(3.12, 101.69);
        let date = NaiveDate::from_ymd_opt(2024, 4, 9).expect("Invalid date provided");
        let params = Configuration::with(Method::MuslimWorldLeague, Madhab::Shafi);
        let result = PrayerSchedule::new()
            .on(date)
            .for_location(location)
            .with_configuration(params)
            .calculate();

        assert!(result.is_ok());
    }
}
