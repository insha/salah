// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::default::Default;

/// Time adjustment for all prayer times.
/// The value is specified in *minutes* and
/// can be either positive or negative.

#[derive(Default, PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct TimeAdjustments {
    pub fajr: i64,
    pub sunrise: i64,
    pub dhuhr: i64,
    pub asr: i64,
    pub maghrib: i64,
    pub isha: i64,
}

impl TimeAdjustments {
    pub fn new(fajr: i64, sunrise: i64, dhuhr: i64, asr: i64, maghrib: i64, isha: i64) -> Self {
        TimeAdjustments {
            fajr,
            sunrise,
            dhuhr,
            asr,
            maghrib,
            isha,
        }
    }
}
