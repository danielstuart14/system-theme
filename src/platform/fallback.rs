use crate::{error::Error, ThemeColor, ThemeContrast, ThemeKind, ThemeScheme};
use std::sync::Arc;
use std::sync::LazyLock;
use tokio::sync::Notify;

static DUMMY_NOTIFY: LazyLock<Arc<Notify>> = LazyLock::new(|| Arc::new(Notify::new()));

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

    pub fn get_notify(&self) -> Arc<Notify> {
        (*DUMMY_NOTIFY).clone()
    }
}
