// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

use chrono::{Date, DateTime, Datelike, Duration, DurationRound, Local};

use crate::astronomy::unit::Normalize;
use crate::astronomy::unit::{Angle, Coordinates};

// The geometric mean longitude of the sun.
pub fn mean_solar_longitude(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 163
    let term1 = 280.4664567;
    let term2 = 36000.76983 * julian_century;
    let term3 = 0.0003032 * julian_century.powf(2.0);
    let degrees = term1 + term2 + term3;

    Angle::new(degrees).unwound()
}

// The geometric mean longitude of the moon.
pub fn mean_lunar_longitude(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 144
    let term1 = 218.3165;
    let term2 = 481267.8813 * julian_century;
    let degrees = term1 + term2;

    Angle::new(degrees).unwound()
}

pub fn ascending_lunar_node_longitude(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 144
    let term1 = 125.04452;
    let term2 = 1934.136261 * julian_century;
    let term3 = 0.0020708 * julian_century.powf(2.0);
    let term4 = julian_century.powf(3.0) / 450000.0;
    let degrees = term1 - term2 + term3 + term4;

    Angle::new(degrees).unwound()
}

// The mean anomaly of the sun.
pub fn mean_solar_anomaly(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 163
    let term1 = 357.52911;
    let term2 = 35999.05029 * julian_century;
    let term3 = 0.0001537 * julian_century.powf(2.0);
    let degrees = term1 + term2 - term3;

    Angle::new(degrees).unwound()
}

// The Sun's equation of the center.
pub fn solar_equation_of_the_center(julian_century: f64, mean_anomaly: Angle) -> Angle {
    // Equation from Astronomical Algorithms page 164
    let mean_radians = mean_anomaly.radians();
    let term1 = (1.914602 - (0.004817 * julian_century) - (0.000014 * julian_century.powf(2.0)))
        * mean_radians.sin();
    let term2 = (0.019993 - (0.000101 * julian_century)) * (2.0 * mean_radians).sin();
    let term3 = 0.000289 * (3.0 * mean_radians).sin();

    Angle::new(term1 + term2 + term3)
}

// The apparent longitude of the Sun, referred to the
// true equinox of the date.
pub fn apparent_solar_longitude(julian_century: f64, mean_longitude: Angle) -> Angle {
    // Equation from Astronomical Algorithms page 164
    let longitude = mean_longitude
        + solar_equation_of_the_center(julian_century, mean_solar_anomaly(julian_century));
    let omega = Angle::new(125.04 - (1934.136 * julian_century));
    let lambda = Angle::new(longitude.degrees - 0.00569 - (0.00478 * omega.radians().sin()));

    lambda.unwound()
}

// The mean obliquity of the ecliptic, formula
// adopted by the International Astronomical Union.
pub fn mean_obliquity_of_the_ecliptic(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 147
    let term1 = 23.439291;
    let term2 = 0.013004167 * julian_century;
    let term3 = 0.0000001639 * julian_century.powf(2.0);
    let term4 = 0.0000005036 * julian_century.powf(3.0);

    Angle::new(term1 - term2 - term3 + term4)
}

// The mean obliquity of the ecliptic, corrected for
// calculating the apparent position of the sun.
pub fn apparent_obliquity_of_the_ecliptic(
    julian_century: f64,
    mean_obliquity_of_the_ecliptic: Angle,
) -> Angle {
    // Equation from Astronomical Algorithms page 165
    let degrees: f64 = 125.04 - (1934.136 * julian_century);

    Angle::new(
        mean_obliquity_of_the_ecliptic.degrees + (0.00256 * Angle::new(degrees).radians().cos()),
    )
}

