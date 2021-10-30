use std::env;
use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Territory {
    id: u32,
    num_dice: u32,
    owner_id: u32,
}

fn generate_gameboard(num_players: u32, num_territories_per_player: u32, num_dice_per_player: u32) -> HashMap<u32, Territory> {
    
    let max_territories = num_territories_per_player * num_players;

    let mut territory_map: HashMap<u32, Territory> = HashMap::new();

    for cur_id in 0..(max_territories) {
        let cur_territory = Territory {
            id: cur_id,
            num_dice: 1,
            owner_id: 0
        };

        territory_map.insert(cur_territory.id, cur_territory);
    }

    assign_territories_to_players(&mut territory_map, num_players, num_territories_per_player);

    assing_dice_to_territories(&mut territory_map, num_players, num_territories_per_player, num_dice_per_player);

    println!("{:#?}", territory_map);

    territory_map
}

fn assign_territories_to_players(territory_map: &mut HashMap<u32, Territory>, num_players: u32, num_territories_per_player: u32){
    let max_territories = num_territories_per_player * num_players;

    let mut territory_ids: Vec<u32> = (0..max_territories).collect();

    let mut rng = thread_rng();
    territory_ids.shuffle(&mut rng);
    
    let mut cur_player_id: u32 = 0;
    for cur_territory in territory_ids {
        cur_player_id = cur_player_id % num_players;
        let mut found_terr: &mut Territory = territory_map.get_mut(&cur_territory).unwrap();
        found_terr.owner_id = cur_player_id;
        cur_player_id += 1;
    }
}

fn assing_dice_to_territories(territory_map: &mut HashMap<u32, Territory>, num_players:u32, num_territories_per_player: u32, num_dice_per_player: u32)
{
    let mut rng = thread_rng();

    for cur_player_id in 0..num_players {
        let mut assigned_dice: u32 = 0;
        let mut cur_players_territories: Vec<u32> = Vec::new();

        for cur_terr in territory_map.values() {
            if cur_terr.owner_id == cur_player_id {
                cur_players_territories.push(cur_terr.id);
            }
        }

        while assigned_dice < (num_dice_per_player - num_territories_per_player) {
            cur_players_territories.shuffle(&mut rng);
            let mut found_terr: &mut Territory = territory_map.get_mut(&cur_players_territories[0]).unwrap();
            found_terr.num_dice += 1;
            assigned_dice += 1;
        }
    }
}

fn main() {

    let args : Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Usage: mini-risk <num players> <num territories per player> <num dice per player>");
    }

    let num_players: u32 = args[1].parse().expect("num players must be a positive integer");
    let num_territories_per_player: u32 = args[2].parse().expect("num player territories per player must be a positive integer");
    let num_dice_per_player: u32 = args[3].parse().expect("num dice per player must be a positive integer");

    let all_territories: HashMap<u32, Territory> = generate_gameboard(num_players, num_territories_per_player, num_dice_per_player);
    
}
