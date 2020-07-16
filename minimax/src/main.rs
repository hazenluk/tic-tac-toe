use std::io::{self, Write};
fn main() {
    loop {
        let mut board: [[i8; 3]; 3] = [[0; 3]; 3]; //initialize to 0
        let mut turn = 1;
        if !confirm("Will you go first? Enter y for yes or any other key for no: "){
            //AI moves first
            let ai_move = evaluate(&board, &turn);
            board[ai_move.0][ai_move.1] = -1;
            println!("\n\t    The AI marks ({}, {}):", ai_move.0, ai_move.1);
            draw_tabbed(&board);
            turn += 1;
        }
        else{
            //Player moves first
            draw(&board);
        }
        
        loop {
            let mut c1 = String::new();
            let mut c2 = String::new();
            print!("\nEnter row coordinate: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut c1)
                .expect("Failed to read line");
            let c1: usize = match c1.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Enter a positive integer only");
                    continue;
                },
            };
            print!("Enter column coordinate: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut c2)
                .expect("Failed to read line");
            let c2: usize = match c2.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Enter a positive integer only");
                    continue;
                },
            };

            if c1 < 3 && c2 < 3{
                if board[c1][c2] == 0 {
                    board[c1][c2] = 1;
                } else {
                    println!("Somebody already made that move");
                    continue;
                }
            }
            else {
                println!("Please enter coordinates between 0 and 2");
                continue;
            }
            draw(&board);
            print!("\n");
            turn += 1;

            let state = static_eval(&board);
            if turn > 9 || state != 0{
                if state == 1 {
                    println!("You win!\n");
                }
                else if state == -1{
                    println!("You lose!\n");
                }
                else {
                    println!("Tie!\n");
                }
                break;
            }

            let ai_move = evaluate(&board, &turn);
            board[ai_move.0][ai_move.1] = -1;
            println!("\t    The AI marks ({}, {}):", ai_move.0, ai_move.1);
            draw_tabbed(&board);
            turn += 1;

            let state = static_eval(&board);
            if turn > 9 || state != 0{
                if state == 1 {
                    println!("You win!\n");
                }
                else if state == -1{
                    println!("You lose!\n");
                }
                else {
                    println!("Tie!\n");
                }
                break;
            }
        }
        if !confirm("Enter y to play again, or any other key to exit: "){
            break;
        }
    }    
}

fn confirm(message: &str) -> bool{
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut reponse = String::new();
    io::stdin()
        .read_line(&mut reponse)
        .expect("Failed to read line");
    let reponse = reponse.trim();
    if reponse == "y" || reponse == "Y" {
        return true;
    }
    false
}

fn draw(board: &[[i8; 3]; 3]){
    println!("  0  1  2");
    println!("  -------");
    for i in 0..3 {
        print!("{}", i);
        for j in 0..3 {
            if board[i][j] == 1 {
                print!(" X ");
            } else if board[i][j] == -1 {
                print!(" O ");
            } else {
                print!("   ");
            }
        }
        print!("\n");
    }
    println!("  -------");
}

fn draw_tabbed(board: &[[i8; 3]; 3]){
    println!("\t\t  0  1  2");
    println!("\t\t  -------");
    for i in 0..3 {
        print!("\t\t{}", i);
        for j in 0..3 {
            if board[i][j] == 1 {
                print!(" X ");
            } else if board[i][j] == -1 {
                print!(" O ");
            } else {
                print!("   ");
            }
        }
        print!("\n");
    }
    println!("\t\t  -------");
}

fn evaluate(board: &[[i8; 3]; 3], turn: &i8) -> (usize, usize) {
    let mut min = 100;
    let mut best: (usize, usize) = (0, 0);
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 0 {
                let mut board_next = *board;
                board_next[i][j] = -1;
                let eval = minimax(board_next, 9-turn, true);
                if eval < min {
                    min = eval;
                    best = (i, j);
                }
            }
        }
    }
    best
}

/*fn debug_evaluate(board: &[[i8; 3]; 3], turn: &i8) -> (usize, usize) {
    println!(" -------");
    let mut min = 100;
    let mut best: (usize, usize) = (0, 0);
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 1 {
                print!(" X ");
            } else if board[i][j] == -1 {
                print!(" O ");
            } else {
                let mut board_next = *board;
                board_next[i][j] = -1;
                let eval = minimax(board_next, 9-turn, true);
                if eval < min {
                    min = eval;
                    best = (i, j);
                }
                if eval < 0 {
                    print!("{} ", eval);
                }
                else{
                    print!(" {} ", eval);
                }
            }
        }
        print!("\n");
    }
    println!(" -------");
    best
}*/

fn minimax(board: [[i8; 3]; 3], depth: i8, maximizing: bool) -> i8 {
    let eval = static_eval(&board);
    if depth == 0 || eval != 0{
        if depth != 0{
            return eval*depth;  
        } else {
            return eval;
        }       
    }
    if maximizing {
        let mut max_eval = -100;
        //for each possible next move
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    let mut board_next = board;
                    board_next[i][j] = 1;
                    max_eval = std::cmp::max(minimax(board_next, depth-1, false), max_eval);
                }
            }
        }
        return max_eval;
    } else {
        let mut min_eval = 100;
        //for each possible next move
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    let mut board_next = board;
                    board_next[i][j] = -1;
                    min_eval = std::cmp::min(minimax(board_next, depth-1, true), min_eval);
                }
            }
        }
        return min_eval;
    }
}

fn static_eval(board: &[[i8; 3]; 3]) -> i8 {
    //check row sums
    for i in 0..3 {
        let mut sum = 0;
        for j in 0..3 {
            sum += board[i][j];
        }
        if sum == 3 {
            return 1;
        }
        else if sum == -3 {
            return -1;
        }
    }
    //check col sums
    for j in 0..3 {
        let mut sum = 0;
        for i in 0..3 {
            sum += board[i][j];
        }
        if sum == 3 {
            return 1;
        }
        else if sum == -3 {
            return -1;
        }
    }
    //check diagonals
    let diag = board[0][0] + board[1][1] + board[2][2];
    if diag == 3 {
        return 1;
    } else if diag == -3 {
        return -1;
    }
    let anti_diag = board[0][2] + board[1][1] + board[2][0];
    if anti_diag == 3 {
        return 1;
    } else if anti_diag == -3 {
        return -1;
    }
    //return 0 if no winner
    0
}