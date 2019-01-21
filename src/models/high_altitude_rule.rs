// Salah
//
// See See README.md and LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

#[derive(PartialEq, Debug, Copy, Clone)]
// Rule for approximating Fajr and Isha at high latitudes
pub enum HighLatitudeRule {
    MiddleOfTheNight,
    SeventhOfTheNight,
    TwilightAngle,
}
