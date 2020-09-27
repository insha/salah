// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

use crate::astronomy::unit::{Angle, Coordinates};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
/// Qiblah
pub struct Qiblah;

impl Qiblah {
    /// Get Qiblah angle from a set of location coordinates
    pub fn from_coords(location: Coordinates) -> f64 {
        // Equation from "Spherical Trigonometry For the use
        // of colleges and schools" page 50
        let makkah_coordinates = Coordinates::new(21.4225241, 39.8261818);
        let term1 = (makkah_coordinates.longitude_angle().radians()
            - location.longitude_angle().radians())
        .sin();
        let term2 = makkah_coordinates.latitude_angle().radians().tan()
            * location.latitude_angle().radians().cos();
        let term3 = (makkah_coordinates.longitude_angle().radians()
            - location.longitude_angle().radians())
        .cos()
            * location.latitude_angle().radians().sin();
        let term4 = term1.atan2(term2 - term3);

        Angle::from_radians(term4).unwound().degrees
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn qiblah_direction_from_nyc_in_north_america() {
        let nyc = Coordinates::new(40.7128, -74.0059);
        let qiblah = Qiblah::from_coords(nyc);

        assert_that!(qiblah).is_close_to(58.4817635, 0.0000001f64);
    }

    #[test]
    fn qiblah_direction_from_sf_in_north_america() {
        let sf = Coordinates::new(37.7749, -122.4194);
        let qiblah = Qiblah::from_coords(sf);

        assert_eq!(qiblah, 18.843822245692426);
    }

    #[test]
    fn qiblah_direction_from_dc_in_north_america() {
        let dc = Coordinates::new(38.9072, -77.0369);
        let qiblah = Qiblah::from_coords(dc);

        assert_eq!(qiblah, 56.56046821463599);
    }

    #[test]
    fn qiblah_direction_from_anchorage_in_north_america() {
        let dc = Coordinates::new(61.2181, -149.9003);
        let qiblah = Qiblah::from_coords(dc);

        assert_eq!(qiblah, 350.8830761159853);
    }

    #[test]
    fn qiblah_directioon_from_sydney_australia() {
        let sydney = Coordinates::new(-33.8688, 151.2093);
        let qiblah = Qiblah::from_coords(sydney);

        assert_eq!(qiblah, 277.4996044487399);
    }

    #[test]
    fn qiblah_directioon_from_auckland_new_zealand() {
        let auckland = Coordinates::new(-36.8485, 174.7633);
        let qiblah = Qiblah::from_coords(auckland);

        assert_eq!(qiblah, 261.19732640365845);
    }

    #[test]
    fn qiblah_direction_from_london_united_kingdom() {
        let london = Coordinates::new(51.5074, -0.1278);
        let qiblah = Qiblah::from_coords(london);

        assert_that!(qiblah).is_close_to(118.9872189, 0.0000001f64);
    }

    #[test]
    fn qiblah_direction_from_paris_france() {
        let paris = Coordinates::new(48.8566, 2.3522);
        let qiblah = Qiblah::from_coords(paris);

        assert_eq!(qiblah, 119.16313542183347);
    }

    #[test]
    fn qiblah_direction_from_oslo_norway() {
        let oslo = Coordinates::new(59.9139, 10.7522);
        let qiblah = Qiblah::from_coords(oslo);

        assert_eq!(qiblah, 139.02785605537514);
    }

    #[test]
    fn qiblah_direction_from_islamabad_pakistan() {
        let islamabad = Coordinates::new(33.7294, 73.0931);
        let qiblah = Qiblah::from_coords(islamabad);

        assert_eq!(qiblah, 255.8816156785436);
    }

    #[test]
    fn qiblah_direction_from_tokyo_japan() {
        let tokyo = Coordinates::new(35.6895, 139.6917);
        let qiblah = Qiblah::from_coords(tokyo);

        assert_eq!(qiblah, 293.02072441441163);
    }
}
