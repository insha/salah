// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2021 Farhan Ahmed. All rights reserved.
//

use std::default::Default;

/// Time adjustment for all prayer times.
/// The value is specified in *minutes* and
/// can be either positive or negative.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TimeAdjustment {
    pub fajr: i64,
    pub sunrise: i64,
    pub dhuhr: i64,
    pub asr: i64,
    pub maghrib: i64,
    pub isha: i64,
}

impl TimeAdjustment {
    pub fn new(fajr: i64, sunrise: i64, dhuhr: i64, asr: i64, maghrib: i64, isha: i64) -> Self {
        TimeAdjustment {
            fajr: fajr,
            sunrise: sunrise,
            dhuhr: dhuhr,
            asr: asr,
            maghrib: maghrib,
            isha: isha,
        }
    }
}

impl Default for TimeAdjustment {
    fn default() -> TimeAdjustment {
        TimeAdjustment {
            fajr: 0,
            sunrise: 0,
            dhuhr: 0,
            asr: 0,
            maghrib: 0,
            isha: 0,
        }
    }
}

/// Builder struct for the [TimeAdjustment](struct.TimeAdjustment.html).
/// It is recommended to use this for all needed adjustments.
pub struct Adjustment {
    fajr: i64,
    sunrise: i64,
    dhuhr: i64,
    asr: i64,
    maghrib: i64,
    isha: i64,
}

impl Adjustment {
    pub fn new() -> Adjustment {
        Adjustment {
            fajr: 0,
            sunrise: 0,
            dhuhr: 0,
            asr: 0,
            maghrib: 0,
            isha: 0,
        }
    }

    pub fn fajr<'a>(&'a mut self, fajr: i64) -> &'a mut Adjustment {
        self.fajr = fajr;
        self
    }

    pub fn sunrise<'a>(&'a mut self, sunrise: i64) -> &'a mut Adjustment {
        self.sunrise = sunrise;
        self
    }

    pub fn dhuhr<'a>(&'a mut self, dhuhr: i64) -> &'a mut Adjustment {
        self.dhuhr = dhuhr;
        self
    }

    pub fn asr<'a>(&'a mut self, asr: i64) -> &'a mut Adjustment {
        self.asr = asr;
        self
    }

    pub fn maghrib<'a>(&'a mut self, maghrib: i64) -> &'a mut Adjustment {
        self.maghrib = maghrib;
        self
    }

    pub fn isha<'a>(&'a mut self, isha: i64) -> &'a mut Adjustment {
        self.isha = isha;
        self
    }

    pub fn done(&self) -> TimeAdjustment {
        TimeAdjustment {
            fajr: self.fajr,
            sunrise: self.sunrise,
            dhuhr: self.dhuhr,
            asr: self.asr,
            maghrib: self.maghrib,
            isha: self.isha,
        }
    }
}