// Mean sidereal time, the hour angle of the vernal equinox.
pub fn mean_sidereal_time(julian_century: f64) -> Angle {
    // Equation from Astronomical Algorithms page 165
    let julian_day = (julian_century * 36525.0) + 2451545.0;
    let term1 = 280.46061837;
    let term2 = 360.98564736629 * (julian_day - 2451545.0);
    let term3 = 0.000387933 * julian_century.powf(2.0);
    let term4 = julian_century.powf(3.0) / 38710000.0;
    let degrees = term1 + term2 + term3 - term4;

    Angle::new(degrees).unwound()
}

pub fn nutation_in_longitude(
    solar_longitude: Angle,
    lunar_longitude: Angle,
    ascending_node: Angle,
) -> f64 {
    // Equation from Astronomical Algorithms page 144
    let term1 = (-17.2 / 3600.0) * ascending_node.radians().sin();
    let term2 = (1.32 / 3600.0) * (2.0 * solar_longitude.radians()).sin();
    let term3 = (0.23 / 3600.0) * (2.0 * lunar_longitude.radians()).sin();
    let term4 = (0.21 / 3600.0) * (2.0 * ascending_node.radians()).sin();

    term1 - term2 - term3 + term4
}

pub fn nutation_in_obliquity(
    solar_longitude: Angle,
    lunar_longitude: Angle,
    ascending_node: Angle,
) -> f64 {
    // Equation from Astronomical Algorithms page 144
    let term1 = (9.2 / 3600.0) * ascending_node.radians().cos();
    let term2 = (0.57 / 3600.0) * (2.0 * solar_longitude.radians()).cos();
    let term3 = (0.10 / 3600.0) * (2.0 * lunar_longitude.radians()).cos();
    let term4 = (0.09 / 3600.0) * (2.0 * ascending_node.radians()).cos();

    term1 + term2 + term3 - term4
}

pub fn altitude_of_celestial_body(
    observer_latitude: Angle,
    declination: Angle,
    local_hour_angle: Angle,
) -> Angle {
    // Equation from Astronomical Algorithms page 93
    let term1 = observer_latitude.radians().sin() * declination.radians().sin();
    let term2 = observer_latitude.radians().cos()
        * declination.radians().cos()
        * local_hour_angle.radians().cos();

    Angle::from_radians((term1 + term2).asin())
}

pub fn approximate_transit(longitude: Angle, sidereal_time: Angle, right_ascension: Angle) -> f64 {
    // Equation from page Astronomical Algorithms 102
    let longitude_angle = longitude * Angle::new(-1.0);

    ((right_ascension + longitude_angle - sidereal_time) / Angle::new(360.0))
        .degrees
        .normalized_to_scale(1.0)
}

// The time at which the sun is at its highest point in the sky (in universal time)
pub fn corrected_transit(
    approximate_transit: f64,
    longitude: Angle,
    sidereal_time: Angle,
    right_ascension: Angle,
    previous_right_ascension: Angle,
    next_right_ascension: Angle,
) -> f64 {
    // Equation from page Astronomical Algorithms 102
    let longitude_angle = longitude * Angle::new(-1.0);
    let plane_angle =
        Angle::new(sidereal_time.degrees + (360.985647 * approximate_transit)).unwound();
    let interpolated_angles = interpolate_angles(
        right_ascension,
        previous_right_ascension,
        next_right_ascension,
        approximate_transit,
    )
    .unwound();
    let angles = (plane_angle - longitude_angle - interpolated_angles).quadrant_shifted();
    let angle_delta = angles / Angle::new(-360.0);

    (approximate_transit + angle_delta.degrees) * 24.0
}

