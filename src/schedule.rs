// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

//! # Prayer Schedule
//!
//! This module provide the main objects that are used for calculating
//! the prayer times.

use chrono::{Date, DateTime, Datelike, Duration, DurationRound, Utc};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::astronomy::ops;
use crate::astronomy::solar::SolarTime;
use crate::astronomy::unit::{Angle, Coordinates};
use crate::models::method::Method;
use crate::models::parameters::Parameters;
use crate::models::prayer::Prayer;

/// A data struct to hold the timing for all
/// prayers.
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
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
    date: Date<Utc>,
    parameters: Parameters,
}

impl PrayerTimes {
    pub fn new(date: Date<Utc>, coordinates: Coordinates, parameters: Parameters) -> PrayerTimes {
        let tomorrow = date.succ();
        let solar_time = SolarTime::new(date, coordinates);
        let solar_time_tomorrow = SolarTime::new(tomorrow, coordinates);

        let asr = solar_time.afternoon(parameters.madhab.shadow());
        let night = solar_time_tomorrow.sunrise - solar_time.sunset;

        let fajr = PrayerTimes::calculate_fajr(parameters, solar_time, night, coordinates, date);
        let sunrise =
            solar_time.sunrise + Duration::minutes(parameters.time_adjustment(Prayer::Sunrise));
        let dhuhr =
            solar_time.transit + Duration::minutes(parameters.time_adjustment(Prayer::Dhuhr));
        let asr = asr + Duration::minutes(parameters.time_adjustment(Prayer::Asr));
        let maghrib =
            solar_time.sunset + Duration::minutes(parameters.time_adjustment(Prayer::Maghrib));
        let isha = PrayerTimes::calculate_isha(parameters, solar_time, night, coordinates, date);

        // Calculate the middle of the night and qiyam times
        let (middle_of_the_night, qiyam, fajr_tomorrow) = PrayerTimes::calculate_qiyam(
            maghrib,
            parameters,
            solar_time_tomorrow,
            coordinates,
            tomorrow,
        );

        PrayerTimes {
            fajr,
            sunrise,
            dhuhr,
            asr,
            maghrib,
            isha,
            middle_of_the_night,
            qiyam,
            fajr_tomorrow,
            coordinates,
            date,
            parameters,
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
            Prayer::QiyamYesterday => panic!("unsupported"),
        }
    }

    #[inline]
    pub fn current(&self) -> Prayer {
        self.prayer_at(Utc::now())
    }

    #[inline]
    pub fn next(&self) -> Prayer {
        self.current().next()
    }

    #[inline]
    pub fn time_remaining(&self) -> Duration {
        Duration::minutes((self.time(self.next()) - Utc::now()).num_minutes())
    }

    pub fn _prayer_at(&self, time: DateTime<Utc>) -> Option<Prayer> {
        if (self.fajr_tomorrow - time).num_seconds() <= 0 {
            Some(Prayer::FajrTomorrow)
        } else if (self.qiyam - time).num_seconds() <= 0 {
            Some(Prayer::Qiyam)
        } else if (self.isha - time).num_seconds() <= 0 {
            Some(Prayer::Isha)
        } else if (self.maghrib - time).num_seconds() <= 0 {
            Some(Prayer::Maghrib)
        } else if (self.asr - time).num_seconds() <= 0 {
            Some(Prayer::Asr)
        } else if (self.dhuhr - time).num_seconds() <= 0 {
            Some(Prayer::Dhuhr)
        } else if (self.sunrise - time).num_seconds() <= 0 {
            Some(Prayer::Sunrise)
        } else if (self.fajr - time).num_seconds() <= 0 {
            Some(Prayer::Fajr)
        } else {
            Some(Prayer::QiyamYesterday)
        }
    }

    /// All time before Fajr returns `QiyamYesterday`
    /// After `FajrTomorrow` it `panic`s
    pub fn prayer_at(&self, time: DateTime<Utc>) -> Prayer {
        if time < self.fajr {
            Prayer::QiyamYesterday
        } else if time < self.sunrise {
            Prayer::Fajr
        } else if time < self.dhuhr {
            Prayer::Sunrise
        } else if time < self.asr {
            Prayer::Dhuhr
        } else if time < self.maghrib {
            Prayer::Asr
        } else if time < self.isha {
            Prayer::Maghrib
        } else if time < self.qiyam {
            Prayer::Isha
        } else if time < self.fajr_tomorrow {
            Prayer::Qiyam
        } else {
            panic!("time out of range")
        }
    }

