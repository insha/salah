// Salah
//
// See README.md and LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{adjustments::TimeAdjustments, parameters::Parameters};

/// Provides preset configuration for a few authorities
/// for calculating prayer times.
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Method {
    /// Muslim World League
    MuslimWorldLeague,

    /// Egyptian General Authority of Survey
    Egyptian,

    /// University of Islamic Sciences, Karachi
    Karachi,

    /// Umm al-Qura University, Makkah
    UmmAlQura,

    /// The Gulf Region
    Dubai,

    /// Moonsighting Committee
    MoonsightingCommittee,

    /// ISNA
    NorthAmerica,

    /// Kuwait
    Kuwait,

    /// Qatar
    Qatar,

    /// Singapore
    Singapore,

    /// Other
    Other,
}

impl Method {
    /// Get method parameters
    pub fn parameters(self) -> Parameters {
        match self {
            Method::MuslimWorldLeague => Parameters {
                fajr_angle: 18.,
                isha_angle: 17.,
                method: self,
                method_adjustments: TimeAdjustments {
                    dhuhr: 1,
                    ..Default::default()
                },
                ..Default::default()
            },

            Method::Egyptian => Parameters {
                fajr_angle: 19.5,
                isha_angle: 17.5,
                method: self,
                method_adjustments: TimeAdjustments {
                    dhuhr: 1,
                    ..Default::default()
                },
                ..Default::default()
            },

            Method::Karachi => Parameters {
                fajr_angle: 18.,
                isha_angle: 18.,
                method: self,
                method_adjustments: TimeAdjustments {
                    dhuhr: 1,
                    ..Default::default()
                },
                ..Default::default()
            },

            Method::UmmAlQura => Parameters {
                fajr_angle: 18.5,
                isha_interval: 90,
                method: self,
                ..Default::default()
            },

            Method::Dubai => Parameters {
                fajr_angle: 18.2,
                isha_angle: 18.2,
                method: self,
                method_adjustments: TimeAdjustments {
                    sunrise: -3,
                    dhuhr: 3,
                    asr: 3,
                    maghrib: 3,
                    ..Default::default()
                },
                ..Default::default()
            },

            Method::MoonsightingCommittee => Parameters {
                fajr_angle: 18.,
                isha_angle: 18.,
                method: self,
                method_adjustments: TimeAdjustments {
                    dhuhr: 5,
                    maghrib: 3,
                    ..Default::default()
                },
                ..Default::default()
            },

            Method::NorthAmerica => Parameters {
                fajr_angle: 15.,
                isha_angle: 15.,
                method: self,
                method_adjustments: TimeAdjustments {
                    dhuhr: 1,
                    ..Default::default()
                },
                ..Default::default()
            },

            Method::Kuwait => Parameters {
                fajr_angle: 18.,
                isha_angle: 17.5,
                method: self,
                ..Default::default()
            },

            Method::Qatar => Parameters {
                fajr_angle: 18.,
                isha_interval: 90,
                method: self,
                ..Default::default()
            },

            Method::Singapore => Parameters {
                fajr_angle: 20.,
                isha_angle: 18.,
                method: self,
                method_adjustments: TimeAdjustments {
                    dhuhr: 1,
                    ..Default::default()
                },
                ..Default::default()
            },

            Method::Other => Parameters::default(),
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn parameters_for_muslim_world_league() {
        let method = Method::MuslimWorldLeague;
        let params = method.parameters();

        assert_eq!(params.method, Method::MuslimWorldLeague);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 17.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_egyptian() {
        let method = Method::Egyptian;
        let params = method.parameters();

        assert_eq!(params.method, Method::Egyptian);
        assert_eq!(params.fajr_angle, 19.5);
        assert_eq!(params.isha_angle, 17.5);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_karachi() {
        let method = Method::Karachi;
        let params = method.parameters();

        assert_eq!(params.method, Method::Karachi);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 18.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_umm_al_qura() {
        let method = Method::UmmAlQura;
        let params = method.parameters();

        assert_eq!(params.method, Method::UmmAlQura);
        assert_eq!(params.fajr_angle, 18.5);
        assert_eq!(params.isha_angle, 0.0);
        assert_eq!(params.isha_interval, 90);
    }

    #[test]
    fn parameters_for_dubai() {
        let method = Method::Dubai;
        let params = method.parameters();

        assert_eq!(params.method, Method::Dubai);
        assert_eq!(params.fajr_angle, 18.2, "Parameters: {:?}", params);
        assert_eq!(params.isha_angle, 18.2);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_moonsighting_committee() {
        let method = Method::MoonsightingCommittee;
        let params = method.parameters();

        assert_eq!(params.method, Method::MoonsightingCommittee);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 18.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_north_america() {
        let method = Method::NorthAmerica;
        let params = method.parameters();

        assert_eq!(params.method, Method::NorthAmerica);
        assert_eq!(params.fajr_angle, 15.0);
        assert_eq!(params.isha_angle, 15.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_kuwait() {
        let method = Method::Kuwait;
        let params = method.parameters();

        assert_eq!(params.method, Method::Kuwait);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 17.5);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_qatar() {
        let method = Method::Qatar;
        let params = method.parameters();

        assert_eq!(params.method, Method::Qatar);
        assert_eq!(params.fajr_angle, 18.0);
        assert_eq!(params.isha_angle, 0.0);
        assert_eq!(params.isha_interval, 90);
    }

    #[test]
    fn parameters_for_singapore() {
        let method = Method::Singapore;
        let params = method.parameters();

        assert_eq!(params.method, Method::Singapore);
        assert_eq!(params.fajr_angle, 20.0);
        assert_eq!(params.isha_angle, 18.0);
        assert_eq!(params.isha_interval, 0);
    }

    #[test]
    fn parameters_for_other() {
        let method = Method::Other;
        let params = method.parameters();

        assert_eq!(params.method, Method::Other);
        assert_eq!(params.fajr_angle, 0.0);
        assert_eq!(params.isha_angle, 0.0);
        assert_eq!(params.isha_interval, 0);
    }
}
