use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize)]
pub struct Territory {
    pub id: u32,
    pub num_dice: u32,
    pub owner_id: u32,
    pub neighbors: Vec<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Gameboard {
    pub territory_map: HashMap<u32, Territory>,
}

impl Gameboard {
    pub fn new(
        num_players: u32,
        num_territories_per_player: u32,
        num_dice_per_player: u32,
    ) -> Gameboard {
        let max_territories = num_territories_per_player * num_players;

        let mut territory_map: HashMap<u32, Territory> = HashMap::new();

        for cur_id in 0..(max_territories) {
            let cur_territory = Territory {
                id: cur_id,
                num_dice: 1,
                owner_id: 0,
                neighbors: Vec::new(),
            };

            territory_map.insert(cur_territory.id, cur_territory);
        }

        assign_territories_to_players(&mut territory_map, num_players, num_territories_per_player);

        assign_dice_to_territories(
            &mut territory_map,
            num_players,
            num_territories_per_player,
            num_dice_per_player,
        );

        connect_territories(&mut territory_map);

        while !is_connected(&territory_map) || !verify_neighbors(&territory_map) {
            clear_neighbors(&mut territory_map);
            connect_territories(&mut territory_map);
        }

        Gameboard {
            territory_map: territory_map,
        }
    }

    pub fn can_attack(&self, attack_from: u32, attack_to: u32) -> bool {
        let mut result: bool = false;
        if self.territory_map.contains_key(&attack_from)
            && self.territory_map.contains_key(&attack_to)
        {
            let attacker: &Territory = self.territory_map.get(&attack_from).unwrap();
            let attacked: &Territory = self.territory_map.get(&attack_to).unwrap();

            if attacker.neighbors.contains(&attacked.id) {
                result = true;
            } else {
                result = false;
                println!("Cannot attack: Selected territories are not neighbors")
            }

            if result && attacker.owner_id != attacked.owner_id {
                result = true;
            } else {
                result = false;
                println!("Cannot attack: Selected territories are owned by the same player")
            }
            if result && attacker.num_dice > 1 {
                result = true;
            } else {
                result = false;
                println!("Cannot attack: Attacking territory has less than 1 die")
            }
        }

        return result;
    }

    pub fn attack(&mut self, attack_from: u32, attack_to: u32) {
        let attacker: &Territory = self.territory_map.get(&attack_from).unwrap();
        let defender: &Territory = self.territory_map.get(&attack_to).unwrap();

        let attack_dice: u32 = attacker.num_dice - 1;
        let attacker_owner: u32 = attacker.owner_id;

        if battle(attacker.num_dice, defender.num_dice) {
            {
                let mut attacker: &mut Territory =
                    self.territory_map.get_mut(&attack_from).unwrap();
                attacker.num_dice = 1;
            }
            {
                let mut defender: &mut Territory = self.territory_map.get_mut(&attack_to).unwrap();
                defender.num_dice += attack_dice;
                defender.owner_id = attacker_owner;
            }
        } else {
            {
                let mut attacker: &mut Territory =
                    self.territory_map.get_mut(&attack_from).unwrap();
                attacker.num_dice = 1;
            }
        }
    }
}

fn assign_territories_to_players(
    territory_map: &mut HashMap<u32, Territory>,
    num_players: u32,
    num_territories_per_player: u32,
) {
    let max_territories = num_territories_per_player * num_players;

    let mut territory_ids: Vec<u32> = (0..max_territories).collect();

    let mut rng = thread_rng();
    territory_ids.shuffle(&mut rng);

    let mut cur_player_id: u32 = 0;
    for cur_territory in territory_ids {
        cur_player_id = cur_player_id % num_players;
        let mut this_terr: &mut Territory = territory_map.get_mut(&cur_territory).unwrap();
        this_terr.owner_id = cur_player_id;
        cur_player_id += 1;
    }
}

fn assign_dice_to_territories(
    territory_map: &mut HashMap<u32, Territory>,
    num_players: u32,
    num_territories_per_player: u32,
    num_dice_per_player: u32,
) {
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
            let mut this_terr: &mut Territory =
                territory_map.get_mut(&cur_players_territories[0]).unwrap();
            this_terr.num_dice += 1;
            assigned_dice += 1;
        }
    }
}