pub fn corrected_hour_angle(
    approximate_transit: f64,
    angle: Angle,
    coordinates: Coordinates,
    after_transit: bool,
    sidereal_time: Angle,
    right_ascension: Angle,
    previous_right_ascension: Angle,
    next_right_ascension: Angle,
    declination: Angle,
    previous_declination: Angle,
    next_declination: Angle,
) -> f64 {
    // Equation from page Astronomical Algorithms 102
    let longitude_angle = coordinates.longitude_angle() * Angle::new(-1.0);
    let term1 = angle.radians().sin()
        - (coordinates.latitude_angle().radians().sin() * declination.radians().sin());
    let term2 = coordinates.latitude_angle().radians().cos() * declination.radians().cos();
    let term_angle = Angle::from_radians((term1 / term2).acos());

    let adjusted_approx_transit = if after_transit {
        approximate_transit + (term_angle.degrees / 360.0)
    } else {
        approximate_transit - (term_angle.degrees / 360.0)
    };

    let plane_angle =
        Angle::new(sidereal_time.degrees + (360.985647 * adjusted_approx_transit)).unwound();
    let interpolated_angles = interpolate_angles(
        right_ascension,
        previous_right_ascension,
        next_right_ascension,
        adjusted_approx_transit,
    )
    .unwound();
    let declination_angle = Angle::new(interpolate(
        declination.degrees,
        previous_declination.degrees,
        next_declination.degrees,
        adjusted_approx_transit,
    ));
    let adjusted_angles = plane_angle - longitude_angle - interpolated_angles;
    let celestial_body_altitude = altitude_of_celestial_body(
        coordinates.latitude_angle(),
        declination_angle,
        adjusted_angles,
    );
    let term3 = (celestial_body_altitude - angle).degrees;
    let term4 = 360.0
        * declination_angle.radians().cos()
        * coordinates.latitude_angle().radians().cos()
        * adjusted_angles.radians().sin();
    let angle_delta = term3 / term4;

    (adjusted_approx_transit + angle_delta) * 24.0
}

// Interpolation of a value given equidistant previous and
// next values and a factor equal to the fraction of the interpolated
// point's time over the time between values.
pub fn interpolate(value: f64, previous_value: f64, next_value: f64, factor: f64) -> f64 {
    // Equation from Astronomical Algorithms page 24
    let a = value - previous_value;
    let b = next_value - value;
    let c = b - a;

    value + ((factor / 2.0) * (a + b + (factor * c)))
}

// Interpolation of three angles, accounting for angle unwinding.
pub fn interpolate_angles(
    value: Angle,
    previous_value: Angle,
    next_value: Angle,
    factor: f64,
) -> Angle {
    // Equation from Astronomical Algorithms page 24
    let a = (value - previous_value).unwound();
    let b = (next_value - value).unwound();
    let c = b - a;

    Angle::new(value.degrees + ((factor / 2.0) * (a.degrees + b.degrees + (factor * c.degrees))))
}

// The Julian Day for the given Gregorian date.
pub fn julian_day_ymdh(year: i32, month: i32, day: i32, hours: f64) -> f64 {
    // Equation from Astronomical Algorithms page 60

    // NOTE: Casting to i32 is done intentionally for the purpose of decimal truncation

    let adjusted_year: i32 = if month > 2 { year } else { year - 1 };
    let adjusted_month: i32 = if month > 2 { month } else { month + 12 };
    let adjusted_day: f64 = (day as f64) + (hours / 24.0);

    let a: i32 = adjusted_year / 100;
    let b: i32 = 2 - a + (a / 4);

    let i0: i32 = (365.25 * ((adjusted_year as f64) + 4716.0)) as i32;
    let i1: i32 = (30.6001 * ((adjusted_month as f64) + 1.0)) as i32;

    (i0 as f64) + (i1 as f64) + adjusted_day + (b as f64) - 1524.5
}

// The Julian Day for the given Gregorian date.
pub fn julian_day<Tz: chrono::TimeZone>(date: Date<Tz>) -> f64 {
    julian_day_ymdh(
        date.year() as i32,
        date.month() as i32,
        date.day() as i32,
        0.0,
    )
}

// Julian century from the epoch.
pub fn julian_century(julian_day: f64) -> f64 {
    // Equation from Astronomical Algorithms page 163
    (julian_day - 2451545.0) / 36525.0
}

