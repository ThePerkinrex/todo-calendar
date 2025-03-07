use chrono::{Local, NaiveDate, TimeZone};
use string::parse_string;
use winnow::{
    ModalResult, Parser,
    ascii::{digit1, multispace0},
    combinator::{alt, fail},
    error::StrContext,
};

use crate::data::{DATE_FINAL_TIME, Deadline};

mod string;

fn parse_naive_date(input: &mut &str) -> ModalResult<NaiveDate> {
    (digit1, '/', digit1, '/', digit1)
        .take()
        .try_map(|x| NaiveDate::parse_from_str(x, "%d/%m/%Y"))
        .context(StrContext::Expected(
            winnow::error::StrContextValue::Description("A date"),
        ))
        .parse_next(input)
}

pub fn parse_deadline(input: &mut &str) -> ModalResult<Deadline> {
    multispace0.parse_next(input)?;
    let name = parse_string(input)?;
    multispace0(input)?;
    '@'.parse_next(input)?;
    multispace0(input)?;
    alt((
        parse_naive_date
            .verify_map(|date| {
                Local
                    .from_local_datetime(&date.and_time(DATE_FINAL_TIME))
                    .latest()
                    .map(|x| x.fixed_offset())
            })
            .context(StrContext::Expected(
                winnow::error::StrContextValue::Description("A date"),
            )),
        fail.context(StrContext::Label("valid date or datetime representation")),
    ))
    .parse_next(input)
    .map(|date| Deadline::new(name, &date))
}
