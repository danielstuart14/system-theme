use iced::widget::{button, center, column, row, text};
use iced::{Center, Element, Theme};
use system_theme::SystemTheme;

pub fn main() -> iced::Result {
    let theme = SystemTheme::new()
        .map(|sys_theme| sys_theme.into())
        .unwrap_or(Theme::Light);

    iced::application(ThemeApp::default, ThemeApp::update, ThemeApp::view)
        .theme(theme)
        .run()
}

#[derive(Default)]
struct ThemeApp {}

#[derive(Debug, Clone)]
enum Message {
    Clicked,
}

impl ThemeApp {
    fn view(&self) -> Element<'_, Message> {
        let txt = text!("Hello, World!");

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

    fn update(&mut self, _message: Message) {}
}