    fn calculate_fajr(
        parameters: Parameters,
        solar_time: SolarTime,
        night: Duration,
        coordinates: Coordinates,
        date: Date<Utc>,
    ) -> DateTime<Utc> {
        let mut fajr = solar_time.time_for_solar_angle(Angle::new(-parameters.fajr_angle), false);

        // special case for moonsighting committee above latitude 55
        if parameters.method == Method::MoonsightingCommittee && coordinates.latitude >= 55.0 {
            fajr = solar_time.sunrise - night / 7;
        }

        let safe_fajr = if parameters.method == Method::MoonsightingCommittee {
            let day_of_year = date.ordinal();
            ops::season_adjusted_morning_twilight(
                coordinates.latitude,
                day_of_year,
                date.year() as u32,
                solar_time.sunrise,
            )
        } else {
            let portion = parameters.night_portions().0;
            let night_fraction = portion * (night.num_seconds() as f64);

            solar_time.sunrise + Duration::seconds(-night_fraction as i64)
        };

        if fajr < safe_fajr {
            fajr = safe_fajr;
        }

        fajr + Duration::minutes(parameters.time_adjustment(Prayer::Fajr))
    }

    fn calculate_isha(
        parameters: Parameters,
        solar_time: SolarTime,
        night: Duration,
        coordinates: Coordinates,
        date: Date<Utc>,
    ) -> DateTime<Utc> {
        let mut isha: DateTime<Utc>;

        if parameters.isha_interval > 0 {
            isha = solar_time.sunset + Duration::minutes(parameters.isha_interval.into());
        } else {
            isha = solar_time.time_for_solar_angle(Angle::new(-parameters.isha_angle), true);

            // special case for moonsighting committee above latitude 55
            if parameters.method == Method::MoonsightingCommittee && coordinates.latitude >= 55.0 {
                isha = solar_time.sunset + night / 7;
            }

            let safe_isha = if parameters.method == Method::MoonsightingCommittee {
                let day_of_year = date.ordinal();

                ops::season_adjusted_evening_twilight(
                    coordinates.latitude,
                    day_of_year,
                    date.year() as u32,
                    solar_time.sunset,
                )
            } else {
                let portion = parameters.night_portions().1;
                let night_fraction = portion * (night.num_seconds() as f64);

                solar_time.sunset + Duration::seconds(night_fraction as i64)
            };

            if isha > safe_isha {
                isha = safe_isha;
            }
        }

        isha + Duration::minutes(parameters.time_adjustment(Prayer::Isha))
    }

    fn calculate_qiyam(
        current_maghrib: DateTime<Utc>,
        parameters: Parameters,
        solar_time: SolarTime,
        coordinates: Coordinates,
        date: Date<Utc>,
    ) -> (DateTime<Utc>, DateTime<Utc>, DateTime<Utc>) {
        let tomorrow = date.succ();
        let solar_time_tomorrow = SolarTime::new(tomorrow, coordinates);
        let night = solar_time_tomorrow.sunrise - solar_time.sunset;

        let tomorrow_fajr =
            PrayerTimes::calculate_fajr(parameters, solar_time, night, coordinates, date);
        let night_duration = (tomorrow_fajr - current_maghrib).num_seconds() as f64;
        let middle_night_portion = (night_duration / 2.0) as i64;
        let last_third_portion = (night_duration * (2.0 / 3.0)) as i64;
        let middle_of_night = (current_maghrib + Duration::seconds(middle_night_portion))
            .duration_round(Duration::minutes(1))
            .unwrap();
        let last_third_of_night = (current_maghrib + Duration::seconds(last_third_portion))
            .duration_round(Duration::minutes(1))
            .unwrap();

        (middle_of_night, last_third_of_night, tomorrow_fajr)
    }
}

