#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
//!
#![warn(missing_docs, rust_2018_idioms, future_incompatible, keyword_idents)]

pub mod error;
mod platform;

use error::Error;

/// Theme scheme
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ThemeScheme {
    /// Light mode
    Light,
    /// Dark mode
    Dark,
}

/// Theme contrast
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ThemeContrast {
    /// Normal contrast
    Normal,
    /// High contrast
    High,
}

/// Theme accent color
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ThemeAccent {
    /// Red component (0.0 - 1.0)
    pub red: f32,
    /// Green component (0.0 - 1.0)
    pub green: f32,
    /// Blue component (0.0 - 1.0)
    pub blue: f32,
}

/// System theme implementation.
pub struct SystemTheme {
    platform: platform::Platform,
}

impl SystemTheme {
    /// Create a new instance of SystemTheme.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            platform: platform::Platform::new()?,
        })
    }

    /// Get the system theme scheme.
    pub fn theme_scheme(&self) -> Result<ThemeScheme, Error> {
        self.platform.theme_scheme()
    }

    /// Get the system theme contrast level.
    pub fn theme_contrast(&self) -> Result<ThemeContrast, Error> {
        self.platform.theme_contrast()
    }

    /// Get the system theme accent color.
    pub fn theme_accent(&self) -> Result<ThemeAccent, Error> {
        self.platform.theme_accent()
    }
}
