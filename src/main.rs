use std::env::{self};
use message_io::node::{self};
use message_io::network::{NetEvent, Transport};

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

    let gameboard: Gameboard =
        Gameboard::new(num_players, num_territories_per_player, num_dice_per_player);

    let gameboard_json = serde_json::to_string_pretty(&gameboard).unwrap();
    println!("{}", gameboard_json);
    //print!("{:#?}", gameboard);

    let (handler, listener) = node::split::<()>();
    handler.network().listen(Transport::Tcp, "0.0.0.0:1234").unwrap();

    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_,_) => unreachable!(), 
        NetEvent::Accepted(_endpoint, _listener) => println!("Client connected"),
        NetEvent::Message(endpoint, data) => {
            println!("Received: {:?}", String::from_utf8(data.to_vec()));
            handler.network().send(endpoint, gameboard_json.as_bytes());
        },
        NetEvent::Disconnected(_endpoint) => println!("Client disconnected"),
    });
}
