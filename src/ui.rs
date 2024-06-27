use std::fmt::Display;
use std::time::Instant;

use iced::{alignment, executor};
use iced::widget::{column, row, text, Button, Space, Text};
use iced::window;
use iced::Application;
use iced::Command;

mod theme;
mod tests;

const SCALE: u16 = 1;
pub const SIZE: iced::Size = iced::Size { width: 220.0 * SCALE as f32, height: 90.0 * SCALE as f32 };
const TITLE: &str = "Cooking Timer";
const FPS: u64 = 60;
// Cannot be higher than the windows refresh rate

#[derive(Debug, Clone)]
pub enum Message {
    NextFrame(Instant),
    TimerStart,
    TimerStop,
    TimerReset,
}

pub struct Timer {
    pub ticking: bool,
    pub elapsed: f64,

    frame_count: u64,
    frame_start: Instant,
}
impl Default for Timer {
    fn default() -> Self {
        Self {
            ticking: true,
            elapsed: 0.0,

            frame_count: 0,
            frame_start: Instant::now(),
        }
    }
}
impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours: u64 = self.elapsed as u64 / 3600;
        let minutes: u64 = (self.elapsed as u64 % 3600) / 60;
        let seconds: u64 = (self.elapsed as u64 % 3600) % 60;

        write!(f, "{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
impl Application for Timer {
    type Message = Message;
    type Theme = theme::Kanagawa;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from(TITLE)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let delta: f64 = self.frame_start.elapsed().as_secs_f64();
        self.frame_start = Instant::now();

        match message {
            Message::NextFrame(_) => {},
            // Don't use the delta from here it's not accurate
            Message::TimerStart => self.toggle(true),
            Message::TimerStop => self.toggle(false),
            Message::TimerReset => self.reset(),
        }

        if self.ticking {
            self.elapsed += delta;
        }

        self.frame_count += 1;

        let mut frame_time: f64 = self.frame_start.elapsed().as_secs_f64();
        while frame_time < 1.0 / FPS as f64 {
            // Waits long enough to limit framerate to FPS
            frame_time = self.frame_start.elapsed().as_secs_f64();
        }

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // Requests next frame based on the refresh rate of the window
        window::frames().map(Message::NextFrame)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        // TODO: Make the colours pretty

        let window_margin: u16 = 10 * SCALE;

        let label_width: u16 = 80 * SCALE;
        let label_margin: u16 = (SIZE.width as u16 - label_width) / 2;
        let label = Text::new(format!("{}", self))
            .width(label_width)
            .size(20 * SCALE);

        let button_width: u16 = 60 * SCALE;
        let button_height: u16 = 35 * SCALE;
        let button_margin: u16 = (SIZE.width as u16 - (button_width * 3)) / 2;
        let mut button_row = row![];
        let buttons: Vec<(&str, Message)> = vec![
            ("Start", Message::TimerStart),
            ("Stop", Message::TimerStop),
            ("Reset", Message::TimerReset),
        ];
        for (button_content, message) in buttons {
            let button = Button::new
                (
                    text(button_content)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
                    .size(14 * SCALE)
                )
                .width(button_width)
                .height(button_height)
                .on_press(message);

            button_row = button_row.push(button);
        }

        column![
            Space::new(0, window_margin),
            row![Space::new(label_margin, 0), label],
            Space::new(0, window_margin),
            row![Space::new(button_margin, 0), button_row],
            Space::new(0, window_margin),
        ].into()
    }
}
impl Timer {
    fn toggle(&mut self, toggled: bool) {
        self.ticking = toggled;
    }

    fn reset(&mut self) {
        if !self.ticking {
            self.elapsed = 0.0;
        }
    }
}
