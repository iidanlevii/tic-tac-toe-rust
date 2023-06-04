use console::Term;

fn main() {
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3];

    let mut player: char = 'O';

    loop {
        player = if player == 'X' { 'O' } else { 'X' };
        print_board(board);

        println!("\n{player} Turn:\nWhere you play?",);

        turn(&mut board, player);

        let (is_full, is_winner) = is_end(board);

        if is_full || is_winner {
            print_board(board);

            if is_winner {
                println!("\n{player} is the winner")
            } else {
                println!("\nDraw")
            }

            break;
        }
    }
}

fn print_board(board: [[char; 3]; 3]) {
    clearscreen::clear().expect("failed to clear screen");

    for row in 0..5 {
        for col in 0..5 {
            let i_even = row % 2 == 0;
            let j_even = col % 2 == 0;

            if i_even && j_even {
                print!("{}", board[row / 2][col / 2])
            } else if i_even {
                print!("│")
            } else if j_even {
                print!("─")
            } else {
                print!("┼")
            }
        }
        println!();
    }
}

fn turn(board: &mut [[char; 3]; 3], player: char) {
    let term: Term = Term::stdout();
    let mut input: char = term.read_char().expect("Can't read char from console");

    while !validate_input(input, *board) {
        println!("Invalid input, try again");
        input = term.read_char().expect("Can't read char from consoleW");
    }

    let (row, col) = get_input(input);

    board[row][col] = player;
}

fn get_input(input: char) -> (usize, usize) {
    let input: usize = input as usize - '1' as usize;

    let row: usize = 2 - input / 3;
    let col: usize = input % 3;

    return (row, col);
}

fn validate_input(input: char, board: [[char; 3]; 3]) -> bool {
    if input > '9' || input < '1' {
        return false;
    }

    let (row, col) = get_input(input);

    return board[row][col] == ' ';
}

fn is_end(board: [[char; 3]; 3]) -> (bool, bool) {
    let mut is_full: bool = true;
    let mut is_winner: bool = false;

    for row in 0..3 {
        if (board[row][0] != ' '
            && board[row][0] == board[row][1]
            && board[row][0] == board[row][2])
            || (board[0][row] != ' '
                && board[0][row] == board[1][row]
                && board[0][row] == board[2][row])
        {
            is_winner = true
        }

        for col in 0..3 {
            if board[row][col] == ' ' {
                is_full = false;
            }
        }
    }

    if (board[0][0] != ' ' && board[0][0] == board[1][1] && board[0][0] == board[2][2])
        || (board[0][2] != ' ' && board[0][2] == board[1][1] && board[0][2] == board[2][0])
    {
        is_winner = true
    }

    (is_full, is_winner)
}
