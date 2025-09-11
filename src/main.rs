//Hello Ice
// use iced::widget::text;
// use iced::{Element, Sandbox, Settings, Theme};

// pub fn main() -> iced::Result {
//     Hello::run(Settings::default())
// }

// struct Hello;

// impl Sandbox for Hello {
//     type Message = ();  // Message represents events (e.g., button clicks).
//                         // () means no messages (because our app is static).
//     fn new() -> Self {  // Called at startup.
//         Self            // Returns initial state → here, just Hello.
//     }

//     fn title(&self) -> String {         // Defines the window’s title bar text.
//         String::from("Iced • Hello")
//     }

//     fn theme(&self) -> Theme {     // Sets the app’s theme → Light mode.
//         Theme::Dark               // Could also be Theme::Dark or Light
//     }

//     fn update(&mut self, _message: Self::Message) { // Runs when messages (events) arrive.
//         // Nothing to update                        // Since Message = (), nothing ever arrives → no updates needed.
//     }           

//     fn view(&self) -> Element<Self::Message> {      // Draw widgets. Builds the UI tree for each frame.
//         text("Hello, Iced!").into()                 // Returns a widget (here: a simple text).
//     }                                               // .into() converts the Text widget into a generic Element.
// }

// //Ice Counter
// use iced::widget::{button, column, row, text};
// use iced::{Alignment, Element, Sandbox, Settings, Theme};
// // button, column, row, text → common UI widgets.
// // Alignment → controls layout alignment.
// // Element → the generic container type for any widget tree.
// // Sandbox → the simple synchronous app trait.
// // Settings, Theme → window setup and light/dark style.

// pub fn main() -> iced::Result {
//     Counter::run(Settings::default())
// }

// #[derive(Default)] // #[derive(Default)] lets us create a Counter with value = 0.
// struct Counter {
//     value: i32,
// }

// #[derive(Debug, Clone, Copy)] // Clone + Copy so messages are lightweight and easy to reuse.
// enum Message {
//     Increment,
//     Decrement,
// }

// impl Sandbox for Counter {  
//     type Message = Message; // Tells Iced this app will use Message as its event type.

//     fn new() -> Self {
//         Self::default()
//     }

//     fn title(&self) -> String {
//         String::from("Iced • Counter (Sandbox)")
//     }

//     fn theme(&self) -> Theme {
//         Theme::Light
//     }

//     fn update(&mut self, message: Self::Message) {      // Runs whenever a Message is triggered.
//         match message {                                 // Changes the app’s state: increases or decreases the count.
//             Message::Increment => self.value += 1,      // After updating, Iced automatically re-renders the view.
//             Message::Decrement => self.value -= 1,
//         }
//     }

//     fn view(&self) -> Element<Self::Message> {
//         let dec = button(text("-")).on_press(Message::Decrement);
//         let inc = button(text("+")).on_press(Message::Increment);

//         column![                                        // A column containing a title and a row.
//             text("Counter").size(32),                   // The row has:
//             row![dec, text(self.value).size(28), inc]   // A decrement button "-".
//                 .spacing(12)                            // The current value (self.value).
//                 .align_items(Alignment::Center),        // An increment button "+".
//         ]
//         .padding(24)
//         .spacing(16)
//         .into()
//     }
// }

use std::ops::Add;

use iced::alignment::Horizontal;
use iced::widget::{button, column, container, row, text};
use iced::{Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    Calc::run(Settings::default())
}

struct Calc {
    display: String,         // shows the current entry/result, starts at "0"
    first: Option<f64>,      // LHS stored when an operator is chosen
    op: Option<Op>,          // pending operation (+ - * /)
    entering_new: bool,      // true when the next digit should start a new number
}

#[derive(Debug, Clone, Copy)]
enum Op { Add, Sub, Mul, Div }

#[derive(Debug, Clone)]
enum Message {
    Digit(u8),    // 0..=9
    SetOp(Op),    // + - * /
    Equals,       // =
    Clear,        // C
    Exit,         // X
}

impl Default for Calc {
    fn default() -> Self {
        Self {
            display: "0".into(),
            first: None,
            op: None,
            entering_new: false,
        }
    }
}

