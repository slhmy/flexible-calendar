use iced::{
    executor, time, Application, Command, Element,
    Settings, Text, Subscription, window
};
use image::open;

pub fn main() -> iced::Result {
    let icon = open("data/icon.jpg").unwrap().into_rgba8();
    let raw = icon.clone().into_raw();
    let width = icon.width();
    let height = icon.height();

    FlexibleCalendar::run(Settings {
        window: window::Settings {
            size: (400, 200),
            icon: Some(window::icon::Icon::from_rgba(raw, width, height).unwrap()),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct FlexibleCalendar {
    now: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Clone, Copy)]
enum ApplicationMessage {
    Tick(chrono::DateTime<chrono::Local>),
}

impl Application for FlexibleCalendar {
    type Executor = executor::Default;
    type Message = ApplicationMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            FlexibleCalendar {
                now: chrono::Local::now(),
            }, Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("flexible-calendar")
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new(format!("{}", self.now.format("%Y-%m-%d %H:%M:%S").to_string())).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(std::time::Duration::from_millis(500))
            .map(|_| Self::Message::Tick(chrono::Local::now()))
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                }
            }
        }

        Command::none()
    }
}