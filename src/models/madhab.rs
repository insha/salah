// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Setting for the Asr prayer time.
/// For Hanafi madhab, the Asr is bit later
/// than that of the Shafi madhab.
///
/// The distinction is the shadow-length factor used after solar transit:
/// Shafi uses one shadow length, while Hanafi uses two.
///
/// # Examples
///
/// ```
/// use salah::Madhab;
///
/// assert!(Madhab::Hanafi.shadow() > Madhab::Shafi.shadow());
/// ```

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Madhab {
    Shafi = 1,
    Hanafi = 2,
}

impl Madhab {
    /// Return the shadow-length multiplier used for Asr.
    ///
    /// A value of one is the earlier Shafi calculation; a value of two is
    /// the later Hanafi calculation.
    pub fn shadow(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shafi_shadow() {
        let shafi = Madhab::Shafi;

        assert_eq!(shafi.shadow(), 1);
    }

    #[test]
    fn hanafi_shadow() {
        let hanafi = Madhab::Hanafi;

        assert_eq!(hanafi.shadow(), 2);
    }
}
