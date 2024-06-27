use iced::window::settings::PlatformSpecific;
use iced::Application;
use iced::Font;
use iced::Settings;

mod ui;
use ui::Timer;

const APP_ID: &str = "cooking_timer";
const FONT_FAMILY: &str = "Noto Sans CJK JP";

fn main() -> Result<(), iced::Error> {
    let window_settings = iced::window::Settings {
        size: ui::SIZE,
        decorations: false,
        resizable: false,
        platform_specific: PlatformSpecific { application_id: String::from(APP_ID) },
        ..Default::default()
    };

    let font = Font {
        family: iced::font::Family::Name(FONT_FAMILY),
        ..Default::default()
    };

    Timer::run( Settings {
        window: window_settings,
        default_font: font,
        ..Default::default()
    })
}
