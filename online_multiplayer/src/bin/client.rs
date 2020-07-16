use std::net::{TcpStream};
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Game {
    player: i8, 
    winner: i8, //0 == no winner, -1 == O, 1 == X
    board: [[i8; 3]; 3]
}

fn main() {
    let mut address = String::new();
    print!("Enter host ip and port as \"ip:port\": ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut address)
        .expect("Failed to read line");
    let address = address.trim();
    println!("Connecting to {}", address);

    match TcpStream::connect(address) {
        Ok(stream) => {
            println!("Successfully connected to {}", address);
            handle_connection(stream);
            
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated connection.");
    print!("Press enter to exit");
    io::stdout().flush().unwrap();
    let mut dummy = String::new();
    io::stdin()
        .read_line(&mut dummy)
        .expect("Failed to read line");
}

fn handle_connection(stream: TcpStream){
    let mut eval = 0;
    while eval == 0 {
        let mut de = serde_json::Deserializer::from_reader(&stream);
        let game = Game::deserialize(&mut de);
        let mut game = match game {
            Ok(game) => {
                game
            },
            Err(e) => {
                println!("Error: {}", e);
                panic!();
            }
        };
        draw_tabbed(&game.board);
        if game.winner == -game.player { //if other player won
            println!("You lose!");
            break;
        }
        else if game.winner == 2 {
            println!("Tie!");
            break;
        }
        println!("Your turn");

        //get user coordinates and mark board
        loop {
            print!("Enter coordinate pair \"ij\": ");
            io::stdout().flush().unwrap();
            let mut coords = String::new();
            io::stdin()
                .read_line(&mut coords)
                .expect("Failed to read line");
            if coords.trim().chars().count() > 2{
                println!("Input too long");
                continue;
            }
            let c1: usize = match coords[0..1].trim().parse() {
                Ok(num) => {
                    num
                }
                Err(num) => {
                    println!("c1: {}", num);
                    println!("Enter positive integers only");
                    continue;
                },
            };
            let c2: usize = match coords[1..2].trim().parse() {
                Ok(num) => {
                    num
                }
                Err(num) => {
                    println!("c2: {}", num);
                    println!("Enter positive integers only");
                    continue;
                },
            };
            if c1 < 3 && c2 < 3{
                if game.board[c1][c2] == 0 {
                    game.board[c1][c2] = game.player;
                } else {
                    println!("Somebody already made that move");
                    continue;
                }
            }
            else {
                println!("Please enter coordinates between 0 and 2");
                continue;
            }
            break; //exit loop if everything done correctly
        }
        draw(&game.board);
        eval = static_eval(&game.board);
        if eval == 0 {
            println!("\tWaiting for other player's input");
        }
        else if eval == game.player {
            println!("You win!");
            game.winner = game.player;
        }
        else {
            println!("Tie!");
            game.winner = 2;
        }
        serde_json::ser::to_writer(&stream, &game).unwrap();
    }
}

pub fn draw(board: &[[i8; 3]; 3]){
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
pub fn draw_tabbed(board: &[[i8; 3]; 3]){
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
pub fn static_eval(board: &[[i8; 3]; 3]) -> i8 {
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
    //check if any remaining moves
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 0 {
                //return no winner if move available
                return 0;
            }
        }
    }
    //return 2 if no moves available
    2
}