// Checks if the given year is a leap year.
pub fn is_leap_year(year: u32) -> bool {
    if year % 4 != 0 {
        return false;
    }

    if year % 100 == 0 && year % 400 != 0 {
        return false;
    }

    true
}

fn adjust(a: f64, b: f64, c: f64, d: f64, dyy: f64) -> f64 {
    if (0.0..=90.0).contains(&dyy) {
        a + (b - a) / 91.0 * dyy
    } else if (91.0..=136.0).contains(&dyy) {
        b + (c - b) / 46.0 * (dyy - 91.0)
    } else if (137.0..=182.0).contains(&dyy) {
        c + (d - c) / 46.0 * (dyy - 137.0)
    } else if (183.0..=228.0).contains(&dyy) {
        d + (c - d) / 46.0 * (dyy - 183.0)
    } else if (229.0..=274.0).contains(&dyy) {
        c + (b - c) / 46.0 * (dyy - 229.0)
    } else {
        b + (a - b) / 91.0 * (dyy - 275.0)
    }
}

// Twilight adjustment based on observational data for use
// in the Moonsighting Committee calculation method.
pub fn season_adjusted_morning_twilight(
    latitude: f64,
    day: u32,
    year: u32,
    sunrise: DateTime<Local>,
) -> DateTime<Local> {
    let a = 75.0 + ((28.65 / 55.0) * latitude.abs());
    let b = 75.0 + ((19.44 / 55.0) * latitude.abs());
    let c = 75.0 + ((32.74 / 55.0) * latitude.abs());
    let d = 75.0 + ((48.10 / 55.0) * latitude.abs());

    let dyy = days_since_solstice(day, year, latitude) as f64;
    let adjustment = adjust(a, b, c, d, dyy);

    let rounded_adjustment = (adjustment * -60.0).round() as i64;
    sunrise + Duration::seconds(rounded_adjustment)
}

// Twilight adjustment based on observational data for use
// in the Moonsighting Committee calculation method.
pub fn season_adjusted_evening_twilight(
    latitude: f64,
    day: u32,
    year: u32,
    sunset: DateTime<Local>,
) -> DateTime<Local> {
    let a = 75.0 + ((25.60 / 55.0) * latitude.abs());
    let b = 75.0 + ((2.050 / 55.0) * latitude.abs());
    let c = 75.0 - ((9.210 / 55.0) * latitude.abs());
    let d = 75.0 + ((6.140 / 55.0) * latitude.abs());

    let dyy = days_since_solstice(day, year, latitude) as f64;
    let adjustment = adjust(a, b, c, d, dyy);

    let rounded_adjustment = (adjustment * 60.0).round() as i64;
    let adjusted_date = sunset + Duration::seconds(rounded_adjustment);

    adjusted_date.duration_round(Duration::minutes(1)).unwrap()
}

