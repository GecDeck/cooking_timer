use iced::widget::{button, text};
use iced::{application, color, Color};

const TEXT: Color = color!(0xDCD7BA);
const BACKGROUND: Color = color!(0x1F1F28);
const ACCENT: Color = color!(0x414148);
const ACTIVE: Color = color!(0xC8C093);
const ACTIVE_TEXT: Color = color!(0x1D1C16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Kanagawa {
    #[default]
    Wave,
}
impl application::StyleSheet for Kanagawa {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: BACKGROUND,
            text_color: TEXT,
        }
    }
}
impl text::StyleSheet for Kanagawa {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: Some(TEXT),
        }
    }
}
impl button::StyleSheet for Kanagawa {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::new(0.0, 0.0),
            background: Some(iced::Background::Color(ACCENT)),
            text_color: TEXT,
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::new(0.0, 0.0),
            background: Some(iced::Background::Color(ACCENT)),
            text_color: ACTIVE_TEXT,
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        // BUG: Text colour not changing
        button::Appearance {
            shadow_offset: iced::Vector::new(0.0, 0.0),
            background: Some(iced::Background::Color(ACTIVE)),
            text_color: ACTIVE_TEXT,
            ..Default::default()
        }
    }
}
