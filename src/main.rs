use std::io::{self, Write};

#[derive(Copy, Clone, PartialEq)]
enum Player {
    Empty,
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
            _ => write!(f, " "),
        }
    }
}

fn board_display(board: &[Player; 9]) {
    for (index, field) in board.iter().enumerate() {
        if index % 3 == 0 {
            print!("\n");
        }
        match field {
            Player::Empty => print!("{} ", index + 1),
            Player::X => print!("X "),
            Player::O => print!("O "),
        }
    }
    print!("\n");
}

fn check_win(board: &[Player; 9], player: Player) -> bool {
    // Check rows
    for i in 0..3 {
        if board[i * 3] == player && board[i * 3 + 1] == player && board[i * 3 + 2] == player {
            return true;
        }
    }
    // Check columns
    for i in 0..3 {
        if board[i] == player && board[i + 3] == player && board[i + 6] == player {
            return true;
        }
    }
    // Check diagonals
    if board[0] == player && board[4] == player && board[8] == player {
        return true;
    }
    if board[2] == player && board[4] == player && board[6] == player {
        return true;
    }
    false
}

fn turn(
    board: &mut [Player; 9],
    player: Player,
    input: fn(&[Player; 9], Player) -> usize,
) -> Player {
    let i: usize = input(board, player);
    board[i] = player;
    return if check_win(board, player) {
        player
    } else {
        Player::Empty
    };
}

fn valid_move(board: &[Player; 9], i: usize) -> bool {
    Player::Empty == board[i]
}

fn check_draw(board: &[Player; 9]) -> bool {
    for i in 0..9 {
        if valid_move(board, i) {
            return false;
        }
    }
    return true;
}

fn swap_player(player: Player) -> Player {
    if player == Player::X {
        Player::O
    } else {
        Player::X
    }
}

fn player_input(board: &[Player; 9], player: Player) -> usize {
    loop {
        print!("{}: ", player);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");
        match input
            .trim()
            .parse::<usize>()
            .map_err(|_| "Invalid number")
            .and_then(|x| {
                if 1 <= x && x <= 9 {
                    Ok(x - 1)
                } else {
                    Err("Value must be less in between 1 and 9")
                }
            })
            .and_then(|x| {
                if valid_move(board, x) {
                    Ok(x)
                } else {
                    Err("Not a valid move")
                }
            }) {
            Ok(i) => return i,
            Err(msg) => println!("{}. Try Again!", msg),
        }
    }
}

fn minimax(maximize: bool, board: &mut [Player; 9], player: Player) -> i32 {
    if check_win(board, player) {
        return if maximize { 1 } else { -1 };
    }
    if check_win(board, swap_player(player)) {
        return if maximize { -1 } else { 1 };
    }
    if check_draw(board) {
        return 0;
    }

    if maximize {
        let mut score: i32 = std::i32::MIN;
        for i in 0..9 {
            if valid_move(board, i) {
                board[i] = player;
                let temp = minimax(!maximize, board, swap_player(player));
                board[i] = Player::Empty;
                if score < temp {
                    score = temp;
                }
            }
        }
        return score;
    } else {
        let mut score: i32 = std::i32::MAX;
        for i in 0..9 {
            if valid_move(board, i) {
                board[i] = player;
                let temp = minimax(!maximize, board, swap_player(player));
                board[i] = Player::Empty;
                if score > temp {
                    score = temp;
                }
            }
        }
        return score;
    }
}

fn ai_input(board: &[Player; 9], player: Player) -> usize {
    let mut simulated_board = board.clone();
    let mut score: i32 = std::i32::MIN;
    let mut best_move: usize = 0;
    for i in 0..9 {
        if valid_move(&simulated_board, i) {
            simulated_board[i] = player;
            let temp: i32 = minimax(false, &mut simulated_board, swap_player(player));
            simulated_board[i] = Player::Empty;
            if score < temp {
                score = temp;
                best_move = i;
            }
        }
    }
    println!("{}: {}", player, best_move + 1);
    return best_move;
}

fn main() {
    let mut board: [Player; 9] = [crate::Player::Empty; 9];
    let players: [Player; 2] = [Player::X, Player::O];
    let input: [fn(&[Player; 9], Player) -> usize; 2] = [player_input, ai_input];
    for round in 0..9 {
        board_display(&board);
        match turn(&mut board, players[round % 2], input[round % 2]) {
            Player::X => {
                board_display(&board);
                println!("X won!");
                return;
            }
            Player::O => {
                board_display(&board);
                println!("O won!");
                return;
            }
            _ => {}
        }
    }
    board_display(&board);
    println!("Draw!");
}