// Solstice calculation to determine a date's seasonal progression.
// Used in the Moonsighting Committee calculation method.
pub fn days_since_solstice(day_of_year: u32, year: u32, latitude: f64) -> i32 {
    let northern_offset = 10;
    let southern_offset = if is_leap_year(year) { 173 } else { 172 };
    let days_in_year = if is_leap_year(year) { 366 } else { 365 };

    if latitude >= 0.0 {
        let days_since_solstice = day_of_year as i32 + northern_offset;

        days_since_solstice
            - if days_since_solstice >= days_in_year {
                days_in_year
            } else {
                0
            }
    } else {
        let days_since_solstice = day_of_year as i32 - southern_offset;

        days_since_solstice
            + if days_since_solstice < 0 {
                days_in_year
            } else {
                0
            }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn calculate_julian_day() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);

        assert_eq!(julian_day, 2448908.5);
    }

    #[test]
    fn calculate_julian_century() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);

        assert_eq!(julian_century, -0.072_183_436_002_737_86);
    }

    #[test]
    fn calculate_mean_solar_longitude() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_longitude = mean_solar_longitude(julian_century);

        assert_eq!(mean_solar_longitude.degrees, 201.80719320670732);
    }

    #[test]
    fn calculate_apparent_solar_longitude() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_longitude = mean_solar_longitude(julian_century);
        let apparent_solar_longitude =
            apparent_solar_longitude(julian_century, mean_solar_longitude).radians();

        assert_eq!(apparent_solar_longitude, 3.489_069_182_045_206);
    }

    #[test]
    fn calculate_mean_obliquity_of_the_ecliptic() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_obliq_of_ecliptic = mean_obliquity_of_the_ecliptic(julian_century);

        assert_eq!(mean_obliq_of_ecliptic.degrees, 23.440229684413012);
    }

    #[test]
    fn calculate_apparent_obliquity_of_the_ecliptic() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_obliq_of_ecliptic = mean_obliquity_of_the_ecliptic(julian_century);
        let apparent_obliq_of_ecliptic =
            apparent_obliquity_of_the_ecliptic(julian_century, mean_obliq_of_ecliptic);

        assert_eq!(apparent_obliq_of_ecliptic.degrees, 23.43999110619955);
    }

    #[test]
    fn calculate_mean_solar_anomaly() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_anomaly = mean_solar_anomaly(julian_century);

        assert_eq!(mean_solar_anomaly.degrees, 278.993_966_431_597_5);
    }

    #[test]
    fn calculate_solar_equation_of_the_center() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_anomaly = mean_solar_anomaly(julian_century);
        let solar_equation_of_center =
            solar_equation_of_the_center(julian_century, mean_solar_anomaly);

        assert_eq!(solar_equation_of_center.degrees, -1.897323843371985);
    }

    #[test]
    fn calculate_mean_lunar_longitude() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_lunar_longitude = mean_lunar_longitude(julian_century);

        assert_eq!(mean_lunar_longitude.degrees, 38.747190008209145);
    }

    #[test]
    fn calculate_acending_lunar_node_longitude() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let ascending_lunar_node = ascending_lunar_node_longitude(julian_century);

        assert_eq!(ascending_lunar_node.degrees, 264.657131805429);
    }

    #[test]
    fn calculate_mean_sidereal_time() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_sidereal_time = mean_sidereal_time(julian_century);

        assert_eq!(mean_sidereal_time.degrees, 21.801339167752303);
    }

    #[test]
    fn calculate_nutation_longitude() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_longitude = mean_solar_longitude(julian_century);
        let mean_lunar_longitude = mean_lunar_longitude(julian_century);
        let ascending_lunar_node = ascending_lunar_node_longitude(julian_century);
        let nutation_longitude = nutation_in_longitude(
            mean_solar_longitude,
            mean_lunar_longitude,
            ascending_lunar_node,
        );

        assert_eq!(nutation_longitude, 0.0044525358169686564);
    }

    #[test]
    fn calculate_nutation_in_obliquity() {
        let julian_day = julian_day_ymdh(1992, 10, 13, 0.0);
        let julian_century = julian_century(julian_day);
        let mean_solar_longitude = mean_solar_longitude(julian_century);
        let mean_lunar_longitude = mean_lunar_longitude(julian_century);
        let ascending_lunar_node = ascending_lunar_node_longitude(julian_century);
        let nutation_obliq = nutation_in_obliquity(
            mean_solar_longitude,
            mean_lunar_longitude,
            ascending_lunar_node,
        );

        assert_eq!(nutation_obliq, -0.000_092_747_500_292_341_56);
    }

    #[test]
    fn calculate_altitude_of_celestial_body() {
        let coordinates = Coordinates::new(35.783_333_333_333_33, -78.65);
        let declination_angle = Angle::new(21.894701414701338);
        let local_hour_angle = Angle::new(108.09275357838322);
        let celestial_body = altitude_of_celestial_body(
            coordinates.latitude_angle(),
            declination_angle,
            local_hour_angle,
        );

        assert_eq!(celestial_body.degrees, -0.900_615_621_559_432_1);
    }
}
