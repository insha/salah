// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019 Farhan Ahmed. All rights reserved.
//

use chrono::{Datelike, Local, Weekday};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Names of all obligatory prayers,
/// sunrise, and Qiyam.
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Prayer {
    QiyamYesterday,
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    Qiyam,
    FajrTomorrow,
}

impl Prayer {
    pub fn name(&self) -> String {
        match self {
            Prayer::Fajr | Prayer::FajrTomorrow => String::from("Fajr"),
            Prayer::Sunrise => String::from("Sunrise"),
            Prayer::Dhuhr => {
                if Local::now().weekday() == Weekday::Fri {
                    String::from("Jumua")
                } else {
                    String::from("Dhuhr")
                }
            }
            Prayer::Asr => String::from("Asr"),
            Prayer::Maghrib => String::from("Maghrib"),
            Prayer::Isha => String::from("Isha"),
            Prayer::Qiyam | Prayer::QiyamYesterday => String::from("Qiyam"),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Prayer::Fajr => Prayer::Sunrise,
            Prayer::Sunrise => Prayer::Dhuhr,
            Prayer::Dhuhr => Prayer::Asr,
            Prayer::Asr => Prayer::Maghrib,
            Prayer::Maghrib => Prayer::Isha,
            Prayer::Isha => Prayer::Qiyam,
            Prayer::Qiyam => Prayer::FajrTomorrow,
            Prayer::QiyamYesterday => Prayer::Fajr,
            Prayer::FajrTomorrow => unreachable!(),
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

        if Local::now().weekday() == Weekday::Fri {
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
