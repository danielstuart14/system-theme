#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
//!
#![warn(missing_docs, rust_2018_idioms, future_incompatible, keyword_idents)]

pub mod error;
mod integration;
mod platform;
mod theme;

use error::Error;

use async_stream::stream;
use futures_core::stream::Stream;
use std::hash::Hash;
use uuid::Uuid;

#[doc(inline)]
pub use theme::{Theme, ThemeColor, ThemeContrast, ThemeKind, ThemePalette, ThemeScheme};

/// System theme implementation.
pub struct SystemTheme {
    platform: platform::Platform,
    identifier: Uuid,
}

impl Hash for SystemTheme {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}

impl SystemTheme {
    /// Create a new instance of SystemTheme.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            platform: platform::Platform::new()?,
            identifier: Uuid::new_v4(),
        })
    }

    /// Get the system theme kind.
    pub fn get_kind(&self) -> Result<ThemeKind, Error> {
        self.platform.theme_kind()
    }

    /// Get the system theme scheme.
    pub fn get_scheme(&self) -> Result<ThemeScheme, Error> {
        self.platform.theme_scheme()
    }

    /// Get the system theme contrast level.
    pub fn get_contrast(&self) -> Result<ThemeContrast, Error> {
        self.platform.theme_contrast()
    }

    /// Get the system theme accent color.
    pub fn get_accent(&self) -> Result<ThemeColor, Error> {
        self.platform.theme_accent()
    }

    /// Get the system theme.
    ///
    /// This is based on the system theme kind, scheme, and contrast level.
    /// A fallback color is used if the platform does not provide it.
    pub fn get_theme(&self) -> Theme {
        let kind = self.get_kind().unwrap_or_default();

        let scheme = self.get_scheme().unwrap_or_default();
        let contrast = self.get_contrast().unwrap_or_default();

        Theme::new(kind, scheme, contrast, self.get_accent().ok())
    }

    /// Subscribe to system theme changes.
    pub fn subscribe(&self) -> impl Stream<Item = ()> {
        let notify = self.platform.get_notify();
        stream! {
            let mut notified = notify.notified();
            loop {
                // Wait for notification
                notified.await;
                // Create new notified before yielding
                notified = notify.notified();
                yield ();
            }
        }
    }
}
