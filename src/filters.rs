// Copyright (c) 2023 Lorenzo Carbonell <a.k.a. atareao>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::convert::TryFrom;

use minijinja::value::{Kwargs, Value, ValueKind};
use minijinja::{Error, ErrorKind, State};
use serde::de::value::SeqDeserializer;
use serde::Deserialize;
use time::format_description::well_known::iso8601::Iso8601;
use time::{format_description, Date, OffsetDateTime};

fn handle_serde_error(err: serde::de::value::Error) -> Error {
    Error::new(ErrorKind::InvalidOperation, "not a valid date or timestamp").with_source(err)
}

#[allow(unused)]
fn value_to_datetime(
    value: Value,
    state: &State,
    kwargs: &Kwargs,
    allow_date: bool,
) -> Result<OffsetDateTime, Error> {
    #[allow(unused_mut)]
    let (mut datetime, had_time) = if let Some(s) = value.as_str() {
        match OffsetDateTime::parse(s, &Iso8601::PARSING) {
            Ok(dt) => (dt, true),
            Err(original_err) => match Date::parse(s, &Iso8601::PARSING) {
                Ok(date) => (date.with_hms(0, 0, 0).unwrap().assume_utc(), false),
                Err(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidOperation,
                        "not a valid date or timestamp",
                    )
                    .with_source(original_err))
                }
            },
        }
    } else if let Ok(v) = f64::try_from(value.clone()) {
        (
            OffsetDateTime::from_unix_timestamp_nanos((v * 1e9) as i128)
                .map_err(|_| Error::new(ErrorKind::InvalidOperation, "date out of range"))?,
            true,
        )
    } else if value.kind() == ValueKind::Seq {
        let mut items = Vec::new();
        for item in value.try_iter()? {
            items.push(i64::try_from(item)?);
        }
        if items.len() == 2 {
            (
                Date::deserialize(SeqDeserializer::new(items.into_iter()))
                    .map_err(handle_serde_error)?
                    .with_hms(0, 0, 0)
                    .unwrap()
                    .assume_utc(),
                false,
            )
        } else {
            (
                OffsetDateTime::deserialize(SeqDeserializer::new(items.into_iter()))
                    .map_err(handle_serde_error)?,
                true,
            )
        }
    } else {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "value is not a datetime",
        ));
    };

    if had_time {
        #[cfg(feature = "timezone")]
        {
            use time_tz::OffsetDateTimeExt;
            let configured_tz = state.lookup("TIMEZONE");
            let tzname = kwargs.get::<Option<&str>>("tz")?.unwrap_or_else(|| {
                configured_tz
                    .as_ref()
                    .and_then(|x| x.as_str())
                    .unwrap_or("original")
            });
            if tzname != "original" {
                let tz = time_tz::timezones::get_by_name(tzname).ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidOperation,
                        format!("unknown timezone '{}'", tzname),
                    )
                })?;
                datetime = datetime.to_timezone(tz)
            };
        }
    } else if !allow_date {
        return Err(Error::new(
            ErrorKind::InvalidOperation,
            "filter requires time, but only received a date",
        ));
    }

    Ok(datetime)
}

/// Formats a timestamp as date and time.
///
/// The value needs to be a unix timestamp, or a parsable string (ISO 8601) or a
/// format supported by `chrono` or `time`.
///
/// The filter accepts two keyword arguments (`format` and `tz`) to influence the format
/// and the timezone.  The default format is `"medium"`.  The defaults for these keyword
/// arguments are taken from two global variables in the template context: `DATETIME_FORMAT`
/// and `TIMEZONE`.  If the timezone is set to `"original"` or is not configured, then
/// the timezone of the value is retained.  Otherwise the timezone is the name of a
/// timezone [from the database](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones).
///
/// ```jinja
/// {{ value|datetimeformat }}
/// ```
///
/// ```jinja
/// {{ value|datetimeformat(format="short") }}
/// ```
///
/// ```jinja
/// {{ value|datetimeformat(format="short", tz="Europe/Vienna") }}
/// ```
///
/// This filter currently uses the `time` crate to format dates and uses the format
/// string specification of that crate in version 2.  For more information read the
/// [Format description documentation](https://time-rs.github.io/book/api/format-description.html).
/// Additionally some special formats are supported:
///
/// * `short`: a short date and time format (`2023-06-24 16:37`)
/// * `medium`: a medium length date and time format (`Jun 24 2023 16:37`)
/// * `long`: a longer date and time format (`June 24 2023 16:37:22`)
/// * `full`: a full date and time format (`Saturday, June 24 2023 16:37:22`)
/// * `unix`: a unix timestamp in seconds only (`1687624642`)
/// * `iso`: date and time in iso format (`2023-06-24T16:37:22+00:00`)
///
/// This filter requires the `datetime` feature, the timezone support requires the `timezone`
/// feature.
pub fn datetimeformat(state: &State, value: Value, kwargs: Kwargs) -> Result<String, Error> {
    let datetime = value_to_datetime(value, state, &kwargs, false)?;
    let configured_format = state.lookup("DATETIME_FORMAT");

    let format = kwargs.get::<Option<&str>>("format")?.unwrap_or_else(|| {
        configured_format
            .as_ref()
            .and_then(|x| x.as_str())
            .unwrap_or("medium")
    });
    kwargs.assert_all_used()?;

    datetime
        .format(
            &format_description::parse_borrowed::<2>(match format {
                "short" => "[year]-[month]-[day] [hour]:[minute]",
                "medium" => "[month repr:short] [day padding:none] [year] [hour]:[minute]",
                "long" => "[month repr:long] [day padding:none] [year] [hour]:[minute]:[second]",
                "full" => "[weekday], [month repr:long] [day padding:none] [year] [hour]:[minute]:[second].[subsecond]",
                "iso" => {
                    "[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"
                }
                "unix" => "[unix_timestamp]",
                other => other,
            })
            .map_err(|err| {
                Error::new(ErrorKind::InvalidOperation, "invalid format string").with_source(err)
            })?,
        )
        .map_err(|err| {
            Error::new(ErrorKind::InvalidOperation, "failed to format date").with_source(err)
        })
}
