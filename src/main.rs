use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use std::cmp::min;
use std::collections::HashSet;

type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Flag {
    Unflagged,
    Unsure,
    Sure,
}

#[derive(Debug, Clone, Copy)]
enum CellType {
    Empty { adjacent_mines: u8 },
    Mine,
}

#[derive(Debug, Clone, Copy)]
enum CellState {
    Opened,
    Unopened(Flag),
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    cell_type: CellType,
    state: CellState,
}

#[derive(Debug, Clone, Copy)]
enum GameStatus {
    InProgress,
    Lost,
    Won,
}

#[derive(Debug, Clone)]
struct GameState {
    status: GameStatus,
    grid: Vec<Vec<Cell>>,
}

fn random_coordinates(count: usize, max_width: usize, max_height: usize) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let width = Uniform::from(0..max_width);
    let height = Uniform::from(0..max_height);

    let mut coordinates = HashSet::with_capacity(count);
    while coordinates.len() < count {
        coordinates.insert((width.sample(&mut rng), height.sample(&mut rng)));
    }

    return coordinates.into_iter().collect();
}

fn find_adjacent(x: usize, y: usize, width: usize, height: usize) -> Vec<Point> {
    let xs = (if x == 0 { 0 } else { x - 1 }..min(x + 2, width)).collect::<Vec<usize>>();
    let ys = (if y == 0 { 0 } else { y - 1 }..min(y + 2, height)).collect::<Vec<usize>>();

    return xs
        .into_iter()
        .cartesian_product(ys.into_iter())
        .into_iter()
        .filter(|&(adj_x, adj_y)| x != adj_x || y != adj_y)
        .collect();
}

fn initial_state(width: usize, height: usize, mines: usize) -> GameState {
    let mut grid = vec![
        vec![
            Cell {
                cell_type: CellType::Empty { adjacent_mines: 0 },
                state: CellState::Unopened(Flag::Unflagged),
            };
            usize::from(width)
        ];
        usize::from(height)
    ];

    let mines = random_coordinates(mines, width, height);

    // Inject mines
    for &(mine_x, mine_y) in mines.iter() {
        let mine = grid[mine_y][mine_x];
        grid[mine_y][mine_x] = Cell {
            cell_type: CellType::Mine,
            ..mine
        };

        // Increment adjacent_mines count for adjacent cells
        let adj = find_adjacent(mine_x, mine_y, width, height);
        for &(x, y) in adj.iter() {
            let cell = grid[y][x];
            match cell.cell_type {
                CellType::Empty { adjacent_mines } => {
                    grid[y][x] = Cell {
                        cell_type: CellType::Empty {
                            adjacent_mines: adjacent_mines + 1,
                        },
                        ..cell
                    }
                }
                CellType::Mine => (),
            }
        }
    }

    return GameState {
        grid,
        status: GameStatus::InProgress,
    };
}

fn is_game_won(grid: &Vec<Vec<Cell>>) -> bool {
    return grid.into_iter().flatten().all(|cell| match cell {
        Cell {
            cell_type: CellType::Empty { adjacent_mines: _ },
            state: CellState::Opened,
        } => true,
        Cell {
            cell_type: CellType::Empty { adjacent_mines: _ },
            state: CellState::Unopened(_),
        } => false,
        Cell {
            cell_type: CellType::Mine,
            state: _,
        } => true,
    });
}

fn open_cell(state: GameState, point: Point) -> GameState {
    match state.status {
        GameStatus::InProgress => {
            let (x, y) = point;
            let mut grid = state.grid;
            let cell = grid[y][x];
            match cell.state {
                CellState::Opened => GameState {
                    status: state.status,
                    grid,
                },
                _ => {
                    // TODO: Add algorithm to auto open empty cells without adjacent mines
                    grid[y][x] = Cell {
                        state: CellState::Opened,
                        ..cell
                    };
                    match cell.cell_type {
                        CellType::Mine => GameState {
                            status: GameStatus::Lost,
                            grid,
                        },
                        _ => GameState {
                            status: if is_game_won(&grid) {
                                GameStatus::Won
                            } else {
                                GameStatus::InProgress
                            },
                            grid,
                        },
                    }
                }
            }
        }
        _ => GameState {
            status: state.status,
            grid: state.grid,
        },
    }
}

fn change_flag(state: GameState, point: Point, flag: Flag) -> GameState {
    match state.status {
        GameStatus::InProgress => {
            let (x, y) = point;
            let mut grid = state.grid;
            let cell = grid[y][x];
            match cell.state {
                CellState::Opened => GameState {
                    status: state.status,
                    grid,
                },
                _ => {
                    grid[y][x] = Cell {
                        state: CellState::Unopened(flag),
                        ..cell
                    };
                    return GameState {
                        status: if is_game_won(&grid) {
                            GameStatus::Won
                        } else {
                            GameStatus::InProgress
                        },
                        grid,
                    };
                }
            }
        }
        _ => GameState {
            status: state.status,
            grid: state.grid,
        },
    }
}

fn main() {
    let state = initial_state(10, 10, 25);
    open_cell(state, (0, 0));
}
