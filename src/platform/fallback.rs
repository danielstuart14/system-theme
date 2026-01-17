use crate::{error::Error, ThemeColor, ThemeContrast, ThemeKind, ThemeScheme};

pub struct Platform {}

impl Platform {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn theme_kind(&self) -> Result<ThemeKind, Error> {
        Err(Error::Unsupported)
    }

    pub fn theme_scheme(&self) -> Result<ThemeScheme, Error> {
        Err(Error::Unsupported)
    }

    pub fn theme_contrast(&self) -> Result<ThemeContrast, Error> {
        Err(Error::Unsupported)
    }

    pub fn theme_accent(&self) -> Result<ThemeColor, Error> {
        Err(Error::Unsupported)
    }
}