fn connect_territories(territory_map: &mut HashMap<u32, Territory>) {
    let mut all_terr_ids: Vec<u32> = Vec::new();
    for cur_terr_id in territory_map.keys() {
        all_terr_ids.push(*cur_terr_id);
    }

    all_terr_ids.sort();
    let min: u32 = all_terr_ids[0];
    let max: u32 = all_terr_ids[all_terr_ids.len() - 1];

    for curr_terr_id in all_terr_ids {
        let mut other_terr_id: u32 = curr_terr_id;

        while other_terr_id == curr_terr_id {
            other_terr_id = rand::thread_rng().gen_range(min..=max);
        }

        let this_terr: &mut Territory = territory_map.get_mut(&curr_terr_id).unwrap();
        if !this_terr.neighbors.contains(&other_terr_id) {
            this_terr.neighbors.push(other_terr_id);
        }

        let other_terr: &mut Territory = territory_map.get_mut(&other_terr_id).unwrap();
        if !other_terr.neighbors.contains(&curr_terr_id) {
            other_terr.neighbors.push(curr_terr_id);
        }
    }
}

fn clear_neighbors(territory_map: &mut HashMap<u32, Territory>) {
    let mut all_terr_ids: Vec<u32> = Vec::new();

    for curr_id in territory_map.keys() {
        all_terr_ids.push(*curr_id);
    }

    for curr_id in all_terr_ids {
        let this_terr: &mut Territory = territory_map.get_mut(&curr_id).unwrap();
        this_terr.neighbors.clear();
    }
}

fn is_connected(territory_map: &HashMap<u32, Territory>) -> bool {
    let mut visited: HashMap<u32, bool> = HashMap::new();

    for cur_terr in territory_map.values() {
        visited.insert(cur_terr.id, false);
    }

    let mut comp_num = 0;

    for cur_terr_id in territory_map.keys() {
        if !visited[cur_terr_id] {
            comp_num += 1;

            let mut queue: VecDeque<u32> = VecDeque::new();
            queue.push_back(*cur_terr_id);
            visited.insert(*cur_terr_id, true);

            while queue.len() > 0 {
                let w: u32 = queue.pop_front().unwrap();

                if territory_map.contains_key(&(w as u32)) {
                    let cur_territory = &territory_map[&(w as u32)];

                    for cur_neighbor in &cur_territory.neighbors {
                        if !visited[cur_neighbor] {
                            visited.insert(*cur_neighbor, true);
                            queue.push_back(*cur_neighbor);
                        }
                    }
                }
            }
        }
    }

    return comp_num == 1;
}

fn verify_neighbors(territory_map: &HashMap<u32, Territory>) -> bool {
    let mut result: bool = true;

    for cur_terr in territory_map.values() {
        for cur_neighbor in &cur_terr.neighbors {
            if territory_map.contains_key(cur_neighbor) {
                let temp_neighbor: &Territory = territory_map.get(cur_neighbor).unwrap();

                if !temp_neighbor.neighbors.contains(&cur_terr.id) {
                    result = false;
                    break;
                }
            }
        }
    }
    return result;
}

