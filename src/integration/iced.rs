use std::sync::Arc;

use crate::{SystemTheme, ThemeColor, ThemePalette};

impl Into<iced::Color> for ThemeColor {
    fn into(self) -> iced::Color {
        iced::Color {
            r: self.red,
            g: self.green,
            b: self.blue,
            a: 1.0,
        }
    }
}

impl Into<iced::theme::Palette> for ThemePalette {
    fn into(self) -> iced::theme::Palette {
        iced::theme::Palette {
            background: self.background.into(),
            text: self.foreground.into(),
            primary: self.accent.into(),
            success: self.success.into(),
            warning: self.warning.into(),
            danger: self.danger.into(),
        }
    }
}

impl Into<iced::Theme> for SystemTheme {
    fn into(self) -> iced::Theme {
        (&self).into()
    }
}

impl Into<iced::Theme> for &SystemTheme {
    fn into(self) -> iced::Theme {
        let palette = ThemePalette::from(self);
        let theme = iced::theme::Custom::new(String::from("SystemTheme"), palette.into());

        iced::Theme::Custom(Arc::new(theme))
    }
}
