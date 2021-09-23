// Copyright (c) The cargo-guppy Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Error, Triple};
use std::{borrow::Cow, collections::BTreeSet, ops::Deref};

// This is generated by the build script.
include!(concat!(env!("OUT_DIR"), "/current_platform.rs"));

/// A platform to evaluate target specs against.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Platform {
    triple: Triple,
    target_features: TargetFeatures,
    flags: BTreeSet<Cow<'static, str>>,
}

impl Platform {
    /// Creates a new `Platform` from the given triple and target features.
    ///
    /// Returns an error if this platform wasn't known to `target-spec`.
    pub fn new(
        triple_str: impl Into<Cow<'static, str>>,
        target_features: TargetFeatures,
    ) -> Result<Self, Error> {
        let triple = Triple::new(triple_str.into()).map_err(Error::UnknownPlatformTriple)?;
        Ok(Self::from_triple(triple, target_features))
    }

    /// Returns the current platform, as detected at build time.
    ///
    /// This will return an error if the current platform was unknown to this version of
    /// `target-spec`.
    pub fn current() -> Result<Self, Error> {
        let triple = Triple::new(CURRENT_TARGET).map_err(Error::UnknownPlatformTriple)?;
        let target_features = TargetFeatures::features(CURRENT_TARGET_FEATURES.iter().copied());
        Ok(Self {
            triple,
            target_features,
            flags: BTreeSet::new(),
        })
    }

    /// Creates a new platform from a `Triple` and target features.
    pub fn from_triple(triple: Triple, target_features: TargetFeatures) -> Self {
        Self {
            triple,
            target_features,
            flags: BTreeSet::new(),
        }
    }

    /// Adds a set of flags to accept.
    ///
    /// A flag is a single token like the `foo` in `cfg(not(foo))`.
    ///
    /// A default `cargo build` will always evaluate flags to false, but custom wrappers may cause
    /// some flags to evaluate to true. For example, as of version 0.6, `cargo web build` will cause
    /// `cargo_web` to evaluate to true.
    pub fn add_flags(&mut self, flags: impl IntoIterator<Item = impl Into<Cow<'static, str>>>) {
        self.flags.extend(flags.into_iter().map(|s| s.into()));
    }

    /// Returns the target triple string for this platform.
    pub fn triple_str(&self) -> &str {
        self.triple.as_str()
    }

    /// Returns the set of flags enabled for this platform.
    pub fn flags(&self) -> impl Iterator<Item = &str> + ExactSizeIterator {
        self.flags.iter().map(|flag| flag.deref())
    }

    /// Returns true if this flag was set with `add_flags`.
    pub fn has_flag(&self, flag: impl AsRef<str>) -> bool {
        self.flags.contains(flag.as_ref())
    }

    /// Returns the underlying `Triple`.
    pub fn triple(&self) -> &Triple {
        &self.triple
    }

    /// Returns the set of target features for this platform.
    pub fn target_features(&self) -> &TargetFeatures {
        &self.target_features
    }
}

/// A set of target features to match.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum TargetFeatures {
    /// The target features are unknown.
    Unknown,
    /// Only match the specified features.
    Features(BTreeSet<Cow<'static, str>>),
    /// Match all features.
    All,
}

impl TargetFeatures {
    /// Creates a new `TargetFeatures` which matches some features.
    pub fn features(features: impl IntoIterator<Item = impl Into<Cow<'static, str>>>) -> Self {
        TargetFeatures::Features(features.into_iter().map(|s| s.into()).collect())
    }

    /// Creates a new `TargetFeatures` which doesn't match any features.
    pub fn none() -> Self {
        TargetFeatures::Features(BTreeSet::new())
    }

    /// Returns `Some(true)` if this feature is a match, `Some(false)` if it isn't, and `None` if
    /// the set of target features is unknown.
    pub fn matches(&self, feature: &str) -> Option<bool> {
        match self {
            TargetFeatures::Unknown => None,
            TargetFeatures::Features(features) => Some(features.contains(feature)),
            TargetFeatures::All => Some(true),
        }
    }
}
