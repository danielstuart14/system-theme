use objc2::{available, rc::Retained, MainThreadMarker};
use objc2_app_kit::{
    NSAppearance, NSAppearanceNameAqua, NSAppearanceNameDarkAqua, NSApplication, NSColor,
    NSWorkspace,
};
use objc2_foundation::NSArray;

use crate::{error::Error, ThemeAccent, ThemeContrast, ThemeKind, ThemeScheme};

/// Check if the given NSAppearance is dark.
fn is_appearance_dark(appearance: Retained<NSAppearance>) -> bool {
    // External statics are unsafe
    let aqua_name = unsafe { NSAppearanceNameAqua };
    let dark_aqua_name = unsafe { NSAppearanceNameDarkAqua };

    // Check which appearance is best matched (dark vs normal aqua)
    match appearance
        .bestMatchFromAppearancesWithNames(&NSArray::from_slice(&[aqua_name, dark_aqua_name]))
    {
        Some(appearance_name) => *appearance_name == *dark_aqua_name,
        None => false,
    }
}

pub struct Platform {
    workspace: Retained<NSWorkspace>,
    application: Option<Retained<NSApplication>>,
}

impl Platform {
    pub fn new() -> Result<Self, Error> {
        // We can only get the application if we're on the main thread
        let application = if let Some(mtm) = MainThreadMarker::new() {
            Some(NSApplication::sharedApplication(mtm))
        } else {
            None
        };

        Ok(Platform {
            workspace: NSWorkspace::sharedWorkspace(),
            application,
        })
    }

    pub fn theme_kind(&self) -> Result<ThemeKind, Error> {
        Ok(ThemeKind::MacOS)
    }

    pub fn theme_scheme(&self) -> Result<ThemeScheme, Error> {
        // Method used is supported since 10.14
        if !available!(macos = 10.14) {
            return Err(Error::Unsupported);
        }

        self.application
            .as_ref()
            .map(|application| {
                if is_appearance_dark(application.effectiveAppearance()) {
                    Ok(ThemeScheme::Dark)
                } else {
                    Ok(ThemeScheme::Light)
                }
            })
            .unwrap_or(Err(Error::Unavailable))
    }

    pub fn theme_contrast(&self) -> Result<ThemeContrast, Error> {
        // Method used is supported since 10.10
        if !available!(macos = 10.10) {
            return Err(Error::Unsupported);
        }

        let contrast = if self.workspace.accessibilityDisplayShouldIncreaseContrast() {
            ThemeContrast::High
        } else {
            ThemeContrast::Normal
        };

        Ok(contrast)
    }

    pub fn theme_accent(&self) -> Result<ThemeAccent, Error> {
        // Method used is supported since 10.14
        if !available!(macos = 10.14) {
            return Err(Error::Unsupported);
        }

        let accent_color = NSColor::controlAccentColor();

        Ok(ThemeAccent {
            red: accent_color.redComponent() as f32,
            green: accent_color.greenComponent() as f32,
            blue: accent_color.blueComponent() as f32,
        })
    }
}
