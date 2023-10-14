# Salah

![Build Status](https://github.com/insha/salah/actions/workflows/rust.yml/badge.svg)
[![Crate.io][cratesio-image]][cratesio]
[![Docs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/salah.svg
[cratesio]: https://crates.io/crates/salah
[docsrs-image]: https://docs.rs/mio/badge.svg
[docsrs]: https://docs.rs/salah/

## Usage

Add the following to your `Cargo.toml` file under the `[dependencies]` section:

```
[dependencies]
salah = "0.7.1"
```

To get prayer times, use the `PrayerSchedule` struct passing in coordinates, date, and calculation parameters.

```rust
use salah::prelude::*;

let new_york_city = Coordinates::new(40.7128, -74.0059);
let date          = Utc.ymd(2019, 1, 25);
let params        = Configuration::with(Method::MoonsightingCommittee, Madhab::Hanafi);
let prayers       = PrayerSchedule::new()
                      .on(date)
                      .for_location(new_york_city)
                      .with_configuration(params)
                      .calculate();
```

### Initialization parameters

#### Coordinates

Create a `Coordinates` struct with the latitude and longitude for the location
you want prayer times for.

```rust
let coordinates = Coordinates::new(40.7128, -74.0059);
```

#### Date

To avoid confusion with timezones the date parameter passed in should be an instance of
`Date<Utc>`. The year, month, and day components need to be populated. All other
components will be ignored.

```rust
let date = Utc.ymd(2019, 1, 25);
```

#### Configuration

The calculation of the prayer times requires certain pieces of information. These can configured using the `Configuration` struct, a builder for the underlaying `Parameters` struct. This struct can be initialized by passing one of the `Method` enum variants (see below for the available variants) along with the one of the `Madhab` enum variants. You can then further customize the calculation parameters if needed.

```rust
let params = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
```

| Parameter | Description |
| --------- | ----------- |
| `method`    | Which preset from the CalculationMethod enum was used. Default value is `other`. |
| `fajr_angle` | Angle of the sun below the horizon used to calculate Fajr. |
| `maghrib_angle` | Angle of the sun below the horizon used to calculate Maghrib, used for some Calculation Methods. |
| `isha_angle` | Angle of the sun below the horizon used to calculate Isha. |
| `isha_interval` | Minutes after Maghrib (if set, the time for Isha will be Maghrib plus `isha_interval`). |
| `madhab` | Which setting from the Madhab enum to use for calculating Asr. |
| `high_latitude_rule` | Which setting from the HighLatitudeRule enum to use for calculating the minimum time for Fajr and the maximum time for Isha. |
| `adjustments` | PrayerAdjustments struct with custom prayer time adjustments in minutes for each prayer time. |
| `rounding` | The behavior for rounding prayer times. Either to nearest minute, to the higher minute, or none.  |
| `shafaq` | Used by the MoonsightingCommittee method to determine how to calculate Isha. See explanation of values below. |

**Method**

Provides preset configuration for a few authorities for calculating prayer times.

| Value | Description |
| ----- | ----------- |
| `MuslimWorldLeague` | Muslim World League. Standard Fajr time with an angle of 18°. Earlier Isha time with an angle of 17°. |
| `Egyptian` | Egyptian General Authority of Survey. Early Fajr time using an angle 19.5° and a slightly earlier Isha time using an angle of 17.5°. |
| `Karachi` | University of Islamic Sciences, Karachi. A generally applicable method that uses standard Fajr and Isha angles of 18°. |
| `UmmAlQura` | Umm al-Qura University, Makkah. Uses a fixed interval of 90 minutes from maghrib to calculate Isha. And a slightly earlier Fajr time with an angle of 18.5°. *Note: you should add a +30 minute custom adjustment for Isha during Ramadan.* |
| `Dubai` | Used in the UAE. Slightly earlier Fajr time and slightly later Isha time with angles of 18.2° for Fajr and Isha in addition to 3 minute offsets for sunrise, Dhuhr, Asr, and Maghrib. |
| `Qatar` | Same Isha interval as `ummAlQura` but with the standard Fajr time using an angle of 18°. |
| `Kuwait` | Standard Fajr time with an angle of 18°. Slightly earlier Isha time with an angle of 17.5°. |
| `MoonsightingCommittee` | Method developed by Khalid Shaukat, founder of Moonsighting Committee Worldwide. Uses standard 18° angles for Fajr and Isha in addition to seasonal adjustment values. This method automatically applies the 1/7 approximation rule for locations above 55° latitude. Recommended for North America and the UK. |
| `Singapore` | Used in Singapore, Malaysia, and Indonesia. Early Fajr time with an angle of 20° and standard Isha time with an angle of 18°. |
| `Turkey` | An approximation of the Diyanet method used in Turkey. This approximation is less accurate outside the region of Turkey. |
| `Tehran` | Institute of Geophysics, University of Tehran. Early Isha time with an angle of 14°. Slightly later Fajr time with an angle of 17.7°. Calculates Maghrib based on the sun reaching an angle of 4.5° below the horizon. |
| `NorthAmerica` | Also known as the ISNA method. Can be used for North America, but the moonsightingCommittee method is preferable. Gives later Fajr times and early Isha times with angles of 15°. |
| `Other` | Defaults to angles of 0°, should generally be used for making a custom method and setting your own values. |

**Madhab**

Setting for the Asr prayer time. For Hanafi madhab, the Asr is bit later than that of the Shafi madhab.

| Value | Description |
| ----- | ----------- |
| `Shafi` | Earlier Asr time (use for Shafi, Maliki, Hanbali, and Jafari) |
| `Hanafi` | Later Asr time |

**HighLatitudeRule**

Rule for approximating Fajr and Isha at high latitudes.

| Value | Description |
| ----- | ----------- |
| `MiddleOfTheNight` | Fajr won't be earlier than the midpoint of the night and isha won't be later than the midpoint of the night. This is the default value to prevent fajr and isha crossing boundaries. |
| `SeventhOfTheNight` | Fajr will never be earlier than the beginning of the last seventh of the night and Isha will never be later than the end of the first seventh of the night. This is recommended to use for locations above 48° latitude to prevent prayer times that would be difficult to perform. |
| `TwilightAngle` | The night is divided into portions of roughly 1/3. The exact value is derived by dividing the fajr/isha angles by 60. This can be used to prevent difficult fajr and isha times at certain locations. |

You can get the recommended High Latitude Rule for a location by calling the `recommended(coordinates:)` function and passing in the coordinates for the location.

```rust
let myCoordinates = Coordinates { latitude: 48.983226, longitude: -3.216649 };
let highLatRule = HighLatitudeRule::recommended(myCoordinates);
```

**Shafaq**

Shafaq is used by the MoonsightingCommittee method to determine what type of twilight to use in order to determine the time for Isha.

| Value | Description |
| ----- | ----------- |
| `General` | General is a combination of Ahmer and Abyad. This is the defualt value and will provide more reasonable times for locations at higher latitudes. |
| `Ahmer` | Ahmer means the twilight is the red glow in the sky. Used by the Shafi, Maliki, and Hanbali madhabs. This generally produces an earlier Isha time. |
| `Abyad` | Abyad means the twilight is the white glow in the sky. Used by the Hanafi madhab. This generally produces a later Isha time. |

### Prayer Schedule

The `PrayerSchedule` struct is a builder for the the `PrayerTimes` struct. Once the `calculate()` method is invoked on it, a `PrayerTime` struct will be initialized and it will contain fields
for all five prayer times, the time for sunrise, and for the Qiyam prayer. 

The prayer time will be an instance of `DateTime<Utc>` and as such will refer to a fixed point in universal time. To display these times for the local timezone you will need to format them with the appropriate local time zone.

This struct provides convenience methods for the prayer times to ease their usage and display.

**PrayerTime**

| Method | Description |
| ------ | ----------- |
| `name()` | Returns the name of the payer transliterated in English. |
| `time(prayer: Prayer)` | Returns the time of the prayer as a `DateTime<Utc>`. See the `DateTime` documentation for manipulating the return value. |
| `current()` | Returns the current prayer as the `Prayer` type. |
| `next()` | Returns the next prayer as the `Prayer` type. |
| `time_remaining()` | Returns a tuple with the *hours* as its first element, and *minutes* as its second element. The value is always in the context of the current prayer. |

**Prayer**

This is an enum and has variants for all prayers, including, *sunrise* and *Qiyam*. This is single method available for this type called, `name()`, that will return the name of the prayer transliterated into English.

## Full Example

```rust
use salah::prelude::*;

let new_york_city = Coordinates::new(40.7128, -74.0059);
let date          = Utc.ymd(2019, 1, 25);
let params        = Configuration::with(Method::NorthAmerica, Madhab::Hanafi);
let prayers       = PrayerSchedule::new()
                        .on(date)
                        .for_location(new_york_city)
                        .with_configuration(params)
                        .calculate();

match prayers
{
    Ok(prayer) => {
        println!("{}: {}", Prayer::Fajr.name(), prayer.time(Prayer::Fajr).format("%-l:%M %p").to_string());
        println!("{}: {}", Prayer::Sunrise.name(), prayer.time(Prayer::Sunrise).format("%-l:%M %p").to_string());
        println!("{}: {}", Prayer::Dhuhr.name(), prayer.time(Prayer::Dhuhr).format("%-l:%M %p").to_string());
        println!("{}: {}", Prayer::Asr.name(), prayer.time(Prayer::Asr).format("%-l:%M %p").to_string());
        println!("{}: {}", Prayer::Maghrib.name(), prayer.time(Prayer::Maghrib).format("%-l:%M %p").to_string());
        println!("{}: {}", Prayer::Isha.name(), prayer.time(Prayer::Isha).format("%-l:%M %p").to_string());
        println!("{}: {}", Prayer::Qiyam.name(), prayer.time(Prayer::Qiyam).format("%-l:%M %p").to_string());
    },
    Err(error) => println!("Could not calculate prayer times: {}", error)
}
```

The output will be (in UTC):

```
Fajr: 10:53 AM
Sunrise: 12:12 PM
Dhuhr: 5:09 PM
Asr: 8:24 PM
Maghrib: 10:05 PM
Isha: 11:24 PM
Qiyam: 6:37 AM
```

## Convenience Utilities

The `PrayerTimes` struct has functions for getting the current prayer and the next prayer. You can also get the time for a specified prayer, making it
easier to dynamically show countdowns until the next prayer.

```rust
...
let prayers = PrayerSchedule::new()
                .on(date)
                .for_location(new_york_city)
                .with_configuration(params)
                .calculate();
let (hours, minutes) = prayers.current().time_remaining();

println!("Current: {} ({}:{})", prayers.current().name, hours, minutes);
println!("Next prayer is {} at {}.", prayers.next().name, prayers.time(prayer.next()).format("%-l:%M %p").to_string()));
```

### Qibla Direction

Get the direction, in degrees from North, of the Qibla from a given set of coordinates.

```rust
let new_york_city   = Coordinates::new(40.7128, -74.0059);
let qiblah_direction = Qiblah::new(new_york_city);

println!("Qiblah: {}", qibla_direction); //  Outputs: Qiblah: 58.4817
```

TO access that actual numerical (f64) value of the qiblah direction, you can use the `value()` method on the `Qiblah` instance.

```rust
let new_york_city   = Coordinates::new(40.7128, -74.0059);
let qiblah_direction = Qiblah::new(new_york_city);

println!("Qiblah: {}", qibla_direction.value()); //  Outputs: Qiblah: 58.48176358718943
```

## Contributing

Please see the `CONTRIBUTING.md` file for more information.

## Code of Conduct

Our contributor code of conduct can be found in the `code-of-conduct.md` file.

## Acknowledgement

This library is based on the [Adhan](https://github.com/batoulapps/Adhan) library by Batoul Apps. All astronomical calculations are high precision equations directly from the book [Astronomical Algorithms](http://www.willbell.com/math/mc1.htm) by Jean Meeus.

## License

Salah is licensed under a three clause BSD License. It basically means: do whatever you want with it as long as the copyright in Salah sticks around, the conditions are not modified and the disclaimer is present. Furthermore you must not use the names of the authors to promote derivatives of the software without written consent.

The full license text can be found in the `LICENSE` file.
