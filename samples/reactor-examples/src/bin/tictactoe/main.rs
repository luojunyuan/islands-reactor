#![windows_subsystem = "windows"]

use islands_reactor::*;
use islands_reactor_examples::run;

fn main() -> Result<()> {
    run("Tic-tac-toe", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (board, set_board) = cx.use_state([None::<char>; 9]);
    let (next_player, set_next_player) = cx.use_state('X');
    let (message, set_message) = cx.use_state(String::from("X's turn"));
    let rows = (0..3)
        .map(|row| {
            let cells = (0..3)
                .map(|column| {
                    let index = row * 3 + column;
                    let value = board[index].map(String::from).unwrap_or_default();
                    let current_board = board;
                    let current_player = next_player;
                    let set_board = set_board.clone();
                    let set_next_player = set_next_player.clone();
                    let set_message = set_message.clone();
                    button(value)
                        .width(64.0)
                        .height(64.0)
                        .on_click(move || {
                            if current_board[index].is_some() {
                                return;
                            }
                            let mut updated = current_board;
                            updated[index] = Some(current_player);
                            set_board.call(updated);
                            let next = if current_player == 'X' { 'O' } else { 'X' };
                            set_next_player.call(next);
                            set_message.call(format!("{next}'s turn"));
                        })
                        .into()
                })
                .collect::<Vec<_>>();
            hstack(cells).spacing(4.0).into()
        })
        .collect::<Vec<_>>();
    let reset_board = set_board;
    let reset_player = set_next_player;
    let reset_message = set_message;
    border(
        vstack((
            text_block(message).font_size(18.0).bold(),
            vstack(rows).spacing(4.0),
            button("New game").on_click(move || {
                reset_board.call([None; 9]);
                reset_player.call('X');
                reset_message.call(String::from("X's turn"));
            }),
        ))
        .spacing(10.0),
    )
    .padding(16.0)
    .into()
}
