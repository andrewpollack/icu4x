// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_datagen::cldr` contains implementations of the [`ICU4X`] [data provider] interface
//! based on the JSON files shipped by CLDR. Create a [`CldrPaths`] and then pass it into
//! [`create_exportable_provider`].
//!
//! This crate contains two implementations of [`CldrPaths`]:
//!
//! - [`CldrPathsLocal`] which points directly to each local CLDR file, and
//! - [`CldrPathsAllInOne`] which points to a local CLDR tree.
//!
//! **Important:** This data provider implementation is not optimized
//! for production use. Read more in the [data provider] docs.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider

mod cldr_paths;
mod cldr_serde;
mod error;
mod reader;
mod transform;

pub use cldr_paths::CldrPaths;
pub use cldr_paths::CldrPathsAllInOne;
pub use cldr_paths::CldrPathsLocal;
pub use error::Error as CldrError;

use icu_provider::datagen::OmnibusDatagenProvider;
use icu_provider::prelude::*;
use icu_provider_adapters::fork::by_key::MultiForkByKeyProvider;
use std::convert::TryFrom;
use std::path::PathBuf;
use transform::calendar::japanese::JapaneseErasProvider;
use transform::datetime::week_data::WeekDataProvider;
use transform::datetime::CommonDateProvider;
use transform::decimal::NumbersProvider;
use transform::list::ListProvider;
use transform::locale_canonicalizer::aliases::AliasesProvider;
use transform::locale_canonicalizer::likely_subtags::LikelySubtagsProvider;
use transform::plurals::PluralsProvider;
use transform::time_zones::TimeZonesProvider;

pub fn create_exportable_provider<T: DataMarker>(
    cldr_paths: &dyn CldrPaths,
    _uprops_root: PathBuf,
) -> Result<MultiForkByKeyProvider<Box<dyn OmnibusDatagenProvider<T> + Sync>>, CldrError>
where
    AliasesProvider: OmnibusDatagenProvider<T>,
    CommonDateProvider: OmnibusDatagenProvider<T>,
    JapaneseErasProvider: OmnibusDatagenProvider<T>,
    LikelySubtagsProvider: OmnibusDatagenProvider<T>,
    NumbersProvider: OmnibusDatagenProvider<T>,
    PluralsProvider: OmnibusDatagenProvider<T>,
    TimeZonesProvider: OmnibusDatagenProvider<T>,
    ListProvider: OmnibusDatagenProvider<T>,
    WeekDataProvider: OmnibusDatagenProvider<T>,
{
    #[allow(unused_variables)] // uprops_root is only used if icu_list
    Ok(MultiForkByKeyProvider {
        providers: vec![
            Box::new(AliasesProvider::try_from(cldr_paths)?),
            Box::new(CommonDateProvider::try_from(cldr_paths)?),
            Box::new(JapaneseErasProvider::try_from(cldr_paths)?),
            Box::new(LikelySubtagsProvider::try_from(cldr_paths)?),
            Box::new(NumbersProvider::try_from(cldr_paths)?),
            Box::new(PluralsProvider::try_from(cldr_paths)?),
            Box::new(TimeZonesProvider::try_from(cldr_paths)?),
            Box::new(WeekDataProvider::try_from(cldr_paths)?),
            Box::new(ListProvider::try_from(cldr_paths, _uprops_root)?),
        ],
    })
}

pub const ALL_KEYS: [ResourceKey; 19] = [
    icu_calendar::provider::JapaneseErasV1Marker::KEY,
    icu_datetime::provider::calendar::DatePatternsV1Marker::KEY,
    icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY,
    icu_datetime::provider::calendar::DateSymbolsV1Marker::KEY,
    icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker::KEY,
    icu_datetime::provider::time_zones::ExemplarCitiesV1Marker::KEY,
    icu_datetime::provider::time_zones::MetaZoneGenericNamesLongV1Marker::KEY,
    icu_datetime::provider::time_zones::MetaZoneGenericNamesShortV1Marker::KEY,
    icu_datetime::provider::time_zones::MetaZoneSpecificNamesLongV1Marker::KEY,
    icu_datetime::provider::time_zones::MetaZoneSpecificNamesShortV1Marker::KEY,
    icu_datetime::provider::week_data::WeekDataV1Marker::KEY,
    icu_decimal::provider::DecimalSymbolsV1Marker::KEY,
    icu_list::provider::AndListV1Marker::KEY,
    icu_list::provider::OrListV1Marker::KEY,
    icu_list::provider::UnitListV1Marker::KEY,
    icu_locale_canonicalizer::provider::AliasesV1Marker::KEY,
    icu_locale_canonicalizer::provider::LikelySubtagsV1Marker::KEY,
    icu_plurals::provider::CardinalV1Marker::KEY,
    icu_plurals::provider::OrdinalV1Marker::KEY,
];
