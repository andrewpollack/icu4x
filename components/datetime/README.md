# icu_datetime [![crates.io](https://img.shields.io/crates/v/icu_datetime)](https://crates.io/crates/icu_datetime)

`icu_datetime` is one of the [`ICU4X`] components.

This API provides necessary functionality for formatting date and time to user readable textual representation.

[`DateTimeFormat`] is the main structure of the component. It accepts a set of arguments which
allow it to collect necessary data from the [data provider], and once instantiated, can be
used to quickly format any date and time provided.

## Examples

```rust
use icu::locid::locale;
use icu::calendar::Gregorian;
use icu::datetime::{DateTimeFormat, DateTimeFormatOptions, mock::parse_gregorian_from_str, options::length};

let provider = icu_testdata::get_provider();

// See the next code example for a more ergonomic example with .into().
let options = DateTimeFormatOptions::Length(length::Bag {
    date: Some(length::Date::Medium),
    time: Some(length::Time::Short),
    ..Default::default()
});

let dtf = DateTimeFormat::<Gregorian>::try_new(locale!("en"), &provider, &options)
    .expect("Failed to create DateTimeFormat instance.");


let date = parse_gregorian_from_str("2020-09-12T12:35:00")
    .expect("Failed to parse date.");

let formatted_date = dtf.format(&date);
assert_eq!(formatted_date.to_string(), "Sep 12, 2020, 12:35 PM");
```

The options can be created more ergonomically using the `Into` trait to automatically
convert a [`options::length::Bag`] into a [`DateTimeFormatOptions::Length`].

```rust
use icu::calendar::Gregorian;
use icu::datetime::{DateTimeFormat, DateTimeFormatOptions, options::length};
let options = length::Bag {
    date: Some(length::Date::Medium),
    time: Some(length::Time::Short),
    ..Default::default()
}.into();

let dtf = DateTimeFormat::<Gregorian>::try_new(locale, &provider, &options);
```

At the moment, the crate provides only options using the [`Length`] bag, but in the future,
we expect to add more ways to customize the output, like skeletons, and components.

*Notice:* Rust at the moment does not have a canonical way to represent date and time. We use
[`DateTime`] as an example of the data necessary for ICU [`DateTimeFormat`] to work, and
[we hope to work with the community](https://github.com/unicode-org/icu4x/blob/main/docs/research/datetime.md)
to develop core date and time APIs that will work as an input for this component. [`DateTime`] additionally
has support for non-Gregorian calendars, which this module will eventually be able to format.

[data provider]: icu_provider
[`ICU4X`]: ../icu/index.html
[`Length`]: options::length
[`DateTime`]: icu_calendar::DateTime

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
