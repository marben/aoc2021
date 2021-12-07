use std::fmt::Formatter;
use std::io::BufRead;

struct BingoBoard {
    board_numbers: Vec<Vec<u32>>,
    marked_numbers: Vec<Vec<bool>>,
}

impl BingoBoard {
    fn new(board_yx: Vec<Vec<u32>>) -> BingoBoard {
        // transpose the input matrix, so that x is horizontal axis and y is vertical
        let mut board_numbers = vec![vec![0; 5]; 5];
        for x in 0..5 {
            for y in 0..5 {
                board_numbers[x][y] = board_yx[y][x];
            }
        }

        BingoBoard {
            marked_numbers: vec![vec![false; 5]; 5],
            board_numbers,
        }
    }

    // returns Some with the numbers of victory row/column
    fn draw_number(&mut self, n: u32) -> Option<Vec<u32>> {
        self.mark_numbers(n);

        if let Some(r) = self.find_winning_row() {
            return Some(r);
        }

        if let Some(c) = self.find_winning_column() {
            return Some(c);
        }

        None
    }

    fn mark_numbers(&mut self, n: u32) {
        for x in 0..5 {
            for y in 0..5 {
                if self.board_numbers[x][y] == n {
                    self.marked_numbers[x][y] = true;
                }
            }
        }
    }

    fn sum_of_unmarked_numbers(&self) -> u32 {
        let mut sum = 0;
        for x in 0..5 {
            for y in 0..5 {
                if self.marked_numbers[x][y] == false {
                    sum += self.board_numbers[x][y];
                }
            }
        }

        sum
    }

    fn find_winning_row(&self) -> Option<Vec<u32>> {
        for y in 0..5 {
            let mut row_nums = Vec::new();
            for x in 0..5 {
                if self.marked_numbers[x][y] == true {
                    row_nums.push(self.board_numbers[x][y]);
                }
            }
            if row_nums.len() == 5 {
                return Some(row_nums);
            }
        }

        None
    }

    fn find_winning_column(&self) -> Option<Vec<u32>> {
        for x in 0..5 {
            let mut col_nums = Vec::new();
            for y in 0..5 {
                if self.marked_numbers[x][y] == true {
                    col_nums.push(self.board_numbers[x][y]);
                }
            }
            if col_nums.len() == 5 {
                return Some(col_nums);
            }
        }

        None
    }
}

impl std::fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                if self.marked_numbers[x][y] == true {
                    write!(f, "|{:2} ", self.board_numbers[x][y])?;
                } else {
                    write!(f, "{:2}  ", self.board_numbers[x][y])?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn load_input() -> Result<(Vec<u32>, Vec<BingoBoard>), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("input")?;
    let reader = std::io::BufReader::new(f);

    let mut lines_iter = reader.lines();
    let drawn_numbers_line = lines_iter.next().ok_or("failed to get first line - drawn numbers")??;
    let drawn_numbers: Vec<u32> = drawn_numbers_line.trim().split(",").map(|s| s.parse::<u32>().unwrap()).collect();

    let mut boards = Vec::new();

    while let Some(l) = lines_iter.next() {
        let line = l?;
        if line != "" {
            return Err(format!("unexpected line '{}'", line).into());
        }

        let mut board = Vec::new();
        for i in 0..5 {
            let line = lines_iter.next().ok_or(format!("failed to get bingo line {}", i))??;
            let line_vec: Vec<u32> = line.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
            // println!("got line vec: {:?}", &line_vec);
            board.push(line_vec);
        }
        boards.push(BingoBoard::new(board));
    }

    Ok((drawn_numbers, boards))
}

fn print_boards_vector(boards: &Vec<BingoBoard>) {
    for b in boards {
        println!("{}", b);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (drawn_numbers, mut boards) = load_input()?;
    print_boards_vector(&boards);
    let mut winning_boards = Vec::new();

    for n in drawn_numbers {
        for i in (0..boards.len()).rev() {
            if let Some(_) = boards[i].draw_number(n) {
                let winning_board = boards.swap_remove(i);
                winning_boards.push((winning_board, n));
            }
        }
    }

    // println!("winning boards number: {}", winning_boards.len());
    let (first_board, first_board_draw_number) = &winning_boards[0];
    println!("first winning board result: {}", first_board.sum_of_unmarked_numbers() * first_board_draw_number);
    println!("first winning drawn number: {} resulting in board \n{}", first_board_draw_number, first_board);
    // println!("first winning board result: {}", first_board.sum_of_unmarked_numbers() * first_board_draw_number);

    let (last_board, last_board_drawn_number) = &winning_boards[winning_boards.len() - 1];
    println!("last winning board result: {}", last_board.sum_of_unmarked_numbers() * last_board_drawn_number);
    println!("last winning drawn number: {} resulting in board \n{}", last_board_drawn_number, last_board);

    Ok(())
}
