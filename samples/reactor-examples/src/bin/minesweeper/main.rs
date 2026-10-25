#![windows_subsystem = "windows"]

use std::time::{SystemTime, UNIX_EPOCH};

use islands_reactor::*;
use islands_reactor_examples::run;

const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const TOTAL: usize = WIDTH * HEIGHT;
const MINES: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellState {
    Hidden,
    Flagged,
    Revealed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameStatus {
    Playing,
    Won,
    Lost,
}

#[derive(Clone, PartialEq, Eq)]
struct Game {
    mines: [bool; TOTAL],
    cells: [CellState; TOTAL],
    status: GameStatus,
}

impl Game {
    fn new(seed: u64) -> Self {
        let mut positions = (0..TOTAL).collect::<Vec<_>>();
        let mut random = seed.max(1);
        for i in (1..positions.len()).rev() {
            random ^= random << 13;
            random ^= random >> 7;
            random ^= random << 17;
            positions.swap(i, random as usize % (i + 1));
        }

        let mut mines = [false; TOTAL];
        for index in positions.into_iter().take(MINES) {
            mines[index] = true;
        }
        Self {
            mines,
            cells: [CellState::Hidden; TOTAL],
            status: GameStatus::Playing,
        }
    }

    fn reveal(&self, index: usize) -> Self {
        if self.status != GameStatus::Playing || self.cells[index] != CellState::Hidden {
            return self.clone();
        }
        let mut next = self.clone();
        if next.mines[index] {
            next.status = GameStatus::Lost;
            for (i, is_mine) in next.mines.iter().copied().enumerate() {
                if is_mine {
                    next.cells[i] = CellState::Revealed;
                }
            }
            return next;
        }

        let mut queue = vec![index];
        while let Some(current) = queue.pop() {
            if next.cells[current] != CellState::Hidden || next.mines[current] {
                continue;
            }
            next.cells[current] = CellState::Revealed;
            if next.neighbor_mines(current) == 0 {
                queue.extend(neighbors(current).into_iter().filter(|neighbor| {
                    next.cells[*neighbor] == CellState::Hidden && !next.mines[*neighbor]
                }));
            }
        }
        if next
            .cells
            .iter()
            .enumerate()
            .all(|(i, cell)| next.mines[i] || *cell == CellState::Revealed)
        {
            next.status = GameStatus::Won;
        }
        next
    }

    fn toggle_flag(&self, index: usize) -> Self {
        if self.status != GameStatus::Playing || self.cells[index] == CellState::Revealed {
            return self.clone();
        }
        let mut next = self.clone();
        next.cells[index] = match next.cells[index] {
            CellState::Hidden => CellState::Flagged,
            CellState::Flagged => CellState::Hidden,
            CellState::Revealed => CellState::Revealed,
        };
        next
    }

    fn neighbor_mines(&self, index: usize) -> usize {
        neighbors(index)
            .into_iter()
            .filter(|neighbor| self.mines[*neighbor])
            .count()
    }

    fn label(&self, index: usize) -> String {
        match self.cells[index] {
            CellState::Flagged => String::from("F"),
            CellState::Hidden => String::new(),
            CellState::Revealed if self.mines[index] => String::from("X"),
            CellState::Revealed => match self.neighbor_mines(index) {
                0 => String::new(),
                count => count.to_string(),
            },
        }
    }
}

fn neighbors(index: usize) -> Vec<usize> {
    let x = index % WIDTH;
    let y = index / WIDTH;
    let mut output = Vec::with_capacity(8);
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                output.push(ny as usize * WIDTH + nx as usize);
            }
        }
    }
    output
}

fn seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(1, |elapsed| elapsed.as_nanos() as u64)
}

fn app(cx: &mut RenderCx) -> Element {
    let (game, set_game) = cx.use_state(Game::new(seed()));
    let status = match game.status {
        GameStatus::Playing => "Reveal cells. Right-click to flag.",
        GameStatus::Won => "Board cleared! Start another game.",
        GameStatus::Lost => "Mine hit. Start another game.",
    };
    let rows = (0..HEIGHT)
        .map(|y| {
            let cells = (0..WIDTH)
                .map(|x| {
                    let index = y * WIDTH + x;
                    let snapshot = game.clone();
                    let flag_snapshot = game.clone();
                    let reveal = set_game.clone();
                    let flag = set_game.clone();
                    button(game.label(index))
                        .width(30.0)
                        .height(30.0)
                        .on_click(move || reveal.call(snapshot.reveal(index)))
                        .on_right_tapped(move || flag.call(flag_snapshot.toggle_flag(index)))
                        .into()
                })
                .collect::<Vec<Element>>();
            hstack(cells).spacing(2.0).into()
        })
        .collect::<Vec<Element>>();
    let reset = set_game;
    border(
        vstack((
            text_block(status),
            vstack(rows).spacing(2.0),
            button("New game").on_click(move || reset.call(Game::new(seed()))),
        ))
        .spacing(10.0),
    )
    .padding(16.0)
    .into()
}

fn main() -> Result<()> {
    run("Minesweeper", app)
}
