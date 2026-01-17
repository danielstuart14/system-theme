#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
//!
#![warn(missing_docs, rust_2018_idioms, future_incompatible, keyword_idents)]

pub mod error;
mod integration;
mod platform;
mod theme;

use error::Error;

#[doc(inline)]
pub use theme::{ThemeColor, ThemeContrast, ThemeKind, ThemePalette, ThemeScheme};

/// System theme implementation.
pub struct SystemTheme {
    platform: platform::Platform,
}

impl AsRef<SystemTheme> for SystemTheme {
    fn as_ref(&self) -> &SystemTheme {
        self
    }
}

impl From<SystemTheme> for ThemePalette {
    fn from(theme: SystemTheme) -> Self {
        (&theme).into()
    }
}

impl From<&SystemTheme> for ThemePalette {
    fn from(theme: &SystemTheme) -> Self {
        let kind = theme.get_kind().unwrap_or_default();

        let scheme = theme.get_scheme().unwrap_or_default();
        let contrast = theme.get_contrast().unwrap_or_default();

        let mut palette = ThemePalette::system_palette(kind, scheme, contrast);

        if let Ok(accent) = theme.get_accent() {
            palette.accent = accent;
        };

        palette
    }
}

impl SystemTheme {
    /// Create a new instance of SystemTheme.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            platform: platform::Platform::new()?,
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
}
