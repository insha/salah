// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2021 Farhan Ahmed. All rights reserved.
//

//! # Prayer Schedule
//!
//! This module provide the main objects that are used for calculating
//! the prayer times.

use chrono::{Date, DateTime, Datelike, Duration, Utc};

use crate::astronomy::ops;
use crate::astronomy::solar::SolarTime;
use crate::astronomy::unit::{Angle, Coordinates, Stride};
use crate::models::method::Method;
use crate::models::parameters::Parameters;
use crate::models::prayer::Prayer;

/// A data struct to hold the timing for all
/// prayers.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PrayerTimes {
    fajr: DateTime<Utc>,
    sunrise: DateTime<Utc>,
    dhuhr: DateTime<Utc>,
    asr: DateTime<Utc>,
    maghrib: DateTime<Utc>,
    isha: DateTime<Utc>,
    middle_of_the_night: DateTime<Utc>,
    qiyam: DateTime<Utc>,
    fajr_tomorrow: DateTime<Utc>,
    coordinates: Coordinates,
    date: DateTime<Utc>,
    parameters: Parameters,
}

impl PrayerTimes {
    pub fn new(date: Date<Utc>, coordinates: Coordinates, parameters: Parameters) -> PrayerTimes {
        let prayer_date = date.and_hms(0, 0, 0);
        let tomorrow = prayer_date.tomorrow();
        let solar_time = SolarTime::new(prayer_date, coordinates);
        let solar_time_tomorrow = SolarTime::new(tomorrow, coordinates);

        let asr = solar_time.afternoon(parameters.madhab.shadow().into());
        let night = solar_time_tomorrow
            .sunrise
            .signed_duration_since(solar_time.sunset);

        let final_fajr =
            PrayerTimes::calculate_fajr(parameters, solar_time, night, coordinates, prayer_date);
        let final_sunrise = solar_time
            .sunrise
            .adjust_time(parameters.time_adjustments(Prayer::Sunrise));
        let final_dhuhr = solar_time
            .transit
            .adjust_time(parameters.time_adjustments(Prayer::Dhuhr));
        let final_asr = asr.adjust_time(parameters.time_adjustments(Prayer::Asr));
        let final_maghrib = ops::adjust_time(
            &solar_time.sunset,
            parameters.time_adjustments(Prayer::Maghrib),
        );
        let final_isha =
            PrayerTimes::calculate_isha(parameters, solar_time, night, coordinates, prayer_date);

        // Calculate the middle of the night and qiyam times
        let (final_middle_of_night, final_qiyam, final_fajr_tomorrow) =
            PrayerTimes::calculate_qiyam(
                final_maghrib,
                parameters,
                solar_time_tomorrow,
                coordinates,
                tomorrow,
            );

        PrayerTimes {
            fajr: final_fajr,
            sunrise: final_sunrise,
            dhuhr: final_dhuhr,
            asr: final_asr,
            maghrib: final_maghrib,
            isha: final_isha,
            middle_of_the_night: final_middle_of_night,
            qiyam: final_qiyam,
            fajr_tomorrow: final_fajr_tomorrow,
            coordinates: coordinates,
            date: prayer_date,
            parameters: parameters,
        }
    }

    pub fn time(&self, prayer: Prayer) -> DateTime<Utc> {
        match prayer {
            Prayer::Fajr => self.fajr,
            Prayer::Sunrise => self.sunrise,
            Prayer::Dhuhr => self.dhuhr,
            Prayer::Asr => self.asr,
            Prayer::Maghrib => self.maghrib,
            Prayer::Isha => self.isha,
            Prayer::Qiyam => self.qiyam,
            Prayer::FajrTomorrow => self.fajr_tomorrow,
        }
    }

    pub fn current(&self) -> Prayer {
        self.current_time(Utc::now()).expect("Out of bounds")
    }

    pub fn next(&self) -> Prayer {
        match self.current() {
            Prayer::Fajr => Prayer::Sunrise,
            Prayer::Sunrise => Prayer::Dhuhr,
            Prayer::Dhuhr => Prayer::Asr,
            Prayer::Asr => Prayer::Maghrib,
            Prayer::Maghrib => Prayer::Isha,
            Prayer::Isha => Prayer::Qiyam,
            Prayer::Qiyam => Prayer::FajrTomorrow,
            _ => Prayer::FajrTomorrow,
        }
    }

