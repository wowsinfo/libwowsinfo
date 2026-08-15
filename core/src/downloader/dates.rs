//! Recent-date helpers.


/// Last 10 days (yesterday back to ten days ago) as `YYYYMMDD` joined by
/// commas, matching `RecentDate` in the Flutter app.
#[must_use]
pub fn recent_dates(now_secs: i64) -> String {
    let days = now_secs.div_euclid(86_400);
    (1..=10)
        .map(|offset| {
            let (year, month, day) = civil_from_days(days - offset);
            format!("{year:04}{month:02}{day:02}")
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// Days since 1970-01-01 -> (year, month, day) using Howard Hinnant's
/// `civil_from_days` algorithm.
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as i64;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as i64;
    (if month <= 2 { year + 1 } else { year }, month, day)
}
