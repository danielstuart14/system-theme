use windows::{
    core::HSTRING,
    Foundation::Metadata::ApiInformation,
    UI::{
        Color,
        ViewManagement::{AccessibilitySettings, UIColorType, UISettings},
    },
};

use crate::{error::Error, ThemeColor, ThemeContrast, ThemeKind, ThemeScheme};

impl From<Color> for ThemeColor {
    fn from(color: Color) -> Self {
        ThemeColor {
            red: color.R as f32 / 255.0,
            green: color.G as f32 / 255.0,
            blue: color.B as f32 / 255.0,
        }
    }
}

// Check if GetColorValue is supported
fn check_color_supported() -> Result<bool, Error> {
    const GET_COLOR_VALUE_TYPE: &str = "Windows.UI.ViewManagement.UISettings";
    const GET_COLOR_VALUE_METHOD: &str = "GetColorValue";

    ApiInformation::IsMethodPresent(
        &HSTRING::from(GET_COLOR_VALUE_TYPE),
        &HSTRING::from(GET_COLOR_VALUE_METHOD),
    )
    .map_err(Error::from_platform)
}

// Check if GetColorValue is supported
fn check_high_contrast_supported() -> Result<bool, Error> {
    const GET_HIGH_CONTRAST_TYPE: &str = "Windows.UI.ViewManagement.AccessibilitySettings";
    const GET_HIGH_CONTRAST_VALUE_METHOD: &str = "HighContrast";

    ApiInformation::IsPropertyPresent(
        &HSTRING::from(GET_HIGH_CONTRAST_TYPE),
        &HSTRING::from(GET_HIGH_CONTRAST_VALUE_METHOD),
    )
    .map_err(Error::from_platform)
}

pub struct Platform {
    ui_settings: Option<UISettings>,
    a11y_settings: Option<AccessibilitySettings>,
}

impl Platform {
    pub fn new() -> Result<Self, Error> {
        // Check if GetColorValue is supported
        let ui_settings = if check_color_supported()? {
            Some(UISettings::new().map_err(Error::from_platform)?)
        } else {
            None
        };

        // Check if HighContrast is supported
        let a11y_settings = if check_high_contrast_supported()? {
            Some(AccessibilitySettings::new().map_err(Error::from_platform)?)
        } else {
            None
        };

        Ok(Platform {
            ui_settings,
            a11y_settings,
        })
    }

    pub fn theme_kind(&self) -> Result<ThemeKind, Error> {
        Ok(ThemeKind::Windows)
    }

    pub fn theme_scheme(&self) -> Result<ThemeScheme, Error> {
        // Get the background color reported by windows and check if dark
        let background = self.get_ui_color(UIColorType::Background)?;

        // Simple way for checking if it is dark. Windows returns #000 or #FFF anyways.
        let color_sum = background.R as u16 + background.G as u16 + background.B as u16;
        Ok(if color_sum < 3 * 128 {
            ThemeScheme::Dark
        } else {
            ThemeScheme::Light
        })
    }

    pub fn theme_contrast(&self) -> Result<ThemeContrast, Error> {
        // Check if high contrast mode is enabled (if supported)
        self.a11y_settings
            .as_ref()
            .map(|settings| {
                settings
                    .HighContrast()
                    .map(|high_contrast| {
                        if high_contrast {
                            ThemeContrast::High
                        } else {
                            ThemeContrast::Normal
                        }
                    })
                    .map_err(Error::from_platform)
            })
            .unwrap_or(Err(Error::Unsupported))
    }

    pub fn theme_accent(&self) -> Result<ThemeColor, Error> {
        // Get main accent color. Ignoring accent shades for now.
        self.get_ui_color(UIColorType::Accent)
            .map(|color| color.into())
    }

    fn get_ui_color(&self, color_type: UIColorType) -> Result<Color, Error> {
        // Get the color reported by windows (if supported)
        self.ui_settings
            .as_ref()
            .map(|settings| {
                settings
                    .GetColorValue(color_type)
                    .map_err(Error::from_platform)
            })
            .unwrap_or(Err(Error::Unsupported))
    }
}
