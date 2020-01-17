// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Rule for approximating Fajr and Isha at high latitudes
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HighLatitudeRule {
    MiddleOfTheNight,
    SeventhOfTheNight,
    TwilightAngle,
}
