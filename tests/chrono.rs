use ezrs::*;

#[test]
fn test_chrono_datetime() {
    let _: DateTime<Utc> = Utc::now();
    let _: DateTime<Local> = Local::now();
    let _ = Utc::today();
    let _ = Local::today();
}

#[test]
fn test_chrono_format() {
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    assert_eq!(
        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        "2014-11-28 12:00:09"
    );
    assert_eq!(dt.to_string(), "2014-11-28 12:00:09 UTC");
}

#[test]
fn test_chrono_parse() {
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let dt1: DateTime<Utc> = "2014-11-28T12:00:09Z".parse().unwrap();
    let dt2: DateTime<Utc> = "2014-11-28T21:00:09+09:00".parse().unwrap();
    assert_eq!(dt1, dt);
    assert_eq!(dt2, dt);

    let fixed_dt = dt.with_timezone(&FixedOffset::east(9 * 3600));
    let dt3: DateTime<FixedOffset> = "2014-11-28T21:00:09+09:00".parse().unwrap();
    assert_eq!(dt3, fixed_dt);
}
