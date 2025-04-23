use iced::{
    Task, Element, Length, widget::{column, text},
};
use fa_iced::*;

struct IconTestApp;

#[derive(Debug, Clone)]
enum Message {}

impl Default for IconTestApp {
    fn default() -> Self {
        Self::new().0
    }
}

impl IconTestApp {
    fn new() -> (Self, Task<Message>) {
        load_font_fontawesome();
        let app = IconTestApp {};
        (
            app,
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Font Awesome Icon Test")
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            text("Should show a user icon below:"),
            iced_text_icon_regular::<Message>(FA_ICON_USER)
        ]
            .padding(40)
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[test]
#[ignore]
fn test_icon_rendering() -> iced::Result {
    let mut window_settings = iced::window::Settings::default();
    window_settings.size = iced::Size::new(1200.0,1024.0);
    iced::application(IconTestApp::title, IconTestApp::update, IconTestApp::view)
        .window(window_settings)
        .run()
}

fn main() -> iced::Result {
    let mut window_settings = iced::window::Settings::default();
    window_settings.size = iced::Size::new(1200.0,1024.0);
    iced::application(IconTestApp::title, IconTestApp::update, IconTestApp::view)
        .window(window_settings)
        .run()
}