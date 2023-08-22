use rand::Rng;

// TODO: add the scrore computation
// TODO: add a screen to replay
// TODO: add a leaderboad and a way to save score and running game to keep where we're at

// TIPS: start from opposite played side and try to get the cells to move block by block to that
// side

struct CellPos(usize, usize);
enum CellValue {
    Empty,
    Value(i32),
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
    grid: [[i32; 4]; 4],
    score: u32,
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[-1; 4]; 4],
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
                if *col > 0 {
                    print!("|{:>5}|", col);
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
                if *col == -1 {
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
                if *col == -1 {
                    valid_cells.push((row_index, col_index));
                }
            }
        }

        valid_cells
    }

    pub fn update_cell(&mut self, x: usize, y: usize, value: i32) {
        self.grid[x][y] = value;
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
        let value = rng.gen_range(1..=2);
        let value = value * 2;
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

    pub fn play(&mut self, player_move: Move) {
        match player_move {
            Move::TOP => self.play_top(),
            Move::LEFT => self.play_left(),
            Move::RIGHT => self.play_right(),
            Move::BOTTOM => self.play_bottom(),
        }
    }
    //    y(0) y(1) y(2) y(3)
    // x(0) 0    0    0    0
    // x(1) 0    0    0    0
    // x(2) 0    2    0    0
    // x(3) 0    0    0    0

    pub fn play_top(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                if self.grid[x][y] == -1 {
                    continue;
                }

                let mut next_row = x;
                let mut next_value = self.grid[x][y];
                let mut merged = false;

                for i in 0..x {
                    if self.grid[x - i - 1][y] == -1 {
                        next_row = x - i - 1;
                    }

                    if self.grid[x - i - 1][y] != -1 && self.grid[x - i - 1][y] == self.grid[x][y] {
                        next_row = x - i - 1;
                        next_value = next_value + self.grid[next_row][y];
                        merged = true;
                        break;
                    }

                    if self.grid[x - i - 1][y] != -1 {
                        break;
                    }
                }

                // println!(
                //     "prev({x}, {y}) / next({next_row}, {y}): ({}/{next_value})",
                //     self.grid[x][y]
                // );

                if x != next_row {
                    self.grid[next_row][y] = next_value;
                    self.grid[x][y] = -1;
                    if merged {
                        self.score += next_value as u32;
                    }
                }
            }
        }
    }

    pub fn play_right(&mut self) {
        for x in (0..self.grid.len()).rev() {
            for y in (0..self.grid[x].len()).rev() {
                // Cell is empty so nothing to do
                if self.grid[x][y] == -1 {
                    continue;
                }

                let mut next_col = y;
                let mut next_value = self.grid[x][y];
                let mut merged = false;

                for i in y..self.grid[x].len() {
                    if self.grid[x][i] == -1 {
                        next_col = i;
                    }

                    if self.grid[x][i] != -1 && self.grid[x][i] == self.grid[x][y] {
                        next_col = i;
                        next_value = next_value + self.grid[x][i];
                        merged = true;
                        break;
                    }

                    if self.grid[x][i] != -1 {
                        break;
                    }
                }

                if y != next_col {
                    self.grid[x][next_col] = next_value;
                    self.grid[x][y] = -1;
                    if merged {
                        self.score += next_value as u32;
                    }
                }
            }
        }
    }

    pub fn play_left(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                if self.grid[x][y] == -1 {
                    continue;
                }

                let mut next_col = y;
                let mut next_value = self.grid[x][y];
                let mut merged = false;

                for i in 0..y {
                    if self.grid[x][y - i - 1] == -1 {
                        next_col = y - i - 1;
                    }

                    if self.grid[x][y - i - 1] != -1 && self.grid[x][y - i - 1] == self.grid[x][y] {
                        next_col = y - i - 1;
                        next_value = next_value + self.grid[x][next_col];
                        merged = true;
                        break;
                    }

                    if self.grid[x][y - i - 1] != -1 {
                        break;
                    }
                }

                if y != next_col {
                    self.grid[x][next_col] = next_value;
                    self.grid[x][y] = -1;
                    if merged {
                        self.score += next_value as u32;
                    }
                }
            }
        }
    }

    pub fn play_bottom(&mut self) {
        for x in (0..self.grid.len()).rev() {
            for y in (0..self.grid[x].len()).rev() {
                println!("({x}, {y}): [{}]", self.grid[x][y]);
                if self.grid[x][y] == -1 {
                    continue;
                }

                let mut next_row = x;
                let mut next_value = self.grid[x][y];
                let mut merged = false;

                if x == self.grid.len() - 1 {
                    break;
                }

                for i in x..self.grid.len() {
                    if i == x {
                        continue;
                    }

                    if self.grid[i][y] == -1 {
                        next_row = i;
                    }

                    if self.grid[i][y] != -1 && self.grid[i][y] == self.grid[x][y] {
                        next_row = i;
                        next_value = next_value + self.grid[next_row][y];
                        merged = true;
                        break;
                    }

                    if self.grid[i][y] != -1 {
                        break;
                    }
                }

                if x != next_row {
                    self.grid[next_row][y] = next_value;
                    self.grid[x][y] = -1;
                    if merged {
                        self.score += next_value as u32;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut board = Board::new();

    if board.check_end() {
        std::process::exit(1);
    }

    board.init();

    loop {
        board.generate_new_cell();
        board.display();

        let input = ask_input();
        println!("input: {input}");

        board.play(input_to_enum(input.as_str()));

        if board.check_end() {
            break;
        }
    }

    println!("Game end!");
}

fn input_to_enum(input: &str) -> Move {
    match input {
        "h" => Move::LEFT,
        "j" => Move::BOTTOM,
        "k" => Move::TOP,
        "l" => Move::RIGHT,
        _ => panic!("Something's wrong"),
    }
}

fn ask_input() -> String {
    let mut input = String::new();
    println!("Use H / J / K / L");

    loop {
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");
        if input.trim() == "j" || input.trim() == "h" || input.trim() == "k" || input.trim() == "l"
        {
            break;
        }
    }
    String::from(input.trim())
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