/// A builder for the [PrayerTimes](struct.PrayerTimes.html) struct.
#[derive(Default)]
pub struct PrayerSchedule {
    date: Option<Date<Utc>>,
    coordinates: Option<Coordinates>,
    params: Option<Parameters>,
}

impl PrayerSchedule {
    pub fn on(&mut self, date: Date<Utc>) -> &mut PrayerSchedule {
        self.date = Some(date);
        self
    }

    pub fn for_location(&mut self, location: Coordinates) -> &mut PrayerSchedule {
        self.coordinates = Some(location);
        self
    }

    pub fn with_configuration(&mut self, params: Parameters) -> &mut PrayerSchedule {
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
    use chrono::TimeZone;

    use super::*;
    use crate::models::madhab::Madhab;

    #[test]
    fn current_prayer_should_be_fajr() {
        // Given the above DateTime, the Fajr prayer is at 2015-07-12T08:42:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(9, 0, 0);

        assert_eq!(times.prayer_at(current_prayer_time), Prayer::Fajr);
    }

    #[test]
    fn current_prayer_should_be_sunrise() {
        // Given the below DateTime, sunrise is at 2015-07-12T10:08:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(11, 0, 0);

        assert_eq!(times.prayer_at(current_prayer_time), Prayer::Sunrise);
    }

    #[test]
    fn current_prayer_should_be_dhuhr() {
        // Given the above DateTime, dhuhr prayer is at 2015-07-12T17:21:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(19, 0, 0);

        assert_eq!(times.prayer_at(current_prayer_time), Prayer::Dhuhr);
    }

    #[test]
    fn current_prayer_should_be_asr() {
        // Given the below DateTime, asr is at 2015-07-12T22:22:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(22, 26, 0);

        assert_eq!(times.prayer_at(current_prayer_time), Prayer::Asr);
    }

    #[test]
    fn current_prayer_should_be_maghrib() {
        // Given the below DateTime, maghrib is at 2015-07-13T00:32:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = Utc.ymd(2015, 7, 13).and_hms(1, 0, 0);

        assert_eq!(times.prayer_at(current_prayer_time), Prayer::Maghrib);
    }

    #[test]
    fn current_prayer_should_be_isha() {
        // Given the below DateTime, isha is at 2015-07-13T01:57:00Z
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = Utc.ymd(2015, 7, 13).and_hms(2, 0, 0);

        assert_eq!(times.prayer_at(current_prayer_time), Prayer::Isha);
    }

    #[test]
    fn current_prayer_should_be_qiyam_yesterday() {
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).and_hms(8, 0, 0);

        assert_eq!(times.prayer_at(current_prayer_time), Prayer::QiyamYesterday);
    }

    #[test]
    fn current_prayer_should_be_qiyam() {
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).succ().and_hms(6, 0, 0);

        assert_eq!(times.prayer_at(current_prayer_time), Prayer::Qiyam);
    }

    #[test]
    #[should_panic]
    fn current_prayer_should_panic() {
        let local_date = Utc.ymd(2015, 7, 12);
        let params = Parameters::with(Method::NorthAmerica, Madhab::Hanafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let times = PrayerTimes::new(local_date, coordinates, params);
        let current_prayer_time = local_date.with_timezone(&Utc).succ().and_hms(9, 0, 0);

        println!("{:?}", times.prayer_at(current_prayer_time));
    }

    #[test]
    fn calculate_times_for_moonsighting_method() {
        let date = Utc.ymd(2016, 1, 31);
        let params = Parameters::with(Method::MoonsightingCommittee, Madhab::Shafi);
        let coordinates = Coordinates::new(35.7750, -78.6336);
        let result = PrayerSchedule::default()
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

            Err(_err) => panic!(),
        }
    }

    #[test]
    fn calculate_times_for_moonsighting_method_with_high_latitude() {
        let date = Utc.ymd(2016, 1, 1);
        let params = Parameters::with(Method::MoonsightingCommittee, Madhab::Hanafi);
        let coordinates = Coordinates::new(59.9094, 10.7349);
        let result = PrayerSchedule::default()
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

            Err(_err) => panic!(),
        }
    }
}
