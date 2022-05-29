// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(clippy::cast_lossless)]

mod timestamp {
    pub struct Timestamp(i64);

    impl From<i64> for Timestamp {
        fn from(timestamp: i64) -> Self {
            Self(timestamp)
        }
    }

    impl From<time::OffsetDateTime> for Timestamp {
        fn from(dt: time::OffsetDateTime) -> Self {
            dt.unix_timestamp().into()
        }
    }

    impl From<Timestamp> for time::OffsetDateTime {
        fn from(ts: Timestamp) -> Self {
            time::OffsetDateTime::from_unix_timestamp(ts.0)
                .expect("unable to convert timestamp to time::OffsetDateTime")
        }
    }

    // impl From<egui_datepicker::Date<egui_datepicker::Local>> for Timestamp {
    //     fn from(dt: egui_datepicker::Date<egui_datepicker::Local>) -> Self {
    //         dt.and_time(egui_datepicker::Local::now().time())
    //             .expect("unable to convert egui_datepicker::Date<Local> to timestamp")
    //             .naive_utc()
    //             .timestamp_millis()
    //             .into()
    //     }
    // }

    impl From<chrono::Date<chrono::Utc>> for Timestamp {
        fn from(date: chrono::Date<chrono::Utc>) -> Self {
            date.and_time(chrono::Utc::now().time())
                .expect("Unable to convert date to timestamp")
                .timestamp_millis()
                .into()
        }
    }

    impl From<chrono::Date<chrono::Local>> for Timestamp {
        fn from(date: chrono::Date<chrono::Local>) -> Self {
            date.and_time(chrono::Local::now().time())
                .expect("Unable to convert date to timestamp")
                .naive_utc()
                .timestamp_millis()
                .into()
        }
    }
}

mod date {
    use chrono::Datelike;

