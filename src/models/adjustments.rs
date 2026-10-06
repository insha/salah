// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::default::Default;

/// Time adjustment for all prayer times.
/// The value is specified in *minutes* and
/// can be either positive or negative.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TimeAdjustment {
    /// Minutes added to Fajr.
    pub fajr: i64,
    /// Minutes added to sunrise.
    pub sunrise: i64,
    /// Minutes added to Dhuhr.
    pub dhuhr: i64,
    /// Minutes added to Asr.
    pub asr: i64,
    /// Minutes added to Maghrib.
    pub maghrib: i64,
    /// Minutes added to Isha.
    pub isha: i64,
}

impl TimeAdjustment {
    /// Construct all six prayer adjustments in minutes.
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
///
/// # Examples
///
/// ```
/// use salah::Adjustment;
///
/// let adjustment = Adjustment::new()
///     .fajr(2)
///     .sunrise(-1)
///     .isha(3)
///     .done();
/// assert_eq!(adjustment.fajr, 2);
/// assert_eq!(adjustment.sunrise, -1);
/// assert_eq!(adjustment.isha, 3);
/// ```
pub struct Adjustment {
    fajr: i64,
    sunrise: i64,
    dhuhr: i64,
    asr: i64,
    maghrib: i64,
    isha: i64,
}

impl Adjustment {
    /// Create a zeroed adjustment builder.
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

    /// Add `fajr` minutes to the calculated Fajr time.
    pub fn fajr<'a>(&'a mut self, fajr: i64) -> &'a mut Adjustment {
        self.fajr = fajr;
        self
    }

    /// Add `sunrise` minutes to the calculated sunrise time.
    pub fn sunrise<'a>(&'a mut self, sunrise: i64) -> &'a mut Adjustment {
        self.sunrise = sunrise;
        self
    }

    /// Add `dhuhr` minutes to the calculated Dhuhr time.
    pub fn dhuhr<'a>(&'a mut self, dhuhr: i64) -> &'a mut Adjustment {
        self.dhuhr = dhuhr;
        self
    }

    /// Add `asr` minutes to the calculated Asr time.
    pub fn asr<'a>(&'a mut self, asr: i64) -> &'a mut Adjustment {
        self.asr = asr;
        self
    }

    /// Add `maghrib` minutes to the calculated Maghrib time.
    pub fn maghrib<'a>(&'a mut self, maghrib: i64) -> &'a mut Adjustment {
        self.maghrib = maghrib;
        self
    }

    /// Add `isha` minutes to the calculated Isha time.
    pub fn isha<'a>(&'a mut self, isha: i64) -> &'a mut Adjustment {
        self.isha = isha;
        self
    }

    /// Finish the builder and return the adjustment values.
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