    pub fn time_remaining(&self) -> (u32, u32) {
        let next_time = self.time(self.next());
        let now = Utc::now();
        let now_to_next = next_time.signed_duration_since(now).num_seconds() as f64;
        let whole: f64 = now_to_next / 60.0 / 60.0;
        let fract = whole.fract();
        let hours = whole.trunc() as u32;
        let minutes = (fract * 60.0).round() as u32;

        (hours, minutes)
    }

    fn current_time(&self, time: DateTime<Utc>) -> Option<Prayer> {
        let current_prayer: Option<Prayer>;

        if self.fajr_tomorrow.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::FajrTomorrow)
        } else if self.qiyam.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Qiyam)
        } else if self.isha.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Isha);
        } else if self.maghrib.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Maghrib);
        } else if self.asr.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Asr);
        } else if self.dhuhr.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Dhuhr);
        } else if self.sunrise.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Sunrise);
        } else if self.fajr.signed_duration_since(time).num_seconds() <= 0 {
            current_prayer = Some(Prayer::Fajr);
        } else {
            current_prayer = None;
        }

        current_prayer
    }

    fn calculate_fajr(
        parameters: Parameters,
        solar_time: SolarTime,
        night: Duration,
        coordinates: Coordinates,
        prayer_date: DateTime<Utc>,
    ) -> DateTime<Utc> {
        let mut fajr = solar_time.time_for_solar_angle(Angle::new(-parameters.fajr_angle), false);

        // special case for moonsighting committee above latitude 55
        if parameters.method == Method::MoonsightingCommittee && coordinates.latitude >= 55.0 {
            let night_fraction = night.num_seconds() / 7;
            fajr = solar_time
                .sunrise
                .checked_add_signed(Duration::seconds(-night_fraction))
                .unwrap();
        } else {
            // Nothing to do.
        }

        let safe_fajr = if parameters.method == Method::MoonsightingCommittee {
            let day_of_year = prayer_date.ordinal();
            ops::season_adjusted_morning_twilight(
                coordinates.latitude,
                day_of_year,
                prayer_date.year() as u32,
                solar_time.sunrise,
            )
        } else {
            let portion = parameters.night_portions().0;
            let night_fraction = portion * (night.num_seconds() as f64);

            solar_time
                .sunrise
                .checked_add_signed(Duration::seconds(-night_fraction as i64))
                .unwrap()
        };

        if fajr < safe_fajr {
            fajr = safe_fajr;
        } else {
            // Nothing to do.
        }

        fajr.adjust_time(parameters.time_adjustments(Prayer::Fajr))
    }

    fn calculate_isha(
        parameters: Parameters,
        solar_time: SolarTime,
        night: Duration,
        coordinates: Coordinates,
        prayer_date: DateTime<Utc>,
    ) -> DateTime<Utc> {
        let mut isha: DateTime<Utc>;

        if parameters.isha_interval > 0 {
            isha = solar_time
                .sunset
                .checked_add_signed(Duration::seconds((parameters.isha_interval * 60) as i64))
                .unwrap();
        } else {
            isha = solar_time.time_for_solar_angle(Angle::new(-parameters.isha_angle), true);

            // special case for moonsighting committee above latitude 55
            if parameters.method == Method::MoonsightingCommittee && coordinates.latitude >= 55.0 {
                let night_fraction = night.num_seconds() / 7;
                isha = solar_time
                    .sunset
                    .checked_add_signed(Duration::seconds(night_fraction))
                    .unwrap();
            } else {
                // Nothing to do.
            }

            let safe_isha = if parameters.method == Method::MoonsightingCommittee {
                let day_of_year = prayer_date.ordinal();

                ops::season_adjusted_evening_twilight(
                    coordinates.latitude,
                    day_of_year,
                    prayer_date.year() as u32,
                    solar_time.sunset,
                )
            } else {
                let portion = parameters.night_portions().1;
                let night_fraction = portion * (night.num_seconds() as f64);

                solar_time
                    .sunset
                    .checked_add_signed(Duration::seconds(night_fraction as i64))
                    .unwrap()
            };

            if isha > safe_isha {
                isha = safe_isha;
            } else {
                // Nothing to do.
            }
        }

        isha.adjust_time(parameters.time_adjustments(Prayer::Isha))
    }

    fn calculate_qiyam(
        current_maghrib: DateTime<Utc>,
        parameters: Parameters,
        solar_time: SolarTime,
        coordinates: Coordinates,
        prayer_date: DateTime<Utc>,
    ) -> (DateTime<Utc>, DateTime<Utc>, DateTime<Utc>) {
        let tomorrow = prayer_date.tomorrow();
        let solar_time_tomorrow = SolarTime::new(tomorrow, coordinates);
        let night = solar_time_tomorrow
            .sunrise
            .signed_duration_since(solar_time.sunset);

        let tomorrow_fajr =
            PrayerTimes::calculate_fajr(parameters, solar_time, night, coordinates, prayer_date);
        let night_duration = tomorrow_fajr
            .signed_duration_since(current_maghrib)
            .num_seconds() as f64;
        let middle_night_portion = (night_duration / 2.0) as i64;
        let last_third_portion = (night_duration * (2.0 / 3.0)) as i64;
        let middle_of_night = current_maghrib
            .checked_add_signed(Duration::seconds(middle_night_portion))
            .unwrap()
            .nearest_minute();
        let last_third_of_night = current_maghrib
            .checked_add_signed(Duration::seconds(last_third_portion))
            .unwrap()
            .nearest_minute();

        (middle_of_night, last_third_of_night, tomorrow_fajr)
    }
}

