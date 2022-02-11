use chrono::{DateTime, Datelike, Duration, Utc};
use ic_cdk::export::candid::{CandidType, Deserialize};
use now::DateTimeNow;
use num_traits::ToPrimitive;
use std::time::UNIX_EPOCH;

#[derive(CandidType, Deserialize)]
pub enum DayOfWeek {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

pub trait TimeNanos {
    fn nanos_till_next(&self, weekday: DayOfWeek) -> u64;
    fn to_datetime(&self) -> DateTime<Utc>;
}

impl TimeNanos for u64 {
    fn nanos_till_next(&self, weekday: DayOfWeek) -> u64 {
        let current_datetime = self.to_datetime();
        let duration_since_beginning_of_the_day =
            current_datetime - current_datetime.beginning_of_day();

        let current_weekday_num = current_datetime
            .weekday()
            .number_from_monday()
            .to_i64()
            .unwrap();
        let target_weekday_num = weekday.to_weekday_num().to_i64().unwrap();

        let same_weekday_correction = if target_weekday_num == current_weekday_num {
            7
        } else {
            0
        };

        let days_till_target =
            (same_weekday_correction + target_weekday_num - current_weekday_num).abs();
        let duration_till_target =
            Duration::days(days_till_target) - duration_since_beginning_of_the_day;

        duration_till_target
            .num_nanoseconds()
            .unwrap()
            .to_u64()
            .unwrap()
    }

    fn to_datetime(&self) -> DateTime<Utc> {
        let system_time = UNIX_EPOCH + std::time::Duration::from_nanos(*self);

        DateTime::<Utc>::from(system_time)
    }
}

impl DayOfWeek {
    pub fn to_weekday_num(&self) -> u32 {
        match self {
            DayOfWeek::Mon => 1,
            DayOfWeek::Tue => 2,
            DayOfWeek::Wed => 3,
            DayOfWeek::Thu => 4,
            DayOfWeek::Fri => 5,
            DayOfWeek::Sat => 6,
            DayOfWeek::Sun => 7,
        }
    }
}

pub const NANOS_IN_DAY: u64 = 1_000_000_000 * 60 * 60 * 24;
pub const NANOS_IN_WEEK: u64 = NANOS_IN_DAY * 7;

#[cfg(test)]
mod tests {
    use crate::common::NANOS_IN_DAY;
    use crate::{DayOfWeek, TimeNanos, NANOS_IN_WEEK};
    use chrono::{DateTime, NaiveDate, Utc};
    use num_traits::ToPrimitive;

    fn str_to_datetime(str: &str) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(
            NaiveDate::parse_from_str(str, "%d-%m-%Y")
                .unwrap()
                .and_hms(0, 0, 0),
            Utc,
        )
    }

    #[test]
    fn works_fine() {
        let mon = str_to_datetime("31-01-2022")
            .timestamp_nanos()
            .to_u64()
            .unwrap();

        let tue = str_to_datetime("08-02-2022")
            .timestamp_nanos()
            .to_u64()
            .unwrap();
        let wed = str_to_datetime("16-02-2022")
            .timestamp_nanos()
            .to_u64()
            .unwrap();
        let thu = str_to_datetime("24-02-2022")
            .timestamp_nanos()
            .to_u64()
            .unwrap();
        let fri = str_to_datetime("04-03-2022")
            .timestamp_nanos()
            .to_u64()
            .unwrap();
        let sat = str_to_datetime("12-03-2022")
            .timestamp_nanos()
            .to_u64()
            .unwrap();
        let sun = str_to_datetime("20-03-2022")
            .timestamp_nanos()
            .to_u64()
            .unwrap();

        // checking days consistency
        assert_eq!(
            mon.nanos_till_next(DayOfWeek::Mon),
            NANOS_IN_WEEK,
            "Invalid time dif mon"
        );
        assert_eq!(
            tue.nanos_till_next(DayOfWeek::Tue),
            NANOS_IN_WEEK,
            "Invalid time dif tue"
        );
        assert_eq!(
            wed.nanos_till_next(DayOfWeek::Wed),
            NANOS_IN_WEEK,
            "Invalid time dif wed"
        );
        assert_eq!(
            thu.nanos_till_next(DayOfWeek::Thu),
            NANOS_IN_WEEK,
            "Invalid time dif thu"
        );
        assert_eq!(
            fri.nanos_till_next(DayOfWeek::Fri),
            NANOS_IN_WEEK,
            "Invalid time dif fri"
        );
        assert_eq!(
            sat.nanos_till_next(DayOfWeek::Sat),
            NANOS_IN_WEEK,
            "Invalid time dif sat"
        );
        assert_eq!(
            sun.nanos_till_next(DayOfWeek::Sun),
            NANOS_IN_WEEK,
            "Invalid time dif sun"
        );

        // checking we can reach any day within the next week
        assert_eq!(
            mon.nanos_till_next(DayOfWeek::Tue),
            NANOS_IN_DAY,
            "Invalid time dif mon-tue"
        );
        assert_eq!(
            mon.nanos_till_next(DayOfWeek::Wed),
            NANOS_IN_DAY * 2,
            "Invalid time dif mon-wed"
        );
        assert_eq!(
            mon.nanos_till_next(DayOfWeek::Thu),
            NANOS_IN_DAY * 3,
            "Invalid time dif mon-thu"
        );
        assert_eq!(
            mon.nanos_till_next(DayOfWeek::Fri),
            NANOS_IN_DAY * 4,
            "Invalid time dif mon-fri"
        );
        assert_eq!(
            mon.nanos_till_next(DayOfWeek::Sat),
            NANOS_IN_DAY * 5,
            "Invalid time dif mon-sat"
        );
        assert_eq!(
            mon.nanos_till_next(DayOfWeek::Sun),
            NANOS_IN_DAY * 6,
            "Invalid time dif mon-sun"
        );
    }
}
