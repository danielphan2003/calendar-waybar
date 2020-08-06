use serde_json::json;
use chrono::prelude::*;

const SPACE: usize = 7;

const WEEKDAY: [&'static str; 7] = 
    ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn get_num_days(year: i32, month: u32) -> i32 {
    match month {
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => 0
    }
}

fn build_weekdays(curr_month: &DateTime<Local>, prev_month: &NaiveDate, prev_month_days: i32, week_number: bool) -> String {
    let week = prev_month.iso_week().week();
    let mut line = if week_number {
        if week < 10 {
            format!("{:>pad_left$}", 
                format!("#{}{}", 0, week), 
                pad_left=SPACE
            )
        } else {
            format!("{:>pad_left$}", 
                week, 
                pad_left=SPACE
            )
        }
    } else {
        String::from("")
    };


    let weekday: i32 = prev_month.weekday().number_from_monday() as i32;
    let day = prev_month.day() as i32;

    let begin_week = if day - weekday < 0 {
        prev_month_days + day - weekday + 1
    } else {
        day - weekday + 1
    };

    let end_week = day + (7 - weekday) + 1;

    if begin_week > end_week {
        for i in begin_week..=prev_month_days {
            let formatted = format!("{:>pad_left$}", i, pad_left=SPACE);
            line += &formatted;
        }
        for i in day..end_week {
            let mut formatted = format!("{:>pad_left$}", format!("{}{}", 0, i), pad_left=SPACE);
            if i == curr_month.day() as i32 && prev_month.month() == curr_month.month() {
                formatted = format!("{:>pad_left$}", " ", pad_left=SPACE);
            }
            line += &formatted;
        }
    } else {
        for i in begin_week..end_week {
            let mut formatted = format!("{:>pad_left$}", i, pad_left=SPACE);
            if i < 10 { 
                formatted = format!("{:>pad_left$}", format!("{}{}", 0, i), pad_left=SPACE); 
            }
            if i == curr_month.day() as i32 {
                formatted = format!("{:>pad_left$}", " ", pad_left=SPACE);
            }
            line += &formatted;
        }
    }

    line
}

fn main() {
    let local: DateTime<Local> = Local::now();
    let month_format = format!(" {}", local.format("%B %Y"));
    let week_format = format!("Week #{}", local.format("%V"));
    let mut calendar_format = WEEKDAY.iter()
        .fold(format!("{:>pad_left$}", "", pad_left=SPACE+5), |calendar, &i| {
            calendar + &format!("{:>pad_left$}", i, pad_left=SPACE-1)
        });

    let week_number = true;
    let month_days = get_num_days(local.year(), local.month());
    let prev_month_days = get_num_days(local.year(), local.month() - 1);
    let weeks: i32 = 
        NaiveDate::from_ymd(
            local.year(), local.month(), month_days as u32)
            .iso_week().week() as i32
        - NaiveDate::from_ymd(
            local.year(), local.month(), 1)
            .iso_week().week() as i32 + 1;
            
    for i in 0..weeks {
        let mut current_date = 7*i+1;
        let mut current_month = local.month();
        let mut current_year = local.year();

        if month_days < current_date {
            current_month += 1;
            if current_month + 1 > 12 {
                current_month = 1;
                current_year += 1;
            }
            current_date = 1
        } 
        
        let time = NaiveDate::from_ymd(current_year, current_month, current_date as u32);
        calendar_format +=  &format!("\n{}", &build_weekdays(
            &local, &time, prev_month_days, week_number));
    }
    let tooltip = format!("{}\n{}\n{}", month_format, week_format, calendar_format);

    let calendar = json!({
        "class": "date",
        "text": local.format("%A, %B%e, %Y").to_string(),
        "tooltip": tooltip,
    });

    // Convert to a string of JSON and print it out
    println!("{}", calendar.to_string());
}