/// A builder for the [PrayerTimes](struct.PrayerTimes.html) struct.
pub struct PrayerSchedule {
    date: Option<Date<Utc>>,
    coordinates: Option<Coordinates>,
    params: Option<Parameters>,
}

impl PrayerSchedule {
    pub fn new() -> PrayerSchedule {
        PrayerSchedule {
            date: None,
            coordinates: None,
            params: None,
        }
    }

    pub fn on<'a>(&'a mut self, date: Date<Utc>) -> &'a mut PrayerSchedule {
        self.date = Some(date);
        self
    }

    pub fn for_location<'a>(&'a mut self, location: Coordinates) -> &'a mut PrayerSchedule {
        self.coordinates = Some(location);
        self
    }

    pub fn with_configuration<'a>(&'a mut self, params: Parameters) -> &'a mut PrayerSchedule {
        self.params = Some(params);
        self
    }

    pub fn calculate(&self) -> Result<PrayerTimes, String> {
        if self.date.is_some() && self.coordinates.is_some() && self.params.is_some() {
            Ok(PrayerTimes::new(
                self.date.unwrap(),
                self.coordinates.unwrap(),
                self.params.unwrap(),
            ))
        } else {
            Err(String::from(
                "Required information is needed in order to calculate the prayer times.",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::madhab::Madhab;
    use crate::Configuration;
    use chrono::{TimeZone, Utc};

    #[test]
    fn current_prayer_should_be_fajr() {
        // Given the above DateTime, the Fajr prayer is at 2015-07-12T08:42:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(9, 0, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Fajr));
    }

    #[test]
    fn current_prayer_should_be_sunrise() {
        // Given the below DateTime, sunrise is at 2015-07-12T10:08:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(11, 0, 0);

        assert_eq!(
            times.current_time(current_prayer_time),
            Some(Prayer::Sunrise)
        );
    }

    #[test]
    fn current_prayer_should_be_dhuhr() {
        // Given the above DateTime, dhuhr prayer is at 2015-07-12T17:21:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(19, 0, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Dhuhr));
    }

    #[test]
    fn current_prayer_should_be_asr() {
        // Given the below DateTime, asr is at 2015-07-12T22:22:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(22, 26, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Asr));
    }

    #[test]
    fn current_prayer_should_be_maghrib() {
        // Given the below DateTime, maghrib is at 2015-07-13T00:32:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = Utc.ymd(2015, 7, 13).and_hms(01, 0, 0);

        assert_eq!(
            times.current_time(current_prayer_time),
            Some(Prayer::Maghrib)
        );
    }

    #[test]
    fn current_prayer_should_be_isha() {
        // Given the below DateTime, isha is at 2015-07-13T01:57:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = Utc.ymd(2015, 7, 13).and_hms(02, 0, 0);

        assert_eq!(times.current_time(current_prayer_time), Some(Prayer::Isha));
    }

    #[test]
    fn current_prayer_should_be_none() {
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(8, 0, 0);

        assert_eq!(times.current_time(current_prayer_time), None);
    }

    #[test]
    fn calculate_times_for_moonsighting_method() {
        let date = Utc.ymd(2016, 1, 31);
        let params = Configuration::with(Method::MoonsightingCommittee, Madhab::Shafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let result = PrayerSchedule::new()
            .on(date)
            .for_location(coordinates)
            .with_configuration(params)
            .calculate();

        match result {
            Ok(schedule) => {
                // fajr    = 2016-01-31 10:48:00 UTC
                // sunrise = 2016-01-31 12:16:00 UTC
                // dhuhr   = 2016-01-31 17:33:00 UTC
                // asr     = 2016-01-31 20:20:00 UTC
                // maghrib = 2016-01-31 22:43:00 UTC
                // isha    = 2016-02-01 00:05:00 UTC
                assert_eq!(
                    schedule.time(Prayer::Fajr).format("%-l:%M %p").to_string(),
                    "10:48 AM"
                );
                assert_eq!(
                    schedule
                        .time(Prayer::Sunrise)
                        .format("%-l:%M %p")
                        .to_string(),
                    "12:16 PM"
                );
                assert_eq!(
                    schedule.time(Prayer::Dhuhr).format("%-l:%M %p").to_string(),
                    "5:33 PM"
                );
                assert_eq!(
                    schedule.time(Prayer::Asr).format("%-l:%M %p").to_string(),
                    "8:20 PM"
                );
                assert_eq!(
                    schedule
                        .time(Prayer::Maghrib)
                        .format("%-l:%M %p")
                        .to_string(),
                    "10:43 PM"
                );
                assert_eq!(
                    schedule.time(Prayer::Isha).format("%-l:%M %p").to_string(),
                    "12:05 AM"
                );
            }

            Err(_err) => assert!(false),
        }
    }

    #[test]
    fn calculate_times_for_moonsighting_method_with_high_latitude() {
        let date = Utc.ymd(2016, 1, 1);
        let params = Configuration::with(Method::MoonsightingCommittee, Madhab::Hanafi);
        let coordinates = Coordinates::new(59.9094, 10.7349);
        let result = PrayerSchedule::new()
            .on(date)
            .for_location(coordinates)
            .with_configuration(params)
            .calculate();

        match result {
            Ok(schedule) => {
                // fajr    = 2016-01-01 06:34:00 UTC
                // sunrise = 2016-01-01 08:19:00 UTC
                // dhuhr   = 2016-01-01 11:25:00 UTC
                // asr     = 2016-01-01 12:36:00 UTC
                // maghrib = 2016-01-01 14:25:00 UTC
                // isha    = 2016-01-01 16:02:00 UTC
                assert_eq!(
                    schedule.time(Prayer::Fajr).format("%-l:%M %p").to_string(),
                    "6:34 AM"
                );
                assert_eq!(
                    schedule
                        .time(Prayer::Sunrise)
                        .format("%-l:%M %p")
                        .to_string(),
                    "8:19 AM"
                );
                assert_eq!(
                    schedule.time(Prayer::Dhuhr).format("%-l:%M %p").to_string(),
                    "11:25 AM"
                );
                assert_eq!(
                    schedule.time(Prayer::Asr).format("%-l:%M %p").to_string(),
                    "12:36 PM"
                );
                assert_eq!(
                    schedule
                        .time(Prayer::Maghrib)
                        .format("%-l:%M %p")
                        .to_string(),
                    "2:25 PM"
                );
                assert_eq!(
                    schedule.time(Prayer::Isha).format("%-l:%M %p").to_string(),
                    "4:02 PM"
                );
            }

            Err(_err) => assert!(false),
        }
    }
}
