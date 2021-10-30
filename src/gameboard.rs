//pub mod gameboard {

    use std::collections::HashMap;
    use rand::thread_rng;
    use rand::prelude::SliceRandom;
    
    #[derive(Debug)]
    struct Territory {
        id: u32,
        num_dice: u32,
        owner_id: u32,
    }

    pub struct Gameboard {
        territory_map: HashMap<u32, Territory>,
    }

    impl Gameboard {
        pub fn new(num_players: u32, num_territories_per_player: u32, num_dice_per_player: u32) -> Gameboard {
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

            assign_dice_to_territories(&mut territory_map, num_players, num_territories_per_player, num_dice_per_player);

            println!("{:#?}", territory_map);

            Gameboard {
                territory_map: territory_map,
            }    
        }
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

    fn assign_dice_to_territories(territory_map: &mut HashMap<u32, Territory>, num_players:u32, num_territories_per_player: u32, num_dice_per_player: u32)
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
//}