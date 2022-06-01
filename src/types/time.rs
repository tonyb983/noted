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

        impl TimePeriod {
            pub fn is_am(self) -> bool {
                self == Self::Am
            }

            pub fn is_pm(self) -> bool {
                self == Self::Pm
            }

            pub fn toggle(&mut self) {
                *self = match self {
                    Self::Am => Self::Pm,
                    Self::Pm => Self::Am,
                };
            }
        }

        impl std::fmt::Display for TimePeriod {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Am => write!(f, "AM"),
                    Self::Pm => write!(f, "PM"),
                }
            }
        }
    }

    mod hour {
        use crate::{types::time::reminder_time::Hour12, util::wrapping::WrappedU8};

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
        pub struct Hour(WrappedU8<0, 23>);

        impl Hour {
            #[must_use]
            pub fn from_time(t: time::Time) -> Self {
                Self(WrappedU8::from(t.hour()))
            }

            #[must_use]
            pub fn from_chrono<T: chrono::Timelike>(c: &T) -> Self {
                Self(WrappedU8::from(c.hour()))
            }

            #[must_use]
            pub fn value(&self) -> u8 {
                self.0.value()
            }

            #[must_use]
            pub const fn from_u8(n: impl Into<u8>) -> Self {
                let n: u8 = n.into();
                Self(n.into())
            }

            #[must_use]
            pub fn to_u8(self) -> u8 {
                self.value()
            }

            #[must_use]
            pub fn morning() -> Self {
                Self(8u8.into())
            }

            #[must_use]
            pub fn afternoon() -> Self {
                Self(14u8.into())
            }

            #[must_use]
            pub fn noon() -> Self {
                Self(12u8.into())
            }

            #[must_use]
            pub fn midnight() -> Self {
                Self(0u8.into())
            }

            #[must_use]
            pub fn evening() -> Self {
                Self(20u8.into())
            }

            #[must_use]
            pub fn twelve_hour(self) -> Hour12 {
                self.standard().0
            }

            #[must_use]
            pub fn twenty_four_hour(self) -> u8 {
                self.value()
            }

            #[must_use]
            pub fn standard(self) -> (Hour12, TimePeriod) {
                let n = self.value();
                if n < 12 {
                    (n.into(), TimePeriod::Am)
                } else {
                    ((n - 12).into(), TimePeriod::Pm)
                }
            }

            #[must_use]
            pub fn military(self) -> (u8, TimePeriod) {
                let n = self.value();
                if n < 12 {
                    (n, TimePeriod::Am)
                } else {
                    (n, TimePeriod::Pm)
                }
            }
        }

        impl From<u8> for Hour {
            fn from(t: u8) -> Self {
                Self(t.into())
            }
        }

        impl From<u32> for Hour {
            fn from(t: u32) -> Self {
                Self(t.into())
            }
        }

        impl From<Hour> for u8 {
            fn from(value: Hour) -> Self {
                value.value()
            }
        }

        impl std::str::FromStr for Hour {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let n = s.parse::<u8>().map_err(|_| ())?;
                Ok(Self(n.into()))
            }
        }

        impl std::ops::Add<u8> for Hour {
            type Output = Self;

            #[allow(
                clippy::suspicious_arithmetic_impl,
                reason = "we wrap addition when it comes to hours"
            )]
            fn add(self, rhs: u8) -> Self::Output {
                Self((self.value() + rhs).into())
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
                clippy::cast_possible_wrap,
                reason = "u8 -> isize HAS to be safe right?"
            )]
            fn sub(self, rhs: u8) -> Self::Output {
                Self(WrappedU8::from_any_signed(
                    self.value() as isize - rhs as isize,
                ))
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
                Self(self.0 + rhs.0)
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
                Self(self.0 - rhs.0)
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
    pub struct Date(time::Date);

    impl Date {
        #[must_use]
        pub fn new(year: i32, month: u8, day: u8) -> Self {
            let month = Month::from_u8_unchecked(month);
            Self(
                time::Date::from_calendar_date(year, month.into(), day)
                    .expect("Unable to create Date from calendar date"),
            )
        }

        #[must_use]
        pub fn from_calendar_date(year: i32, month: impl Into<Month>, day: u8) -> Self {
            let month = month.into();
            Self(
                time::Date::from_calendar_date(year, month.into(), day)
                    .expect("Unable to create Date from calendar date"),
            )
        }

        #[must_use]
        pub fn from_iso_week_date(year: i32, week: u8, weekday: impl Into<Weekday>) -> Self {
            let weekday = weekday.into();
            Self(
                time::Date::from_iso_week_date(year, week, weekday.into())
                    .expect("Unable to create Date from ISO week date"),
            )
        }

        #[must_use]
        pub fn to_calendar_date(&self) -> (i32, Month, u8) {
            let (y, m, d) = self.0.to_calendar_date();
            (y, m.into(), d)
        }

        #[must_use]
        pub fn to_ymd(&self) -> (i32, Month, u8) {
            self.to_calendar_date()
        }

        #[must_use]
        pub fn to_iso_week_date(&self) -> (i32, u8, Weekday) {
            let (y, w, wd) = self.0.to_iso_week_date();
            (y, w, wd.into())
        }

        #[must_use]
        pub fn to_ordinal_date(&self) -> u16 {
            self.0.ordinal()
        }

        #[must_use]
        pub fn year(&self) -> i32 {
            self.0.year()
        }

        #[must_use]
        pub fn month(&self) -> Month {
            self.0.month().into()
        }

        #[must_use]
        pub fn day(&self) -> u8 {
            self.0.day()
        }

        #[must_use]
        pub fn weekday(&self) -> Weekday {
            self.0.weekday().into()
        }

        #[must_use]
        pub fn to_chrono(&self) -> chrono::NaiveDate {
            let (y, w, wd) = self.to_iso_week_date();
            chrono::NaiveDate::from_isoywd(y, w.into(), wd.into())
        }

        pub fn modify_as_chrono(&mut self, f: impl FnOnce(&mut chrono::Date<chrono::Local>)) {
            let mut chrono_date = self.to_chrono_local();
            f(&mut chrono_date);
            *self = chrono_date.into();
        }

        #[must_use]
        pub fn to_chrono_utc(&self) -> chrono::Date<chrono::Utc> {
            chrono::Date::from_utc(self.to_chrono(), chrono::Utc)
        }

        #[must_use]
        pub fn to_chrono_local(&self) -> chrono::Date<chrono::Local> {
            self.to_chrono_utc().with_timezone(&chrono::Local)
        }

        #[must_use]
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

mod min {
    use crate::util::wrapping::WrappedU8;

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
    pub struct Minute(WrappedU8<0, 59>);

    impl Minute {
        #[must_use]
        pub const fn from_u8(minute: u8) -> Self {
            Self(WrappedU8::new(minute))
        }

        #[must_use]
        pub fn value(&self) -> u8 {
            self.0.value()
        }

        #[must_use]
        pub fn from_1_to_60(minute: u8) -> Self {
            Self::from_u8(minute.saturating_sub(1))
        }

        #[must_use]
        pub fn from_chrono<T: chrono::Timelike>(ct: &T) -> Self {
            Self(ct.minute().into())
        }

        #[must_use]
        pub fn from_time(tt: time::Time) -> Self {
            Self(tt.minute().into())
        }
    }

    impl From<u32> for Minute {
        fn from(minute: u32) -> Self {
            Self(WrappedU8::from_any_unsigned(minute as usize))
        }
    }

    impl std::str::FromStr for Minute {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let n = s.parse::<u8>().map_err(|_| ())?;
            Ok(Self(n.into()))
        }
    }

    impl std::ops::Add<u8> for Minute {
        type Output = Self;

        fn add(self, rhs: u8) -> Self::Output {
            Self(self.0.add(rhs))
        }
    }

    impl std::ops::AddAssign<u8> for Minute {
        fn add_assign(&mut self, rhs: u8) {
            self.0.add_assign(rhs);
        }
    }

    impl std::ops::Sub<u8> for Minute {
        type Output = Self;

        fn sub(self, rhs: u8) -> Self {
            Self(self.0.sub(rhs))
        }
    }

    impl std::ops::SubAssign<u8> for Minute {
        fn sub_assign(&mut self, rhs: u8) {
            self.0.sub_assign(rhs);
        }
    }

    impl From<Minute> for u8 {
        fn from(minute: Minute) -> Self {
            minute.value()
        }
    }

    impl From<u8> for Minute {
        fn from(minute: u8) -> Self {
            Self::from_u8(minute)
        }
    }
}

mod reminder_time {
    use crate::{types::time::date::TimePeriod, util::wrapping::WrappedU8};

    use super::{date::Hour, Date, Minute};

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
    pub struct Hour12(WrappedU8<0, 11>);

    impl Hour12 {
        #[must_use]
        pub const fn from_u8(hour: u8) -> Self {
            Self(WrappedU8::new(hour))
        }

        #[must_use]
        pub fn value(self) -> u8 {
            self.0.value()
        }

        #[must_use]
        pub fn from_chrono<T: chrono::Timelike>(ct: &T) -> Self {
            Self(ct.hour().into())
        }

        #[must_use]
        pub fn from_time(tt: time::Time) -> Self {
            Self(tt.hour().into())
        }
    }

    impl From<u8> for Hour12 {
        fn from(hour: u8) -> Self {
            Self(WrappedU8::new(hour))
        }
    }

    impl From<Hour12> for u8 {
        fn from(hour: Hour12) -> Self {
            hour.value()
        }
    }

    impl From<Hour> for (Hour12, TimePeriod) {
        fn from(hour: Hour) -> Self {
            hour.standard()
        }
    }

    impl From<(Hour12, TimePeriod)> for Hour {
        fn from(hour: (Hour12, TimePeriod)) -> Self {
            match hour.1 {
                TimePeriod::Am => hour.0.value().into(),
                TimePeriod::Pm => (hour.0.value() + 12).into(),
            }
        }
    }

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
    pub struct SimpleTime {
        pub hour: Hour12,
        pub minute: Minute,
        pub period: TimePeriod,
    }

    impl SimpleTime {
        pub fn new(hour: Hour12, minute: Minute, period: TimePeriod) -> Self {
            Self {
                hour,
                minute,
                period,
            }
        }

        pub fn from_military(hour: Hour, minute: Minute) -> Self {
            let (hr, period) = hour.standard();
            Self::new(hr, minute, period)
        }

        pub fn to_military(self) -> (Hour, Minute) {
            (
                match self.period {
                    TimePeriod::Am => self.hour.value().into(),
                    TimePeriod::Pm => (self.hour.value() + 12).into(),
                },
                self.minute,
            )
        }

        pub fn to_hmp(self) -> (Hour12, Minute, TimePeriod) {
            (self.hour, self.minute, self.period)
        }

        pub fn to_time(self) -> time::Time {
            let (hour, minute) = self.to_military();
            time::Time::from_hms(hour.value(), minute.value(), 0)
                .expect("unable to build time::Time from hms")
        }

        pub fn to_chrono(self) -> chrono::NaiveTime {
            let (hour, minute) = self.to_military();
            chrono::NaiveTime::from_hms(hour.value().into(), minute.value().into(), 0)
        }
    }

    impl std::fmt::Display for Hour12 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value())
        }
    }

    impl std::fmt::Display for SimpleTime {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{:02}:{:02} {}",
                self.hour.value(),
                self.minute.value(),
                self.period
            )
        }
    }

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
    pub struct ReminderTime {
        pub date: Date,
        pub time: SimpleTime,
    }

    impl ReminderTime {
        #[must_use]
        pub fn epoch() -> Self {
            Self::from_time_dt(time::OffsetDateTime::UNIX_EPOCH)
        }

        #[must_use]
        pub fn to_chrono_date_utc(&self) -> chrono::Date<chrono::Utc> {
            self.date.to_chrono_utc()
        }

        #[must_use]
        pub fn to_chrono_date_local(&self) -> chrono::Date<chrono::Local> {
            self.to_chrono_dt_local().date()
        }

        #[must_use]
        pub fn to_chrono_time(&self) -> chrono::NaiveTime {
            let (hour, minute) = self.time.to_military();
            chrono::NaiveTime::from_hms(hour.value().into(), minute.value().into(), 0)
        }

        #[must_use]
        pub fn to_time_date(&self) -> time::Date {
            self.date.to_timelib()
        }

        #[must_use]
        pub fn to_time_time(&self) -> time::Time {
            let (hour, minute) = self.time.to_military();
            time::Time::from_hms(hour.value(), minute.value(), 0)
                .expect("unable to build time from hours and minutes")
        }

        #[must_use]
        pub fn to_chrono_dt(&self) -> chrono::DateTime<chrono::Utc> {
            self.to_chrono_date_utc()
                .and_time(self.to_chrono_time())
                .expect("unable to build chrono datetime")
        }

        #[must_use]
        pub fn to_chrono_dt_local(&self) -> chrono::DateTime<chrono::Local> {
            self.to_chrono_dt().with_timezone(&chrono::Local)
        }

        #[must_use]
        pub fn to_time_dt(&self) -> time::OffsetDateTime {
            self.to_time_date()
                .with_time(self.to_time_time())
                .assume_utc()
        }

        #[must_use]
        pub fn to_time_dt_local(&self) -> time::OffsetDateTime {
            self.to_time_dt().to_offset(
                time::UtcOffset::current_local_offset()
                    .expect("unable to get current local offset"),
            )
        }

        #[must_use]
        pub fn from_time_dt(mut dt: time::OffsetDateTime) -> Self {
            crate::util::dtf::ensure_time_is_utc(&mut dt);
            let date: Date = dt.date().into();
            let (hour, min): (Hour, Minute) = {
                let time = dt.time();
                let (h, m, _) = time.as_hms();
                (Hour::from_u8(h), Minute::from_u8(m))
            };
            Self {
                date,
                time: SimpleTime::from_military(hour, min),
            }
        }

        pub fn replace_date_t(&mut self, date: time::Date) {
            self.date = date.into();
        }

        pub fn replace_time(&mut self, hour: Hour12, minute: Minute, period: TimePeriod) {
            self.time = SimpleTime::new(hour, minute, period);
        }

        pub fn replace_time_t(&mut self, time: time::Time) {
            let (hour, min, _) = time.as_hms();
            self.time = SimpleTime::from_military(Hour::from_u8(hour), Minute::from_u8(min));
        }

        pub fn replace_time_c<T: chrono::Timelike>(&mut self, t: &T) {
            let hour = t.hour().into();
            let min = t.minute().into();
            self.time = SimpleTime::from_military(hour, min);
        }

        pub fn replace_hour(&mut self, hour: impl Into<Hour12>) {
            self.time.hour = hour.into();
        }

        pub fn replace_mins(&mut self, min: impl Into<Minute>) {
            self.time.minute = min.into();
        }

        pub fn replace_period(&mut self, period: impl Into<TimePeriod>) {
            self.time.period = period.into();
        }
    }
}

pub use date::{Date, Hour, TimePeriod};
pub use min::Minute;
pub use reminder_time::{Hour12, ReminderTime, SimpleTime};
pub use timestamp::Timestamp;

impl std::fmt::Display for Hour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.value())
    }
}

impl std::fmt::Display for Minute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.value())
    }
}
