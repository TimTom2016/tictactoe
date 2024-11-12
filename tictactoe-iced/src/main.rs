use iced::futures;
use iced::widget::{self, center, column, row, text};
use iced::*;
use iced::{Center, Element, Fill, Right, Task};
pub fn main() -> iced::Result {
    iced::application("TicTacToe Rust", TicTacToeApp::update, TicTacToeApp::view)
        .run_with(TicTacToeApp::new)
}

#[derive(Debug)]
enum TicTacToe {
    Player1,
    Player2,
    Stopped,
}

pub struct TicTacToeApp {
    grid: Grid,
    state: TicTacToe,
}

#[derive(Debug, Clone)]
enum Message {
    PlayerMoved((i64, i64, FieldStates)),
}

impl TicTacToeApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                grid: Grid::new(3, 3),
                state: TicTacToe::Stopped,
            },
            Task::none(),
        )
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content: Element<_> = match self.state {
            TicTacToe::Player1 => text("Player1").into(),
            TicTacToe::Player2 => column![text("Player2")].into(),
            TicTacToe::Stopped => text("Stopped").into(),
        };
        for field in self.grid {}

        center(content).into()
    }
}
mod grid {
    use iced::{mouse::Interaction, widget::canvas};
    use tictactoe_logic::grid::{self, FieldStates};
    pub struct Grid {
        grid: grid::Grid,
    }
}
