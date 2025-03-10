use chrono::{Days, NaiveDate};
use rust_decimal::Decimal;

use crate::{
    charts::chart_properties_portrait::imports::ResolutionKind,
    impl_into_string_timespan_value,
    types::{Timespan, TimespanDuration, TimespanValue},
    utils::day_start,
};

pub type DateValue<V> = TimespanValue<NaiveDate, V>;

impl Timespan for NaiveDate {
    fn from_date(date: NaiveDate) -> Self {
        date
    }

    fn into_date(self) -> NaiveDate {
        self
    }

    fn saturating_next_timespan(&self) -> Self {
        self.checked_add_days(Days::new(1))
            .unwrap_or(NaiveDate::MAX)
    }

    fn saturating_previous_timespan(&self) -> Self {
        self.checked_sub_days(Days::new(1))
            .unwrap_or(NaiveDate::MIN)
    }

    fn enum_variant() -> ResolutionKind {
        ResolutionKind::Day
    }

    fn saturating_start_timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        day_start(self)
    }

    fn checked_add(&self, duration: TimespanDuration<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_add_days(Days::new(duration.repeats()))
    }

    fn checked_sub(&self, duration: TimespanDuration<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_sub_days(Days::new(duration.repeats()))
    }

    fn max() -> Self {
        NaiveDate::MAX
    }

    fn min() -> Self {
        NaiveDate::MIN
    }
}

impl_into_string_timespan_value!(NaiveDate, i64);
impl_into_string_timespan_value!(NaiveDate, f64);
impl_into_string_timespan_value!(NaiveDate, Decimal);