fn battle(num_dice_attacker: u32, num_dice_defender: u32) -> bool {
    let mut attacker_sum: u32 = 0;
    let mut attacked_sum: u32 = 0;

    let mut die_roll = rand::thread_rng();

    for _cur_dice in 0..(num_dice_attacker - 1) {
        attacker_sum += die_roll.gen_range(1..7);
    }

    for _cur_dice in 0..(num_dice_defender - 1) {
        attacked_sum += die_roll.gen_range(1..7);
    }

    return attacker_sum > attacked_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_gameboard() {
        let num_players: u32 = 2;
        let num_territories_per_player: u32 = 2;
        let num_dice_per_player: u32 = 10;

        let sut_gameboard: crate::gameboard::Gameboard =
            Gameboard::new(num_players, num_territories_per_player, num_dice_per_player);

        for curr_player in 0..num_players {
            let mut sum_dice: u32 = 0;
            let mut sum_terr: u32 = 0;

            for cur_terr in sut_gameboard.territory_map.values() {
                if cur_terr.owner_id == curr_player {
                    sum_dice += cur_terr.num_dice;
                    sum_terr += 1;
                }
            }

            assert_eq!(num_territories_per_player, sum_terr);
            assert_eq!(num_dice_per_player, sum_dice);
        }
    }

    #[test]
    fn is_connected_one_node() {
        let terr_one = Territory {
            id: 1,
            num_dice: 1,
            owner_id: 0,
            neighbors: Vec::new(),
        };

        let mut territory_map: HashMap<u32, Territory> = HashMap::new();
        territory_map.insert(1, terr_one);

        assert_eq!(true, is_connected(&territory_map));
    }

    #[test]
    fn is_connected_two_nodes() {
        let terr_one = Territory {
            id: 1,
            num_dice: 1,
            owner_id: 0,
            neighbors: vec![2],
        };

        let terr_two = Territory {
            id: 2,
            num_dice: 1,
            owner_id: 0,
            neighbors: vec![1],
        };

        let mut territory_map: HashMap<u32, Territory> = HashMap::new();
        territory_map.insert(1, terr_one);
        territory_map.insert(2, terr_two);

        assert_eq!(true, is_connected(&territory_map));
    }

    #[test]
    fn is_connected_two_nodes_not_connected() {
        let terr_one = Territory {
            id: 1,
            num_dice: 1,
            owner_id: 0,
            neighbors: Vec::new(),
        };

        let terr_two = Territory {
            id: 2,
            num_dice: 1,
            owner_id: 0,
            neighbors: Vec::new(),
        };

        let mut territory_map: HashMap<u32, Territory> = HashMap::new();
        territory_map.insert(1, terr_one);
        territory_map.insert(2, terr_two);

        assert_eq!(false, is_connected(&territory_map));
    }

    #[test]
    fn is_connected_three_nodes_none_connected() {
        let terr_one = Territory {
            id: 1,
            num_dice: 1,
            owner_id: 0,
            neighbors: Vec::new(),
        };

        let terr_two = Territory {
            id: 2,
            num_dice: 1,
            owner_id: 0,
            neighbors: Vec::new(),
        };

        let terr_three = Territory {
            id: 3,
            num_dice: 1,
            owner_id: 0,
            neighbors: Vec::new(),
        };

        let mut territory_map: HashMap<u32, Territory> = HashMap::new();
        territory_map.insert(1, terr_one);
        territory_map.insert(2, terr_two);
        territory_map.insert(3, terr_three);

        assert_eq!(false, is_connected(&territory_map));
    }

    #[test]
    fn is_connected_three_nodes_one_island() {
        let terr_one = Territory {
            id: 1,
            num_dice: 1,
            owner_id: 0,
            neighbors: vec![2],
        };

        let terr_two = Territory {
            id: 2,
            num_dice: 1,
            owner_id: 0,
            neighbors: vec![1],
        };

        let terr_three = Territory {
            id: 3,
            num_dice: 1,
            owner_id: 0,
            neighbors: Vec::new(),
        };

        let mut territory_map: HashMap<u32, Territory> = HashMap::new();
        territory_map.insert(1, terr_one);
        territory_map.insert(2, terr_two);
        territory_map.insert(3, terr_three);

        assert_eq!(false, is_connected(&territory_map));
    }

    #[test]
    fn is_connected_three_nodes() {
        let terr_one = Territory {
            id: 1,
            num_dice: 1,
            owner_id: 0,
            neighbors: vec![2, 3],
        };

        let terr_two = Territory {
            id: 2,
            num_dice: 1,
            owner_id: 0,
            neighbors: vec![1, 3],
        };

        let terr_three = Territory {
            id: 3,
            num_dice: 1,
            owner_id: 0,
            neighbors: vec![1, 2],
        };

        let mut territory_map: HashMap<u32, Territory> = HashMap::new();
        territory_map.insert(1, terr_one);
        territory_map.insert(2, terr_two);
        territory_map.insert(3, terr_three);

        assert_eq!(true, is_connected(&territory_map));
    }

    #[test]
    fn test_clear_neighbors() {
        let terr_one = Territory {
            id: 1,
            num_dice: 1,
            owner_id: 0,
            neighbors: vec![2, 3],
        };

        let terr_two = Territory {
            id: 2,
            num_dice: 1,
            owner_id: 0,
            neighbors: vec![1, 3],
        };

        let mut territory_map: HashMap<u32, Territory> = HashMap::new();
        territory_map.insert(1, terr_one);
        territory_map.insert(2, terr_two);

        clear_neighbors(&mut territory_map);

        for cur_terr in territory_map.values() {
            assert_eq!(0, cur_terr.neighbors.len());
        }
    }
}

