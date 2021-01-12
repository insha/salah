# Salah

[![Build Status](https://travis-ci.org/insha/salah.svg?branch=master)](https://travis-ci.org/insha/salah)
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
salah = "0.5.0"
```

To get prayer times, use the `PrayerSchedule` struct passing in coordinates, date, and calculation parameters.

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
| `method`    | Member of `Method` enum |
| `fajr_angle` | Angle of the sun used to calculate Fajr |
| `isha_angle` | Angle of the sun used to calculate Isha |
| `isha_interval` | Minutes after Maghrib (if set, the time for Isha will be Maghrib plus `isha_interval`) |
| `madhab` | Member of the `Madhab` enum, used to calculate Asr |
| `high_latitude_rule` | Member of the `HighLatitudeRule` enum, used to set a minimum time for Fajr and a max time for Isha |
| `time_adjustments` | `TimeAdjustment` struct with adjustment based on the selected `Method`. |
| `custom_time_adjustments` | `TimeAdjustment` struct with custom prayer time adjustments in minutes for each prayer time. By default, all values are `0`.|

**Method**

Provides preset configuration for a few authorities for calculating prayer times.

| Value | Description |
| ----- | ----------- |
| `MuslimWorldLeague` | Muslim World League. Fajr angle: 18, Isha angle: 17 |
| `Egyptian` | Egyptian General Authority of Survey. Fajr angle: 19.5, Isha angle: 17.5 |
| `Karachi` | University of Islamic Sciences, Karachi. Fajr angle: 18, Isha angle: 18 |
| `UmmAlQura` | Umm al-Qura University, Makkah. Fajr angle: 18.5, Isha interval: 90. *Note: you should add a +30 minute custom adjustment for Isha during Ramadan.* |
| `Dubai` | Method used in UAE. Fajr angle: 18.2, Isha angle: 18.2. |
| `Qatar` | Modified version of Umm al-Qura used in Qatar. Fajr angle: 18, Isha interval: 90. |
| `Kuwait` | Method used by the country of Kuwait. Fajr angle: 18, Isha angle: 17.5 |
| `MoonsightingCommittee` | Moonsighting Committee. Fajr angle: 18, Isha angle: 18. Also uses seasonal adjustment values. |
| `Singapore` | Method used by Singapore. Fajr angle: 20, Isha angle: 18. |
| `NorthAmerica` | Referred to as the ISNA method. Fajr angle: 15, Isha angle: 15 |
| `Other` | Fajr angle: 0, Isha angle: 0. This is the default value for when manually initializing the `Parameters` struct. |

**Madhab**

Setting for the Asr prayer time. For Hanafi madhab, the Asr is bit later than that of the Shafi madhab.

| Value | Description |
| ----- | ----------- |
| `Shafi` | Earlier Asr time |
| `Hanafi` | Later Asr time |

**HighLatitudeRule**

Rule for approximating Fajr and Isha at high latitudes.

| Value | Description |
| ----- | ----------- |
| `MiddleOfTheNight` | Fajr will never be earlier than the middle of the night and Isha will never be later than the middle of the night |
| `SeventhOfTheNight` | Fajr will never be earlier than the beginning of the last seventh of the night and Isha will never be later than the end of the first seventh of the night |
| `TwilightAngle` | Similar to SeventhOfTheNight, but instead of 1/7, the fraction of the night used is fajr_angle/60 and isha_angle/60 |


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
let qibla_direction = Qibla::new(coordinates: new_york_city);

println!("Qiblah: {:?}", qibla_direction); //  Outputs: Qiblah: 58.4817
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