    mod weekday {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Deserialize,
            serde::Serialize,
        )]
        #[repr(u8)]
        pub enum Weekday {
            Sunday = 0,
            Monday = 1,
            Tuesday = 2,
            Wednesday = 3,
            Thursday = 4,
            Friday = 5,
            Saturday = 6,
        }

        impl Weekday {
            pub fn from_u8_unchecked(n: impl Into<u8>) -> Self {
                let n = n.into();
                n.try_into().unwrap()
            }
        }

        impl From<Weekday> for u8 {
            fn from(value: Weekday) -> Self {
                value as u8
            }
        }

        impl TryFrom<u8> for Weekday {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Self::Sunday),
                    1 => Ok(Self::Monday),
                    2 => Ok(Self::Tuesday),
                    3 => Ok(Self::Wednesday),
                    4 => Ok(Self::Thursday),
                    5 => Ok(Self::Friday),
                    6 => Ok(Self::Saturday),
                    _ => Err(()),
                }
            }
        }

        impl From<Weekday> for time::Weekday {
            fn from(weekday: Weekday) -> Self {
                match weekday {
                    Weekday::Sunday => Self::Sunday,
                    Weekday::Monday => Self::Monday,
                    Weekday::Tuesday => Self::Tuesday,
                    Weekday::Wednesday => Self::Wednesday,
                    Weekday::Thursday => Self::Thursday,
                    Weekday::Friday => Self::Friday,
                    Weekday::Saturday => Self::Saturday,
                }
            }
        }
        impl From<time::Weekday> for Weekday {
            fn from(day: time::Weekday) -> Self {
                match day {
                    time::Weekday::Sunday => Self::Sunday,
                    time::Weekday::Monday => Self::Monday,
                    time::Weekday::Tuesday => Self::Tuesday,
                    time::Weekday::Wednesday => Self::Wednesday,
                    time::Weekday::Thursday => Self::Thursday,
                    time::Weekday::Friday => Self::Friday,
                    time::Weekday::Saturday => Self::Saturday,
                }
            }
        }

        impl From<Weekday> for chrono::Weekday {
            fn from(weekday: Weekday) -> Self {
                match weekday {
                    Weekday::Sunday => Self::Sun,
                    Weekday::Monday => Self::Mon,
                    Weekday::Tuesday => Self::Tue,
                    Weekday::Wednesday => Self::Wed,
                    Weekday::Thursday => Self::Thu,
                    Weekday::Friday => Self::Fri,
                    Weekday::Saturday => Self::Sat,
                }
            }
        }
        impl From<chrono::Weekday> for Weekday {
            fn from(day: chrono::Weekday) -> Self {
                match day {
                    chrono::Weekday::Sun => Self::Sunday,
                    chrono::Weekday::Mon => Self::Monday,
                    chrono::Weekday::Tue => Self::Tuesday,
                    chrono::Weekday::Wed => Self::Wednesday,
                    chrono::Weekday::Thu => Self::Thursday,
                    chrono::Weekday::Fri => Self::Friday,
                    chrono::Weekday::Sat => Self::Saturday,
                }
            }
        }
    }

    mod month {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Deserialize,
            serde::Serialize,
        )]
        #[repr(u8)]
        pub enum Month {
            January = 1,
            February = 2,
            March = 3,
            April = 4,
            May = 5,
            June = 6,
            July = 7,
            August = 8,
            September = 9,
            October = 10,
            November = 11,
            December = 12,
        }

        impl Month {
            pub fn from_u8_unchecked(n: impl Into<u8>) -> Self {
                let n = n.into();
                n.try_into().unwrap()
            }
        }

        impl From<Month> for u8 {
            fn from(value: Month) -> Self {
                value as u8
            }
        }

        // impl From<u8> for Month {
        //     fn from(t: u8) -> Self {
        //         match t {
        //             1 => Self::January,
        //             2 => Self::February,
        //             3 => Self::March,
        //             4 => Self::April,
        //             5 => Self::May,
        //             6 => Self::June,
        //             7 => Self::July,
        //             8 => Self::August,
        //             9 => Self::September,
        //             10 => Self::October,
        //             11 => Self::November,
        //             12 => Self::December,
        //             _ => panic!("Invalid month"),
        //         }
        //     }
        // }

        impl TryFrom<u8> for Month {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    1 => Ok(Self::January),
                    2 => Ok(Self::February),
                    3 => Ok(Self::March),
                    4 => Ok(Self::April),
                    5 => Ok(Self::May),
                    6 => Ok(Self::June),
                    7 => Ok(Self::July),
                    8 => Ok(Self::August),
                    9 => Ok(Self::September),
                    10 => Ok(Self::October),
                    11 => Ok(Self::November),
                    12 => Ok(Self::December),
                    _ => Err(()),
                }
            }
        }

        impl From<Month> for time::Month {
            fn from(month: Month) -> Self {
                match month {
                    Month::January => time::Month::January,
                    Month::February => time::Month::February,
                    Month::March => time::Month::March,
                    Month::April => time::Month::April,
                    Month::May => time::Month::May,
                    Month::June => time::Month::June,
                    Month::July => time::Month::July,
                    Month::August => time::Month::August,
                    Month::September => time::Month::September,
                    Month::October => time::Month::October,
                    Month::November => time::Month::November,
                    Month::December => time::Month::December,
                }
            }
        }

        impl From<time::Month> for Month {
            fn from(month: time::Month) -> Self {
                match month {
                    time::Month::January => Self::January,
                    time::Month::February => Self::February,
                    time::Month::March => Self::March,
                    time::Month::April => Self::April,
                    time::Month::May => Self::May,
                    time::Month::June => Self::June,
                    time::Month::July => Self::July,
                    time::Month::August => Self::August,
                    time::Month::September => Self::September,
                    time::Month::October => Self::October,
                    time::Month::November => Self::November,
                    time::Month::December => Self::December,
                }
            }
        }

        impl From<Month> for chrono::Month {
            fn from(month: Month) -> Self {
                match month {
                    Month::January => chrono::Month::January,
                    Month::February => chrono::Month::February,
                    Month::March => chrono::Month::March,
                    Month::April => chrono::Month::April,
                    Month::May => chrono::Month::May,
                    Month::June => chrono::Month::June,
                    Month::July => chrono::Month::July,
                    Month::August => chrono::Month::August,
                    Month::September => chrono::Month::September,
                    Month::October => chrono::Month::October,
                    Month::November => chrono::Month::November,
                    Month::December => chrono::Month::December,
                }
            }
        }

        impl From<chrono::Month> for Month {
            fn from(month: chrono::Month) -> Self {
                match month {
                    chrono::Month::January => Self::January,
                    chrono::Month::February => Self::February,
                    chrono::Month::March => Self::March,
                    chrono::Month::April => Self::April,
                    chrono::Month::May => Self::May,
                    chrono::Month::June => Self::June,
                    chrono::Month::July => Self::July,
                    chrono::Month::August => Self::August,
                    chrono::Month::September => Self::September,
                    chrono::Month::October => Self::October,
                    chrono::Month::November => Self::November,
                    chrono::Month::December => Self::December,
                }
            }
        }
    }

    mod period {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Deserialize,
            serde::Serialize,
        )]
        #[repr(u8)]
        pub enum TimePeriod {
            Am = 0,
            Pm = 1,
        }
    }

    mod hour {
        use super::TimePeriod;

        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            serde::Deserialize,
            serde::Serialize,
        )]
        #[repr(u8)]
        pub enum Hour {
            Zero = 0,
            One = 1,
            Two = 2,
            Three = 3,
            Four = 4,
            Five = 5,
            Six = 6,
            Seven = 7,
            Eight = 8,
            Nine = 9,
            Ten = 10,
            Eleven = 11,
            Twelve = 12,
            Thirteen = 13,
            Fourteen = 14,
            Fifteen = 15,
            Sixteen = 16,
            Seventeen = 17,
            Eighteen = 18,
            Nineteen = 19,
            Twenty = 20,
            TwentyOne = 21,
            TwentyTwo = 22,
            TwentyThree = 23,
        }

        impl Hour {
            pub fn from_u8_unchecked(n: impl Into<u8>) -> Self {
                let n = n.into();
                n.try_into().unwrap()
            }

            pub fn to_u8(self) -> u8 {
                self as u8
            }

            pub fn morning() -> Self {
                Self::Eight
            }

            pub fn afternoon() -> Self {
                Self::Fourteen
            }

            pub fn noon() -> Self {
                Self::Twelve
            }

            pub fn midnight() -> Self {
                Self::Zero
            }

            pub fn evening() -> Self {
                Self::Twenty
            }

            pub fn twelve_hour(self) -> u8 {
                self.standard().0
            }

            pub fn twenty_four_hour(self) -> u8 {
                self as u8
            }

            pub fn standard(self) -> (u8, TimePeriod) {
                match self {
                    Self::Zero => (12, TimePeriod::Am),
                    Self::One => (1, TimePeriod::Am),
                    Self::Two => (2, TimePeriod::Am),
                    Self::Three => (3, TimePeriod::Am),
                    Self::Four => (4, TimePeriod::Am),
                    Self::Five => (5, TimePeriod::Am),
                    Self::Six => (6, TimePeriod::Am),
                    Self::Seven => (7, TimePeriod::Am),
                    Self::Eight => (8, TimePeriod::Am),
                    Self::Nine => (9, TimePeriod::Am),
                    Self::Ten => (10, TimePeriod::Am),
                    Self::Eleven => (11, TimePeriod::Am),
                    Self::Twelve => (12, TimePeriod::Pm),
                    Self::Thirteen => (1, TimePeriod::Pm),
                    Self::Fourteen => (2, TimePeriod::Pm),
                    Self::Fifteen => (3, TimePeriod::Pm),
                    Self::Sixteen => (4, TimePeriod::Pm),
                    Self::Seventeen => (5, TimePeriod::Pm),
                    Self::Eighteen => (6, TimePeriod::Pm),
                    Self::Nineteen => (7, TimePeriod::Pm),
                    Self::Twenty => (8, TimePeriod::Pm),
                    Self::TwentyOne => (9, TimePeriod::Pm),
                    Self::TwentyTwo => (10, TimePeriod::Pm),
                    Self::TwentyThree => (11, TimePeriod::Pm),
                }
            }
            pub fn military(self) -> (u8, TimePeriod) {
                let n = self.to_u8();
                if n < 12 {
                    (n, TimePeriod::Am)
                } else {
                    (n - 12, TimePeriod::Pm)
                }
            }
        }

        // impl From<u8> for Hour {
        //     fn from(t: u8) -> Self {
        //         match t {
        //             0 => Self::Zero,
        //             1 => Self::One,
        //             2 => Self::Two,
        //             3 => Self::Three,
        //             4 => Self::Four,
        //             5 => Self::Five,
        //             6 => Self::Six,
        //             7 => Self::Seven,
        //             8 => Self::Eight,
        //             9 => Self::Nine,
        //             10 => Self::Ten,
        //             11 => Self::Eleven,
        //             12 => Self::Twelve,
        //             13 => Self::Thirteen,
        //             14 => Self::Fourteen,
        //             15 => Self::Fifteen,
        //             16 => Self::Sixteen,
        //             17 => Self::Seventeen,
        //             18 => Self::Eighteen,
        //             19 => Self::Nineteen,
        //             20 => Self::Twenty,
        //             21 => Self::TwentyOne,
        //             22 => Self::TwentyTwo,
        //             23 => Self::TwentyThree,
        //             _ => panic!("Invalid hour"),
        //         }
        //     }
        // }

        impl TryFrom<u8> for Hour {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Self::Zero),
                    1 => Ok(Self::One),
                    2 => Ok(Self::Two),
                    3 => Ok(Self::Three),
                    4 => Ok(Self::Four),
                    5 => Ok(Self::Five),
                    6 => Ok(Self::Six),
                    7 => Ok(Self::Seven),
                    8 => Ok(Self::Eight),
                    9 => Ok(Self::Nine),
                    10 => Ok(Self::Ten),
                    11 => Ok(Self::Eleven),
                    12 => Ok(Self::Twelve),
                    13 => Ok(Self::Thirteen),
                    14 => Ok(Self::Fourteen),
                    15 => Ok(Self::Fifteen),
                    16 => Ok(Self::Sixteen),
                    17 => Ok(Self::Seventeen),
                    18 => Ok(Self::Eighteen),
                    19 => Ok(Self::Nineteen),
                    20 => Ok(Self::Twenty),
                    21 => Ok(Self::TwentyOne),
                    22 => Ok(Self::TwentyTwo),
                    23 => Ok(Self::TwentyThree),
                    _ => Err(()),
                }
            }
        }

        impl From<Hour> for u8 {
            fn from(value: Hour) -> Self {
                value as u8
            }
        }

        impl std::ops::Add<u8> for Hour {
            type Output = Self;

            #[allow(
                clippy::suspicious_arithmetic_impl,
                reason = "we wrap addition when it comes to hours"
            )]
            fn add(self, rhs: u8) -> Self::Output {
                let total = (self.to_u8() as u16).saturating_add(rhs as u16);
                let total = total % 24;
                Hour::from_u8_unchecked(total as u8)
            }
        }
        impl std::ops::AddAssign<u8> for Hour {
            fn add_assign(&mut self, rhs: u8) {
                *self = *self + rhs;
            }
        }

        impl std::ops::Sub<u8> for Hour {
            type Output = Self;

            #[allow(
                clippy::suspicious_arithmetic_impl,
                reason = "we wrap addition when it comes to hours"
            )]
            fn sub(self, rhs: u8) -> Self::Output {
                let total = (self.to_u8() as u16).wrapping_sub(rhs as u16);
                let total = (total % 24) as u8;
                Hour::from_u8_unchecked(total)
            }
        }
        impl std::ops::SubAssign<u8> for Hour {
            fn sub_assign(&mut self, rhs: u8) {
                *self = *self - rhs;
            }
        }

        impl std::ops::Add<Self> for Hour {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                self + rhs as u8
            }
        }
        impl std::ops::AddAssign<Self> for Hour {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl std::ops::Sub<Self> for Hour {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self - rhs as u8
            }
        }
        impl std::ops::SubAssign<Self> for Hour {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
    }

    pub use hour::Hour;
    pub use month::Month;
    pub use period::TimePeriod;
    pub use weekday::Weekday;

    pub struct Date(time::Date);

    impl Date {
        pub fn new(year: i32, month: u8, day: u8) -> Self {
            let month = Month::from_u8_unchecked(month);
            Self(
                time::Date::from_calendar_date(year, month.into(), day)
                    .expect("Unable to create Date from calendar date"),
            )
        }

        pub fn from_calendar_date(year: i32, month: impl Into<Month>, day: u8) -> Self {
            let month = month.into();
            Self(
                time::Date::from_calendar_date(year, month.into(), day)
                    .expect("Unable to create Date from calendar date"),
            )
        }

        pub fn from_iso_week_date(year: i32, week: u8, weekday: impl Into<Weekday>) -> Self {
            let weekday = weekday.into();
            Self(
                time::Date::from_iso_week_date(year, week, weekday.into())
                    .expect("Unable to create Date from ISO week date"),
            )
        }

        pub fn to_calendar_date(&self) -> (i32, Month, u8) {
            let (y, m, d) = self.0.to_calendar_date();
            (y, m.into(), d)
        }

        pub fn to_ymd(&self) -> (i32, Month, u8) {
            self.to_calendar_date()
        }

        pub fn to_iso_week_date(&self) -> (i32, u8, Weekday) {
            let (y, w, wd) = self.0.to_iso_week_date();
            (y, w, wd.into())
        }

        pub fn to_ordinal_date(&self) -> u16 {
            self.0.ordinal()
        }

        pub fn year(&self) -> i32 {
            self.0.year()
        }

        pub fn month(&self) -> Month {
            self.0.month().into()
        }

        pub fn day(&self) -> u8 {
            self.0.day()
        }

        pub fn weekday(&self) -> Weekday {
            self.0.weekday().into()
        }

        pub fn to_chrono(&self) -> chrono::NaiveDate {
            let (y, w, wd) = self.to_iso_week_date();
            chrono::NaiveDate::from_isoywd(y, w.into(), wd.into())
        }

        pub fn modify_as_chrono(&mut self, f: impl FnOnce(&mut chrono::Date<chrono::Local>)) {
            let mut chrono_date = self.to_chrono_local();
            f(&mut chrono_date);
            *self = chrono_date.into();
        }

        pub fn to_chrono_utc(&self) -> chrono::Date<chrono::Utc> {
            chrono::Date::from_utc(self.to_chrono(), chrono::Utc)
        }

        pub fn to_chrono_local(&self) -> chrono::Date<chrono::Local> {
            self.to_chrono_utc().with_timezone(&chrono::Local)
        }

        pub fn to_timelib(&self) -> time::Date {
            self.0
        }

        pub fn modify_as_timelib(&mut self, f: impl FnOnce(&mut time::Date)) {
            f(&mut self.0);
        }
    }

    impl From<time::Date> for Date {
        fn from(date: time::Date) -> Self {
            Self(date)
        }
    }

    impl From<chrono::NaiveDate> for Date {
        fn from(date: chrono::NaiveDate) -> Self {
            let weekday: Weekday = date.weekday().into();
            let iso = date.iso_week();
            let year = iso.year();
            let week = iso.week();
            #[allow(
                clippy::cast_possible_truncation,
                reason = "the week returned by chrono is always a valid u8"
            )]
            let time_date = time::Date::from_iso_week_date(year, week as _, weekday.into())
                .expect("unable to build date from calendar date components");
            Self(time_date)
        }
    }

    impl From<Date> for chrono::NaiveDate {
        fn from(date: Date) -> Self {
            let (y, m, d) = date.to_ymd();
            let m: u8 = m.into();
            chrono::NaiveDate::from_ymd(y, m as _, d as _)
        }
    }

    impl From<chrono::Date<chrono::Utc>> for Date {
        fn from(date: chrono::Date<chrono::Utc>) -> Self {
            date.naive_local().into()
        }
    }

    impl From<chrono::Date<chrono::Local>> for Date {
        fn from(date: chrono::Date<chrono::Local>) -> Self {
            date.naive_local().into()
        }
    }
}

pub use date::Date;
pub use timestamp::Timestamp;
