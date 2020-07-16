use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, channel};

#[derive(Serialize, Deserialize)]
struct Game {
    player: i8, //-1 == O, 1 == X
    winner: i8, //-1 == O, 0 == no winner, 1 == X, 2 == tie
    board: [[i8; 3]; 3]
}

fn main() {
    #![allow(non_snake_case)]
    //thread channels
    let (GM1_tx, P1_rx) = channel(); //GM to P1
    let (GM2_tx, P2_rx) = channel(); //GM to P2
    let (P1_tx, GM_rx) = channel(); //P1 to GM
    let P2_tx = mpsc::Sender::clone(&P1_tx); //P2 to GM

    //wrap in option so take can ensure single use
    let mut P1_rx = Some(P1_rx);
    let mut P2_rx = Some(P2_rx);
    let mut P1_tx = Some(P1_tx);
    let mut P2_tx = Some(P2_tx);

    let mut active_players: u8 = 0;

    //GM thread
    let gm_handle = thread::spawn(move|| {
        let mut game = Game {player: 1, winner: 0, board: [[0; 3]; 3]};//initial state
        while game.winner == 0{
            game.player = 1;
            GM1_tx.send(game).unwrap();
            game = GM_rx.recv().unwrap();
            game.player = -1;
            if game.winner != 0 {
                GM2_tx.send(game).unwrap();
                //sleep to make sure final game state sent before terminating
                //^^really should probably just add handles to all threads and join
                thread::sleep(std::time::Duration::from_millis(500));
                break;
            }
            GM2_tx.send(game).unwrap();
            game = GM_rx.recv().unwrap();
        }
    });

    let mut port = String::new();
    print!("Enter host port: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut port)
        .expect("Failed to read line");
    let address = String::from("0.0.0.0:") + &port.trim();
    println!("Hosting on {}", address);
    let listener = TcpListener::bind(&address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                if active_players == 0 {
                    let rx = P1_rx.take().unwrap();
                    let tx = P1_tx.take().unwrap();
                    thread::spawn(move|| {
                        println!("Spawning P1");
                        handle_client(stream, rx, tx);
                    });
                    active_players += 1;
                }
                else if active_players == 1 {
                    let rx = P2_rx.take().unwrap();
                    let tx = P2_tx.take().unwrap();
                    thread::spawn(move|| {
                        println!("Spawning P2");
                        handle_client(stream, rx, tx);
                    });
                    break;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

    gm_handle.join().unwrap();
    println!("Closing server");
    // close the socket server
    drop(listener);
}

fn handle_client(stream: TcpStream, old_state: mpsc::Receiver<Game>, new_state: mpsc::Sender<Game>) {
    loop{
        let game = old_state.recv().unwrap(); //wait till gets board and update
        serde_json::ser::to_writer(&stream, &game).unwrap(); //send game to player

        //terminate connection if game over
        if game.winner != 0 {
            break;
        }

        let mut de = serde_json::Deserializer::from_reader(&stream); //receive updated board
        let game = Game::deserialize(&mut de);
        match game {
            Ok(game) => {
                draw(&game.board);
                //terminate connection if won
                if game.winner != 0 {
                    new_state.send(game).unwrap();
                    break;
                }
                new_state.send(game).unwrap();
                continue;
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
                //println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            }
        }
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