impl Application for Calc {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String { "Iced — Calculator (Starter+)".into() }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::Digit(n) => {
                // HINT IMPLEMENTED: number entry
                // If we're starting a fresh number or currently "0", replace.
                // Otherwise append.
                if self.entering_new || self.display == "0" {
                    self.display = n.to_string();
                    self.entering_new = false;
                } else {
                    self.display.push(char::from(b'0' + n));
                }
            }

            Message::SetOp(new_op) => {
                let current = parse_f64(&self.display);
                match (self.first, self.op) {
                    (None, _) => {
                        // First op after typing a number: store LHS
                        self.first = Some(current);
                    }
                    (Some(_lhs), Some(_op)) if !self.entering_new => {
                        // TODO (student): CHAINING

                        let result = apply(_op, _lhs, current);
                        if result.is_nan() {
                            self.display = "Error".to_string()
                        }else{
                            self.first = Some(result);
                            self.display = result.to_string();
                        }
                    }
                    _ => {
                        // Pressed operator twice? You choose a policy:
                        // - Replace pending op
                        // - Or ignore second press
                    }
                }
                // Set/replace the pending operator and start new entry
                self.op = Some(new_op);
                self.entering_new = true;
            }

            Message::Equals => {
                let current = parse_f64(&self.display);

                if let (Some(_lhs), Some(_op)) = (self.first, self.op) {
                    let result = apply(_op, _lhs, current);
                        if result.is_nan() {
                            self.display = "Error".to_string()
                        }else{
                            self.display = result.to_string();
                            self.first = Some(result);
                        }
                    
                    self.op = None;
                }
            }

            Message::Clear => {
                // Reset to initial state
                *self = Self::default();
                self.display = "0".into();
            }

            Message::Exit => {
                // Simple close for the lab
                std::process::exit(0);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Layout constants (avoid stretching to full window)
        const BTN: f32 = 64.0;
        const GAP: f32 = 8.0;
        const GRID_W: f32 = BTN * 4.0 + GAP * 3.0;

        // Uniform button helpers
        let btn = |label: &str, msg: Message| {
            button(text(label))
                .on_press(msg)
                .width(Length::Fixed(BTN))
                .height(Length::Fixed(BTN))
        };
        let digit = |n: u8| btn(&n.to_string(), Message::Digit(n));
        let op = |sym: &str, o: Op| btn(sym, Message::SetOp(o));
        let clear = |sym: &str| btn(sym, Message::Clear);
        let equal = |sym: &str| btn(sym, Message::Equals);

        // Display: fixed width, right aligned
        let display = container(text(&self.display).size(36))
            .width(Length::Fixed(GRID_W))
            .padding([8, 12])
            .align_x(Horizontal::Right);

        // Rows
        let r1 = row![ digit(1), digit(2), digit(3), op("+", Op::Add) ]
            .spacing(GAP).width(Length::Fixed(GRID_W));
        let r2 = row![ digit(4), digit(5), digit(6), op("-", Op::Sub) ]
            .spacing(GAP).width(Length::Fixed(GRID_W));
        let r3 = row![ digit(7), digit(8), digit(9), op("*", Op::Mul) ]
            .spacing(GAP).width(Length::Fixed(GRID_W));
        let r4 = row![ digit(00), clear("C"), equal("="), op("/", Op::Div) ]
            .spacing(GAP).width(Length::Fixed(GRID_W));
        // Do your code 

        // Exit matches grid width (not Fill)
        let exit = button(text("Exit (X)"))
            .on_press(Message::Exit)
            .width(Length::Fixed(GRID_W))
            .height(Length::Fixed(48.0));

        column![display, r1, r2, r3, r4, exit]
            .spacing(GAP)
            .padding(12)
            .into()
    }
}

/* ──────────────── Small utilities ──────────────── */

fn parse_f64(s: &str) -> f64 {
    // HINT: safe parse; extend later for decimals
    s.parse::<f64>().unwrap_or(0.0)
}

// You will write this:
fn apply(op: Op, a: f64, b: f64) -> f64 {
    match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
        Op::Div => if b == 0.0{
            f64::NAN
        }else{
            let result = a / b;
            let formatted = format!("{:.2}", result);
            parse_f64(&formatted)
        },
    }
}
