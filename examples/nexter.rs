use salah::prelude::*;

fn main() {
    let city = Coordinates::new(24.383_144, 88.583_183); // Rajshahi
    let date = Local::today();
    let params = Parameters::with(Method::Karachi, Madhab::Hanafi);
    let prayers = PrayerTimes::calculate(date, city, params);

    println!(
        "Fajr: {}",
        prayers
            .time(Prayer::Fajr)
            .with_timezone(&Local)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "Sunrise: {}",
        prayers
            .time(Prayer::Sunrise)
            .with_timezone(&Local)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "Dhuhr: {}",
        prayers
            .time(Prayer::Dhuhr)
            .with_timezone(&Local)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "Asr: {}",
        prayers
            .time(Prayer::Asr)
            .with_timezone(&Local)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "Maghrib: {}",
        prayers
            .time(Prayer::Maghrib)
            .with_timezone(&Local)
            .format("%-l:%M %p")
            .to_string()
    );
    println!(
        "Isha: {}",
        prayers
            .time(Prayer::Isha)
            .with_timezone(&Local)
            .format("%-l:%M %p")
            .to_string()
    );

    println!();

    println!(
        "Fajr   : {}",
        prayers
            .time(Prayer::Fajr)
            .with_timezone(&Local)
            .to_rfc3339()
    );
    println!(
        "Sunrise: {}",
        prayers
            .time(Prayer::Sunrise)
            .with_timezone(&Local)
            .to_rfc3339()
    );
    println!(
        "Dhuhr  : {}",
        prayers
            .time(Prayer::Dhuhr)
            .with_timezone(&Local)
            .to_rfc3339()
    );
    println!(
        "Asr    : {}",
        prayers.time(Prayer::Asr).with_timezone(&Local).to_rfc3339()
    );
    println!(
        "Maghrib: {}",
        prayers
            .time(Prayer::Maghrib)
            .with_timezone(&Local)
            .to_rfc3339()
    );
    println!(
        "Isha   : {}",
        prayers
            .time(Prayer::Isha)
            .with_timezone(&Local)
            .to_rfc3339()
    );
    println!(
        "Qiyam  : {}",
        prayers
            .time(Prayer::Qiyam)
            .with_timezone(&Local)
            .to_rfc3339()
    );

    println!();
    println!("Now    : {}", Local::now().to_rfc3339());
    println!();

    println!(
        "Current prayer: {} ({} minutes remaining)",
        prayers.current().name(),
        prayers.time_remaining().num_minutes()
    );

    println!(
        "Next prayer: {} @ {}",
        prayers.next().name(),
        prayers
            .time(prayers.next())
            .with_timezone(&Local)
            .format("%-l:%M %p")
            .to_string()
    );

    let t11am = Local::today().and_hms(5, 0, 0).with_timezone(&Local);
    let pat11am = prayers.prayer_at(t11am);

    assert_eq!(pat11am, Prayer::Fajr);

    println!(
        "Prayer @ 11am: {} ({} minutes remaining)",
        pat11am.name(),
        (prayers.time(pat11am.next()) - t11am).num_minutes()
    );

    println!(
        "Next prayer @ 11am: {} @ {}",
        pat11am.next().name(),
        prayers
            .time(pat11am.next())
            .with_timezone(&Local)
            .format("%-l:%M %p")
            .to_string()
    );
}
