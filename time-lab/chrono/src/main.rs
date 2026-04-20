use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, TimeZone, Utc, Duration, FixedOffset, Local, Offset};

fn main() {
    test_naive_date();
    test_naive_time();
    test_naive_datetime();
    test_datetime_utc();
    test_datetime_local();
    test_datetime_formatting();
    test_datetime_parsing();
    test_datetime_arithmetic();
    test_duration();
    test_timestamp();
    test_timezone();
    test_from_timestamp();
    test_parse_from_rfc3339();
    
    println!("\nAll tests passed!");
}

fn test_naive_date() {
    println!("Testing NaiveDate...");
    
    let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    assert_eq!(date.year(), 2024);
    assert_eq!(date.month(), 3);
    assert_eq!(date.day(), 15);
    assert_eq!(date.weekday().to_string(), "Fri");
    assert_eq!(date.ordinal(), 75);
    assert_eq!(date, NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());
    assert!(date > NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    
    let today = Local::now().date_naive();
    assert!(today >= date);
    
    println!("  NaiveDate tests passed");
}

fn test_naive_time() {
    println!("Testing NaiveTime...");
    
    let time = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
    assert_eq!(time.hour(), 14);
    assert_eq!(time.minute(), 30);
    assert_eq!(time.second(), 45);
    assert_eq!(time.format("%H:%M:%S").to_string(), "14:30:45");
    assert_eq!(time, NaiveTime::from_hms_opt(14, 30, 45).unwrap());
    
    let time_with_millis = NaiveTime::from_hms_milli_opt(14, 30, 45, 500).unwrap();
    assert_eq!(time_with_millis.nanosecond(), 500_000_000);
    
    println!("  NaiveTime tests passed");
}

fn test_naive_datetime() {
    println!("Testing NaiveDateTime...");
    
    let naive = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap().and_hms_opt(14, 30, 45).unwrap();
    assert_eq!(naive.date(), NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());
    assert_eq!(naive.time(), NaiveTime::from_hms_opt(14, 30, 45).unwrap());
    assert_eq!(naive.year(), 2024);
    assert_eq!(naive.month(), 3);
    assert_eq!(naive.day(), 15);
    assert_eq!(naive.hour(), 14);
    assert_eq!(naive.minute(), 30);
    assert_eq!(naive.second(), 45);
    assert_eq!(naive.weekday().to_string(), "Fri");
    
    let parsed = NaiveDateTime::parse_from_str("2024-03-15 14:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(naive, parsed);
    
    println!("  NaiveDateTime tests passed");
}

fn test_datetime_utc() {
    println!("Testing DateTime<Utc>...");
    
    let utc_datetime = Utc::now();
    
    assert!(utc_datetime.year() >= 2024);
    assert!(utc_datetime.timestamp() > 0);
    
    let dt = Utc.timestamp_opt(1710500000, 0).single().unwrap();
    assert_eq!(dt.to_utc().year(), 2024);
    
    let utc_str = utc_datetime.to_rfc3339();
    assert!(utc_str.contains("+") || utc_str.ends_with("Z"));
    
    println!("  DateTime<Utc> tests passed");
}

fn test_datetime_local() {
    println!("Testing DateTime<Local>...");
    
    let local_now = Local::now();
    assert!(local_now.year() >= 2024);
    
    let offset = local_now.offset();
    let fix: FixedOffset = offset.fix();
    assert_eq!(fix.local_minus_utc(), offset.local_minus_utc());
    
    println!("  DateTime<Local> tests passed");
}

fn test_datetime_formatting() {
    println!("Testing DateTime formatting...");
    
    let dt = Utc::now();
    
    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    assert_eq!(formatted.len(), 19);
    
    let custom = dt.format("Date: %Y/%m/%d, Time: %H:%M").to_string();
    assert!(custom.starts_with("Date: "));
    
    let rfc = dt.to_rfc3339();
    assert!(rfc.len() >= 20);
    
    println!("  DateTime formatting tests passed");
}

fn test_datetime_parsing() {
    println!("Testing DateTime parsing...");
    
    let dt = NaiveDateTime::parse_from_str("2024-03-15 14:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 3);
    assert_eq!(dt.day(), 15);
    assert_eq!(dt.hour(), 14);
    assert_eq!(dt.minute(), 30);
    assert_eq!(dt.second(), 45);
    
    let dt_with_tz = DateTime::parse_from_rfc3339("2024-03-15T14:30:45Z").unwrap();
    assert_eq!(dt_with_tz.to_utc(), dt.and_utc());
    
    let dt_with_offset = DateTime::parse_from_rfc3339("2024-03-15T14:30:45+08:00").unwrap();
    assert_eq!(dt_with_offset.offset().local_minus_utc(), 8 * 3600);
    
    let result = NaiveDateTime::parse_from_str("invalid", "%Y-%m-%d");
    assert!(result.is_err());
    
    println!("  DateTime parsing tests passed");
}

fn test_datetime_arithmetic() {
    println!("Testing DateTime arithmetic...");
    
    let dt = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap().and_hms_opt(14, 30, 45).unwrap();
    
    let added = dt + Duration::days(1);
    assert_eq!(added.day(), 16);
    
    let added_hours = dt + Duration::hours(10);
    assert_eq!(added_hours.hour(), 0);
    assert_eq!(added_hours.day(), 16);
    
    let subtracted = dt - Duration::days(5);
    assert_eq!(subtracted.day(), 10);
    
    let subtracted_mins = dt - Duration::minutes(45);
    assert_eq!(subtracted_mins.minute(), 45);
    assert_eq!(subtracted_mins.hour(), 13);
    
    let duration = dt.signed_duration_since(
        NaiveDate::from_ymd_opt(2024, 3, 10).unwrap().and_hms_opt(10, 0, 0).unwrap()
    );
    assert_eq!(duration.num_days(), 5);
    
    println!("  DateTime arithmetic tests passed");
}

fn test_duration() {
    println!("Testing Duration...");
    
    let dur = Duration::days(5) + Duration::hours(12) + Duration::minutes(30);
    assert_eq!(dur.num_days(), 5);
    assert_eq!(dur.num_hours(), 5 * 24 + 12);
    assert_eq!(dur.num_minutes(), (5 * 24 + 12) * 60 + 30);
    assert_eq!(dur.num_seconds(), ((5 * 24 + 12) * 60 + 30) * 60);
    
    let neg_dur = Duration::days(-3);
    assert_eq!(neg_dur.num_days(), -3);
    
    let abs_dur = neg_dur.abs();
    assert_eq!(abs_dur.num_days(), 3);
    
    let dur_sum = Duration::hours(10) + Duration::hours(14);
    assert_eq!(dur_sum.num_hours(), 24);
    
    let dur_mul = Duration::hours(2) * 3;
    assert_eq!(dur_mul.num_hours(), 6);
    
    let zero = Duration::zero();
    assert_eq!(zero, Duration::seconds(0));
    
    println!("  Duration tests passed");
}

fn test_timestamp() {
    println!("Testing timestamp operations...");
    
    let dt = Utc::now();
    let ts = dt.timestamp();
    assert!(ts > 0);
    
    let dt_from_ts = DateTime::<Utc>::from_timestamp(ts, 0).unwrap();
    assert_eq!(dt_from_ts.timestamp(), ts);
    
    let naive = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
    let ts_2024 = naive.and_utc().timestamp();
    assert!(ts_2024 >= 1704067200);
    
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let ts_date = date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
    assert!(ts_date >= 0);
    
    println!("  timestamp operations tests passed");
}

fn test_timezone() {
    println!("Testing timezone operations...");
    
    let utc = Utc::now();
    let utc_tz = utc.with_timezone(&Utc);
    assert_eq!(utc, utc_tz);
    
    let fixed_offset = FixedOffset::east_opt(8 * 3600).unwrap();
    let local_dt = Utc::now().with_timezone(&fixed_offset);
    assert_eq!(local_dt.offset().local_minus_utc(), 8 * 3600);
    
    let neg_offset = FixedOffset::west_opt(5 * 3600).unwrap();
    let neg_dt = Utc::now().with_timezone(&neg_offset);
    assert_eq!(neg_dt.offset().local_minus_utc(), -5 * 3600);
    
    let datetime = fixed_offset.timestamp_opt(1710500000, 0).single().unwrap();
    assert!(datetime.year() >= 2024);
    
    println!("  timezone operations tests passed");
}

fn test_from_timestamp() {
    println!("Testing from_timestamp...");
    
    let dt_opt = DateTime::<Utc>::from_timestamp(1710500000, 0);
    assert!(dt_opt.is_some());
    
    let dt = dt_opt.unwrap();
    assert_eq!(dt.timestamp(), 1710500000);
    
    let dt_valid = DateTime::<Utc>::from_timestamp(0, 0);
    assert!(dt_valid.is_some());
    
    let dt_nanos = DateTime::<Utc>::from_timestamp(1710500000, 123456789);
    assert!(dt_nanos.is_some());
    
    println!("  from_timestamp tests passed");
}

fn test_parse_from_rfc3339() {
    println!("Testing parse_from_rfc3339...");
    
    let dt = DateTime::parse_from_rfc3339("2024-03-15T14:30:45Z").unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 3);
    assert_eq!(dt.day(), 15);
    assert_eq!(dt.hour(), 14);
    assert_eq!(dt.minute(), 30);
    assert_eq!(dt.second(), 45);
    assert_eq!(dt.offset().local_minus_utc(), 0);
    
    let dt2 = DateTime::parse_from_rfc3339("2024-03-15T14:30:45+08:00").unwrap();
    assert_eq!(dt2.offset().local_minus_utc(), 8 * 3600);
    
    let dt3 = DateTime::parse_from_rfc3339("2024-03-15T14:30:45-05:30").unwrap();
    assert_eq!(dt3.offset().local_minus_utc(), -5 * 3600 - 1800);
    
    let dt4 = DateTime::parse_from_rfc3339("2024-03-15T14:30:45.123Z").unwrap();
    assert!(dt4.nanosecond() > 0);
    
    let invalid = DateTime::parse_from_rfc3339("invalid");
    assert!(invalid.is_err());
    
    println!("  parse_from_rfc3339 tests passed");
}