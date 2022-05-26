use message_io::network::{NetEvent, Transport};
use message_io::node::{self};
use std::env::{self};

mod gameboard;
use crate::gameboard::Gameboard;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Usage: mini-risk <num players> <num territories per player> <num dice per player>");
    }

    let num_players: u32 = args[1]
        .parse()
        .expect("num players must be a positive integer");
    let num_territories_per_player: u32 = args[2]
        .parse()
        .expect("num player territories per player must be a positive integer");
    let num_dice_per_player: u32 = args[3]
        .parse()
        .expect("num dice per player must be a positive integer");

    if num_dice_per_player < num_territories_per_player {
        panic!("The number of dice per player must be greater than or equal to the number of territories 
                per player");
    }

    let mut gameboard: Gameboard =
        Gameboard::new(num_players, num_territories_per_player, num_dice_per_player);

    let mut gameboard_json = serde_json::to_string_pretty(&gameboard).unwrap();
    println!("{}", gameboard_json);
    //print!("{:#?}", gameboard);

    let (handler, listener) = node::split::<()>();
    handler
        .network()
        .listen(Transport::Tcp, "0.0.0.0:1234")
        .unwrap();

    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => unreachable!(),
        NetEvent::Accepted(_endpoint, _listener) => println!("Client connected"),
        NetEvent::Message(endpoint, data) => {
            let incoming_message: String = String::from_utf8(data.to_vec()).unwrap();
            println!("Received: {:?}", String::from_utf8(data.to_vec()));
            let split_message: Vec<&str> = incoming_message.split(":").collect();

            match split_message[0] {
                "Connect" => {
                    println!("Got a Connect message");
                    handler.network().send(endpoint, gameboard_json.as_bytes());
                }
                "Attack" => {
                    println!("Got an Attack message");
                    if split_message.len() >= 3 {
                        let attack_source: u32 = split_message[1].parse::<u32>().unwrap();
                        let attack_target: u32 = split_message[2].parse::<u32>().unwrap();
                        if gameboard.can_attack(attack_source, attack_target) {
                            gameboard.attack(attack_source, attack_target);
                            gameboard_json = serde_json::to_string_pretty(&gameboard).unwrap();
                            handler.network().send(endpoint, gameboard_json.as_bytes());
                        } else {
                            // TODO: reply with error
                        }
                    } else {
                        println!("Malformed Attack message");
                    }
                }
                _ => println!("Got an unknown message"),
            }
        }
        NetEvent::Disconnected(_endpoint) => println!("Client disconnected"),
    });
}
