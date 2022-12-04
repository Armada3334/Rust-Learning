use iced::{button, scrollable, text_input, Align, Button, Column, Command, Container,
    Element, Length, Row, Scrollable, Settings, Text, TextInput, Column as _};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut app = App {
        roll: rng.gen_range(1, 7),
        modifier: 0,
        roll_input: text_input::State::new(),
        modifier_input: text_input::State::new(),
    };

    let settings = Settings {
        window: iced::window::Settings {
            size: (300, 300),
            resizable: false,
            decorations: true,
        },
        ..Settings::default()
    };

    app.run(settings);
}

struct App {
    roll: u32,
    modifier: i32,
    roll_input: text_input::State,
    modifier_input: text_input::State,
}

impl App {
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Roll => {
                let mut rng = rand::thread_rng();
                self.roll = rng.gen_range(1, 7);
                Command::none()
            }
            Message::ModifierChanged(new_modifier) => {
                self.modifier = new_modifier;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let final_roll = self.roll as i32 + self.modifier;

        let roll_button = Button::new(&mut self.roll_input, Text::new("Roll"))
            .on_press(Message::Roll);

        let roll_text = Text::new(format!("Roll: {}", self.roll)).size(32);

        let modifier_input = TextInput::new(
            &mut self.modifier_input,
            "Modifier",
            &self.modifier.to_string(),
            Message::ModifierChanged,
        );

        let final_roll_text = Text::new(format!("Final roll: {}", final_roll)).size(32);

        let content = Column::new()
            .push(roll_text)
            .push(roll_button)
            .push(modifier_input)
            .push(final_roll_text);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Roll,
    ModifierChanged(i32),
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (App {
            roll: 0,
            modifier: 0,
            roll_input: text_input::State::new(),
            modifier_input: text_input::State::new(),
        },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("Dice Roller")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.update(message)
    }

    fn view(&mut self) -> Element<Message> {
        self.view()
    }
}
