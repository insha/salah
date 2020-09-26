// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

//! An Islamic prayer time implementation based on the [Adhan](https://github.com/batoulapps/Adhan) library by Batoul Apps.
//! While it has a lot of commnalities with the interface has
//! been changed slightly to make it more ergonomic and intuitive.
//!
//! ##### Example
//!
//! ```
//! use salah::prelude::*;
//!
//! let new_york_city = Coordinates::new(40.7128, -74.0059);
//! let date          = Utc.ymd(2019, 1, 25);
//! let params        = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
//! let prayers       = PrayerSchedule::new()
//!                       .on(date)
//!                       .for_location(new_york_city)
//!                       .with_configuration(params)
//!                       .calculate();
//! ```

mod astronomy;
mod models;
mod schedule;

pub use crate::astronomy::unit::{Coordinates, Stride};
pub use crate::models::adjustments::{Adjustment, TimeAdjustment};
pub use crate::models::madhab::Madhab;
pub use crate::models::method::Method;
pub use crate::models::parameters::{Configuration, Parameters};
pub use crate::models::prayer::Prayer;
pub use crate::schedule::{PrayerSchedule, PrayerTimes};
pub use chrono::{Date, DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};

/// A convenience module appropriate for glob imports (`use salah::prelude::*;`).
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::astronomy::unit::{Coordinates, Stride};
    #[doc(no_inline)]
    pub use crate::models::adjustments::{Adjustment, TimeAdjustment};
    #[doc(no_inline)]
    pub use crate::models::madhab::Madhab;
    #[doc(no_inline)]
    pub use crate::models::method::Method;
    #[doc(no_inline)]
    pub use crate::models::high_altitude_rule::HighLatitudeRule;
    #[doc(no_inline)]
    pub use crate::models::parameters::{Configuration, Parameters};
    #[doc(no_inline)]
    pub use crate::models::prayer::Prayer;
    #[doc(no_inline)]
    pub use crate::schedule::{PrayerSchedule, PrayerTimes};
    #[doc(no_inline)]
    pub use chrono::{Date, DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_prayer_times() {
        let local_date = Utc.ymd(2015, 7, 12);
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
        let date = Utc.ymd(2015, 7, 12);
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
        let date = Utc.ymd(2015, 7, 12);
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
        let date = Utc.ymd(2015, 7, 12);
        // let qiyam_date  = Utc.ymd(2015, 7, 13);
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
}
