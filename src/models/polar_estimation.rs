// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use chrono::{DateTime, Utc};

use crate::astronomy::solar::SolarTime;
use crate::astronomy::unit::{Coordinates, Stride};

/// Strategy for estimating prayer times when the sun never rises or sets
/// (polar day and polar night). Only the latitude is substituted; the
/// original longitude is always preserved.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PolarEstimation {
    /// Scan toward the equator along the same longitude until a latitude
    /// with a valid sunrise and sunset is found.
    NearestLatitude,
    /// Substitute a fixed latitude of 45° (north or south of the equator),
    /// keeping the original longitude. This mirrors the approach that the
    /// Muslim World League recommends for locations above 66° latitude.
    Reference45,
}

impl PolarEstimation {
    // Solar calculations for a single day of prayer times touch today,
    // tomorrow and the day after tomorrow, so all three must succeed
    // before the original coordinates are kept.
    pub fn resolve_coordinates(self, date: DateTime<Utc>, coordinates: Coordinates) -> Coordinates {
        let tomorrow = date.tomorrow();
        let day_after_tomorrow = tomorrow.tomorrow();

        if SolarTime::new(date, coordinates).is_ok()
            && SolarTime::new(tomorrow, coordinates).is_ok()
            && SolarTime::new(day_after_tomorrow, coordinates).is_ok()
        {
            coordinates
        } else {
            match self {
                PolarEstimation::NearestLatitude => nearest_working_latitude(date, coordinates),
                PolarEstimation::Reference45 => {
                    Coordinates::new(45.0 * coordinates.latitude.signum(), coordinates.longitude)
                }
            }
        }
    }
}

fn nearest_working_latitude(date: DateTime<Utc>, coordinates: Coordinates) -> Coordinates {
    let tomorrow = date.tomorrow();
    let day_after_tomorrow = tomorrow.tomorrow();
    let latitude_sign = coordinates.latitude.signum();
    let mut lower_bound = 0.0_f64;
    let mut upper_bound = coordinates.latitude.abs();

    for _ in 0..24 {
        let probe_latitude = (lower_bound + upper_bound) / 2.0;
        let probe_coordinates =
            Coordinates::new(probe_latitude * latitude_sign, coordinates.longitude);

        if SolarTime::new(date, probe_coordinates).is_ok()
            && SolarTime::new(tomorrow, probe_coordinates).is_ok()
            && SolarTime::new(day_after_tomorrow, probe_coordinates).is_ok()
        {
            lower_bound = probe_latitude;
        } else {
            upper_bound = probe_latitude;
        }
    }

    Coordinates::new(lower_bound * latitude_sign, coordinates.longitude)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        NaiveDate::from_ymd_opt(year, month, day)
            .expect("Invalid date provided")
            .and_hms_opt(0, 0, 0)
            .expect("Invalid time provided")
            .and_utc()
    }

    #[test]
    fn coordinates_are_kept_when_solar_times_exist() {
        let coordinates = Coordinates::new(51.5072, -0.1276);

        assert_eq!(
            PolarEstimation::NearestLatitude.resolve_coordinates(date(2026, 6, 21), coordinates),
            coordinates
        );
    }

    #[test]
    fn nearest_latitude_resolves_polar_day() {
        let coordinates = Coordinates::new(69.6492, 18.9553);
        let resolved =
            PolarEstimation::NearestLatitude.resolve_coordinates(date(2026, 6, 21), coordinates);

        assert!(resolved.latitude.abs() < coordinates.latitude);
        assert_eq!(resolved.longitude, coordinates.longitude);
        assert!(SolarTime::new(date(2026, 6, 21), resolved).is_ok());
        assert!(SolarTime::new(date(2026, 6, 22), resolved).is_ok());
        assert!(SolarTime::new(date(2026, 6, 23), resolved).is_ok());
    }

    #[test]
    fn reference_45_uses_fixed_latitude() {
        let coordinates = Coordinates::new(69.6492, 18.9553);
        let resolved =
            PolarEstimation::Reference45.resolve_coordinates(date(2026, 6, 21), coordinates);

        assert_eq!(resolved, Coordinates::new(45.0, 18.9553));
    }

    #[test]
    fn reference_45_keeps_southern_sign() {
        let coordinates = Coordinates::new(-69.0, 158.9);
        let resolved =
            PolarEstimation::Reference45.resolve_coordinates(date(2026, 6, 21), coordinates);

        assert_eq!(resolved, Coordinates::new(-45.0, 158.9));
    }
}
