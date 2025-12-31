use zbus::{
    blocking::{fdo::DBusProxy, Connection},
    names::BusName,
    zvariant::OwnedValue,
};

use crate::{error::Error, ThemeAccent, ThemeContrast, ThemeKind, ThemeScheme};

const DESKTOP_PORTAL_DEST: &str = "org.freedesktop.portal.Desktop";
const DESKTOP_PORTAL_PATH: &str = "/org/freedesktop/portal/desktop";
const SETTINGS_INTERFACE: &str = "org.freedesktop.portal.Settings";
const READ_METHOD: &str = "ReadOne";
const APPERANCE_NAMESPACE: &str = "org.freedesktop.appearance";

const COLOR_SCHEME_KEY: &str = "color-scheme";
const CONTRAST_KEY: &str = "contrast";
const ACCENT_COLOR_KEY: &str = "accent-color";

const PORTAL_NOT_FOUND: &str = "org.freedesktop.portal.Error.NotFound";
const DBUS_UNKNOWN_SERVICE: &str = "org.freedesktop.DBus.Error.ServiceUnknown";
const DBUS_UNKNOWN_METHOD: &str = "org.freedesktop.DBus.Error.UnknownMethod";

const GTK_PORTAL_IMPL: &str = "org.freedesktop.impl.portal.desktop.gtk";

impl From<zbus::Error> for Error {
    fn from(value: zbus::Error) -> Self {
        match &value {
            zbus::Error::InterfaceNotFound => Error::Unsupported,
            zbus::Error::Unsupported => Error::Unsupported,
            zbus::Error::MethodError(name, _, _) => {
                let name_str = name.as_str();
                // Errors that can be returned if not supported by the platform
                if name_str == PORTAL_NOT_FOUND
                    || name_str == DBUS_UNKNOWN_SERVICE
                    || name_str == DBUS_UNKNOWN_METHOD
                {
                    Error::Unsupported
                } else {
                    Error::from_platform(value)
                }
            }
            _ => Error::from_platform(value),
        }
    }
}

/// Check if color component is valid
fn check_color_component(component: f64) -> bool {
    (0.0..=1.0).contains(&component)
}

pub struct Platform {
    conn: Connection,
}

impl Platform {
    pub fn new() -> Result<Self, Error> {
        let conn = Connection::session()?;
        Ok(Self { conn })
    }

    pub fn theme_kind(&self) -> Result<ThemeKind, Error> {
        if self.check_has_owner(
            GTK_PORTAL_IMPL
                .try_into()
                .expect("Failed to convert GTK_PORTAL_IMPL"),
        )? {
            // If we have GTK Portal, we're using GTK
            Ok(ThemeKind::Gtk)
        } else {
            // Anything else will be Qt
            Ok(ThemeKind::Qt)
        }
    }

    pub fn theme_scheme(&self) -> Result<ThemeScheme, Error> {
        let scheme: u32 = self.get_settings_apperance(COLOR_SCHEME_KEY)?;

        // 1 = dark, 2 = light
        match scheme {
            1 => Ok(ThemeScheme::Dark),
            2 => Ok(ThemeScheme::Light),
            _ => Err(Error::Unavailable),
        }
    }

    pub fn theme_contrast(&self) -> Result<ThemeContrast, Error> {
        let contrast: u32 = self.get_settings_apperance(CONTRAST_KEY)?;

        // 0 = normal, 1 = high
        match contrast {
            0 => Ok(ThemeContrast::Normal),
            1 => Ok(ThemeContrast::High),
            _ => Err(Error::Unavailable),
        }
    }

    pub fn theme_accent(&self) -> Result<ThemeAccent, Error> {
        let accent: (f64, f64, f64) = self.get_settings_apperance(ACCENT_COLOR_KEY)?;

        // Check color components range (invalid -> not configured)
        if !check_color_component(accent.0)
            || !check_color_component(accent.1)
            || !check_color_component(accent.2)
        {
            return Err(Error::Unavailable);
        }

        Ok(ThemeAccent {
            red: accent.0 as f32,
            green: accent.1 as f32,
            blue: accent.2 as f32,
        })
    }

    fn check_has_owner(&self, name: BusName<'_>) -> Result<bool, Error> {
        let proxy = DBusProxy::new(&self.conn)?;

        match proxy.get_name_owner(name) {
            Ok(_) => Ok(true),
            Err(zbus::fdo::Error::NameHasNoOwner(_)) => Ok(false),
            Err(e) => Err(Error::from_platform(e)),
        }
    }

    fn get_settings_apperance<T: TryFrom<OwnedValue>>(&self, key: &str) -> Result<T, Error> {
        // Call method to read a settings appearance
        let response = self.conn.call_method(
            Some(DESKTOP_PORTAL_DEST),
            DESKTOP_PORTAL_PATH,
            Some(SETTINGS_INTERFACE),
            READ_METHOD,
            &(APPERANCE_NAMESPACE, key),
        )?;

        // As the result is a variant, convert it to a value first
        let value = response
            .body()
            .deserialize::<OwnedValue>()
            .map_err(Error::from_platform)?;

        // Now try to convert it to the desired type (invalid -> not configured)
        value.try_into().map_err(|_| Error::Unavailable)
    }
}
