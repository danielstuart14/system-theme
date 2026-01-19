use iced::widget::{button, center, column, row, text};
use iced::Subscription;
use iced::{Center, Element, Theme};
use std::sync::Arc;
use system_theme::SystemTheme;

pub fn main() -> iced::Result {
    iced::application(ThemeApp::new, ThemeApp::update, ThemeApp::view)
        .theme(ThemeApp::theme)
        .subscription(ThemeApp::subscription)
        .run()
}

struct ThemeApp {
    sys_theme: Option<Arc<SystemTheme>>,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Clicked,
    ThemeChanged,
}

impl ThemeApp {
    fn new() -> Self {
        let sys_theme = SystemTheme::new().ok();

        let theme = sys_theme
            .as_ref()
            .map(|sys_theme| sys_theme.get_theme().into())
            .unwrap_or(Theme::Light);

        Self {
            sys_theme: sys_theme.map(Arc::new),
            theme,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions = Vec::new();

        if let Some(sys_theme) = self.sys_theme.clone() {
            subscriptions.push(
                Subscription::run_with(sys_theme, |sys_theme| sys_theme.subscribe())
                    .map(|_| Message::ThemeChanged),
            )
        }

        Subscription::batch(subscriptions)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged => {
                // Get the new system theme
                self.sys_theme.as_ref().map(|sys_theme| {
                    self.theme = sys_theme.get_theme().into();
                });
            }
            _ => {} // Do nothing
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn view(&self) -> Element<'_, Message> {
        let txt = text(format!("Theme: {}", self.theme));

        let button = |label| button(text(label).align_x(Center)).padding(10).width(80);

        let accent_button = button("Accent").on_press(Message::Clicked);
        let danger_button = button("Danger")
            .style(button::danger)
            .on_press(Message::Clicked);
        let warning_button = button("Warning")
            .style(button::warning)
            .on_press(Message::Clicked);
        let success_button = button("Success")
            .style(button::success)
            .on_press(Message::Clicked);

        let controls =
            row![accent_button, danger_button, warning_button, success_button].spacing(20);

        let content = column![txt, controls].align_x(Center).spacing(20);

        center(content).into()
    }
}