#[test]
fn verify_neighbors_one_terr() {
    let terr_one = Territory {
        id: 1,
        num_dice: 1,
        owner_id: 0,
        neighbors: Vec::new(),
    };

    let mut territory_map: HashMap<u32, Territory> = HashMap::new();
    territory_map.insert(1, terr_one);

    assert_eq!(true, verify_neighbors(&territory_map));
}

#[test]
fn verify_neighbors_two_terrs_good() {
    let terr_one = Territory {
        id: 1,
        num_dice: 1,
        owner_id: 0,
        neighbors: vec![2],
    };

    let terr_two = Territory {
        id: 2,
        num_dice: 1,
        owner_id: 0,
        neighbors: vec![1],
    };

    let mut territory_map: HashMap<u32, Territory> = HashMap::new();
    territory_map.insert(1, terr_one);
    territory_map.insert(2, terr_two);

    assert_eq!(true, verify_neighbors(&territory_map));
}

#[test]
fn verify_neighbors_two_terrs_bad() {
    let terr_one = Territory {
        id: 1,
        num_dice: 1,
        owner_id: 0,
        neighbors: vec![2],
    };

    let terr_two = Territory {
        id: 2,
        num_dice: 1,
        owner_id: 0,
        neighbors: Vec::new(),
    };

    let mut territory_map: HashMap<u32, Territory> = HashMap::new();
    territory_map.insert(1, terr_one);
    territory_map.insert(2, terr_two);

    assert_eq!(false, verify_neighbors(&territory_map));
}

#[test]
fn verify_neighbors_three_terrs_good() {
    let terr_one = Territory {
        id: 1,
        num_dice: 1,
        owner_id: 0,
        neighbors: vec![2],
    };

    let terr_two = Territory {
        id: 2,
        num_dice: 1,
        owner_id: 0,
        neighbors: vec![1, 3],
    };

    let terr_three = Territory {
        id: 3,
        num_dice: 1,
        owner_id: 0,
        neighbors: vec![2],
    };

    let mut territory_map: HashMap<u32, Territory> = HashMap::new();
    territory_map.insert(1, terr_one);
    territory_map.insert(2, terr_two);
    territory_map.insert(3, terr_three);

    assert_eq!(true, verify_neighbors(&territory_map));
}

#[test]
fn verify_neighbors_three_terrs_bad() {
    let terr_one = Territory {
        id: 1,
        num_dice: 1,
        owner_id: 0,
        neighbors: vec![2],
    };

    let terr_two = Territory {
        id: 2,
        num_dice: 1,
        owner_id: 0,
        neighbors: vec![1, 3],
    };

    let terr_three = Territory {
        id: 3,
        num_dice: 1,
        owner_id: 0,
        neighbors: vec![1, 2],
    };

    let mut territory_map: HashMap<u32, Territory> = HashMap::new();
    territory_map.insert(1, terr_one);
    territory_map.insert(2, terr_two);
    territory_map.insert(3, terr_three);

    assert_eq!(false, verify_neighbors(&territory_map));
}
