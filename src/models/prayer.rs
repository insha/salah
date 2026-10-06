// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use chrono::{Datelike, Utc, Weekday};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A prayer or solar event represented by a calculated schedule.
///
/// # Examples
///
/// ```
/// use salah::Prayer;
///
/// assert_eq!(Prayer::Fajr.name(), "Fajr");
/// assert_eq!(Prayer::FajrTomorrow.name(), "Fajr");
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Prayer {
    /// Dawn prayer, calculated from the Fajr solar angle.
    Fajr,
    /// Sunrise.
    Sunrise,
    /// Noon prayer, at solar transit.
    Dhuhr,
    /// Afternoon prayer.
    Asr,
    /// Sunset prayer.
    Maghrib,
    /// Night prayer, calculated from an angle, twilight, or interval.
    Isha,
    /// Night vigil time, calculated from the last third of the night.
    Qiyam,
    /// Fajr on the following calendar day, used for overnight navigation.
    FajrTomorrow,
}

impl Prayer {
    /// Return the English transliteration used for display.
    pub fn name(&self) -> String {
        match self {
            Prayer::Fajr | Prayer::FajrTomorrow => String::from("Fajr"),
            Prayer::Sunrise => String::from("Sunrise"),
            Prayer::Dhuhr => {
                if Utc::now().weekday() == Weekday::Fri {
                    String::from("Jumua")
                } else {
                    String::from("Dhuhr")
                }
            }
            Prayer::Asr => String::from("Asr"),
            Prayer::Maghrib => String::from("Maghrib"),
            Prayer::Isha => String::from("Isha"),
            Prayer::Qiyam => String::from("Qiyam"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prayer_name_for_fajr_en_transliteration() {
        assert_eq!(Prayer::Fajr.name(), "Fajr");
        assert_eq!(Prayer::Sunrise.name(), "Sunrise");

        if Utc::now().weekday() == Weekday::Fri {
            assert_eq!(Prayer::Dhuhr.name(), "Jumua");
        } else {
            assert_eq!(Prayer::Dhuhr.name(), "Dhuhr");
        }

        assert_eq!(Prayer::Asr.name(), "Asr");
        assert_eq!(Prayer::Maghrib.name(), "Maghrib");
        assert_eq!(Prayer::Isha.name(), "Isha");
        assert_eq!(Prayer::Qiyam.name(), "Qiyam");
    }
}
