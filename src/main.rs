use rand::Rng;
use std::fmt;

// TODO: add the scrore computation
// TODO: add a screen to replay
// TODO: add a leaderboad and a way to save score and running game to keep where we're at

// TIPS: start from opposite played side and try to get the cells to move block by block to that
// side

#[derive(Debug)]
struct CellPos(usize, usize);

#[derive(Debug, Copy)]
enum CellValue {
    Empty,
    Value(i32),
}

impl Clone for CellValue {
    fn clone(&self) -> Self {
        match &self {
            CellValue::Empty => CellValue::Empty,
            CellValue::Value(val) => CellValue::Value(*val),
        }
    }
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let CellValue::Value(value) = self {
            write!(f, "{}", value)
        } else {
            write!(f, "-1")
        }
    }
}

enum Move {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

impl Move {
    fn play(&self) {
        // TODO: add play function here
    }
}

struct Board {
    grid: [[CellValue; 4]; 4],
    score: u32,
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[CellValue::Empty; 4]; 4],
            score: 0,
        }
    }

    fn display(&self) {
        clear();

        println!("Score: {}", self.score);
        println!("-----------------------");

        for rows in self.grid.iter() {
            print!("|{:>5}|", "-----");
            print!("|{:>5}|", "-----");
            print!("|{:>5}|", "-----");
            print!("|{:>5}|", "-----");
            print!("\n");

            for col in rows {
                if let CellValue::Value(value) = col {
                    print!("|{:>5}|", value);
                } else {
                    print!("|{:>5}|", "");
                }
            }

            print!("\n");
        }

        print!("|{:>5}|", "-----");
        print!("|{:>5}|", "-----");
        print!("|{:>5}|", "-----");
        print!("|{:>5}|", "-----");
        print!("\n");
    }

    pub fn check_end(&self) -> bool {
        let mut is_end = true;

        for rows in self.grid.iter() {
            for col in rows {
                if let CellValue::Empty = col {
                    is_end = false;
                    break;
                }
            }
        }

        is_end
    }

    pub fn get_valid_gen_cells(&self) -> Vec<(usize, usize)> {
        let mut valid_cells: Vec<(usize, usize)> = vec![];

        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                if let CellValue::Empty = col {
                    valid_cells.push((row_index, col_index));
                }
            }
        }

        valid_cells
    }

    pub fn update_cell(&mut self, x: usize, y: usize, value: i32) {
        self.grid[x][y] = CellValue::Value(value);
    }

    pub fn get_random_valid_cell(&self) -> (usize, usize) {
        let cells = self.get_valid_gen_cells();
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..cells.len());
        let (x, y) = cells.get(random).expect("Something wrong");
        (*x, *y)
    }

    pub fn generate_new_cell(&mut self) {
        let (x, y) = self.get_random_valid_cell();
        let mut rng = rand::thread_rng();
        let mut value = rng.gen_range(1..=100);
        let value = if value > 90 { 4 } else { 2 };
        self.update_cell(x, y, value);
    }

    pub fn init(&mut self) {
        // for the first value
        let (x, y) = self.get_random_valid_cell();
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..=2);
        let value = value * 2;
        self.update_cell(x, y, value);
    }

    pub fn play(&mut self, player_move: Move) -> bool {
        let has_moved = match player_move {
            Move::TOP => self.play_top(),
            Move::LEFT => self.play_left(),
            Move::RIGHT => self.play_right(),
            Move::BOTTOM => self.play_bottom(),
        };

        has_moved
    }
    //    y(0) y(1) y(2) y(3)
    // x(0) 0    0    0    0
    // x(1) 0    0    0    0
    // x(2) 0    2    0    0
    // x(3) 0    0    0    0

    pub fn play_top(&mut self) -> bool {
        let mut has_moved = false;

        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                if let CellValue::Empty = self.grid[x][y] {
                    continue;
                }

                let mut next_row = x;
                let mut next_value = if let CellValue::Value(value) = self.grid[x][y] {
                    value
                } else {
                    -1
                };
                let mut merged = false;

                for i in 0..x {
                    if let CellValue::Empty = self.grid[x - i - 1][y] {
                        next_row = x - i - 1;
                    }

                    if let CellValue::Value(value) = self.grid[x - i - 1][y] {
                        if let CellValue::Value(val) = self.grid[x][y] {
                            if value == val {
                                next_row = x - i - 1;
                                next_value = next_value + value;
                                merged = true;
                                break;
                            }
                        }
                    }

                    if let CellValue::Value(_) = self.grid[x - i - 1][y] {
                        break;
                    }
                }

                if x != next_row {
                    self.grid[next_row][y] = CellValue::Value(next_value);
                    self.grid[x][y] = CellValue::Empty;
                    if merged {
                        self.score += next_value as u32;
                    }
                    has_moved = true;
                }
            }
        }

        has_moved
    }

    pub fn play_right(&mut self) -> bool {
        let mut has_moved = false;

        for x in (0..self.grid.len()).rev() {
            for y in (0..self.grid[x].len()).rev() {
                // Cell is empty so nothing to do
                if let CellValue::Empty = self.grid[x][y] {
                    continue;
                }

                let mut next_col = y;
                let mut next_value = if let CellValue::Value(val) = self.grid[x][y] {
                    val
                } else {
                    -1
                };
                let mut merged = false;

                for i in y..self.grid[x].len() {
                    if y == i {
                        continue;
                    }

                    if let CellValue::Empty = self.grid[x][i] {
                        next_col = i;
                    } else {
                        println!("y{i}");
                    }

                    if let CellValue::Value(value) = self.grid[x][i] {
                        if let CellValue::Value(val) = self.grid[x][y] {
                            if value == val {
                                next_col = i;
                                next_value = next_value + val;
                                merged = true;
                                break;
                            }
                        }
                    }

                    if let CellValue::Value(_) = self.grid[x][i] {
                        break;
                    }
                }

                if y != next_col {
                    self.grid[x][next_col] = CellValue::Value(next_value);
                    self.grid[x][y] = CellValue::Empty;
                    if merged {
                        self.score += next_value as u32;
                    }
                    has_moved = true;
                }
            }
        }

        has_moved
    }

    pub fn play_left(&mut self) -> bool {
        let mut has_moved = false;

        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                if let CellValue::Empty = self.grid[x][y] {
                    continue;
                }

                let mut next_col = y;
                let mut next_value = if let CellValue::Value(value) = self.grid[x][y] {
                    value
                } else {
                    -1
                };
                let mut merged = false;

                for i in 0..y {
                    if let CellValue::Empty = self.grid[x][y - i - 1] {
                        next_col = y - i - 1;
                    }

                    if let CellValue::Value(value) = self.grid[x][y - i - 1] {
                        if let CellValue::Value(val) = self.grid[x][y] {
                            if value == val {
                                next_col = y - i - 1;
                                next_value = next_value + value;
                                merged = true;
                                break;
                            }
                        }
                    }

                    if let CellValue::Value(_) = self.grid[x][y - i - 1] {
                        break;
                    }
                }

                if y != next_col {
                    self.grid[x][next_col] = CellValue::Value(next_value);
                    self.grid[x][y] = CellValue::Empty;
                    if merged {
                        self.score += next_value as u32;
                    }
                    has_moved = true;
                }
            }
        }

        has_moved
    }

    pub fn play_bottom(&mut self) -> bool {
        let mut has_moved = false;

        for x in (0..self.grid.len()).rev() {
            for y in (0..self.grid[x].len()).rev() {
                if let CellValue::Empty = self.grid[x][y] {
                    continue;
                }

                let mut next_row = x;
                let mut next_value = if let CellValue::Value(value) = self.grid[x][y] {
                    value
                } else {
                    -1
                };
                let mut merged = false;

                if x == self.grid.len() - 1 {
                    break;
                }

                for i in x..self.grid.len() {
                    if i == x {
                        continue;
                    }

                    if let CellValue::Empty = self.grid[i][y] {
                        next_row = i;
                    }

                    if let CellValue::Value(value) = self.grid[i][y] {
                        if let CellValue::Value(val) = self.grid[x][y] {
                            if value == val {
                                next_row = i;
                                next_value = next_value + value;
                                merged = true;
                                break;
                            }
                        }
                    }

                    if let CellValue::Value(_) = self.grid[i][y] {
                        break;
                    }
                }

                if x != next_row {
                    self.grid[next_row][y] = CellValue::Value(next_value);
                    self.grid[x][y] = CellValue::Empty;
                    if merged {
                        self.score += next_value as u32;
                    }

                    has_moved = true;
                }
            }
        }

        has_moved
    }
}

fn main() {
    let mut board = Board::new();

    if board.check_end() {
        std::process::exit(1);
    }

    board.init();
    board.generate_new_cell();

    loop {
        board.display();

        let input = ask_input();
        println!("input: {input}");

        if let Some(value) = input_to_enum(input.as_str()) {
            let has_moved = board.play(value);

            if board.check_end() {
                break;
            }

            if has_moved {
                board.generate_new_cell();
            }
        }
    }

    println!("Game end!");
}

fn input_to_enum(input: &str) -> Option<Move> {
    match input {
        "h" => Some(Move::LEFT),
        "j" => Some(Move::BOTTOM),
        "k" => Some(Move::TOP),
        "l" => Some(Move::RIGHT),
        _ => None,
    }
}

fn ask_input() -> String {
    let mut input = String::new();
    println!("Use H / J / K / L");
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");

    // loop {
    //     if input.trim() == "j" || input.trim() == "h" || input.trim() == "k" || input.trim() == "l"
    //     {
    //         break;
    //     } else {
    //         input = String::from("");
    //     }
    // }

    String::from(input.trim())
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
