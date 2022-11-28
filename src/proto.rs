use crate::proto::acorn::Date;
use chrono::NaiveDate;

pub mod acorn;

impl Into<NaiveDate> for Date {
    fn into(self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year as i32, self.month, self.day).unwrap()
    }
}
