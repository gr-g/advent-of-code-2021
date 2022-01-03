use advent_of_code_2021::graph::Graph;

fn home(amphipod: char) -> usize {
    match amphipod {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => panic!(),
    }
}

fn energy(amphipod: char) -> usize {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

struct Burrow {
    room_positions: Vec<usize>, // x coordinates of the home rooms
    room_depth: usize, // depth of each room
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct BurrowState {
    hallway: [Option<char>; 11],
    rooms: [Vec<char>; 11],
}

impl BurrowState {
    fn create_from(s: &str) -> Self {
        let hallway = [None; 11];
        let mut rooms: [Vec<char>; 11] = Default::default();

        let mut lines = s.lines();
        assert_eq!(lines.next(), Some("#############"));
        assert_eq!(lines.next(), Some("#...........#"));

        for l in lines {
            for (x, c) in l.char_indices() {
                if x > 0 && x < 12 && c.is_ascii_uppercase() {
                    rooms[x-1].insert(0, c);
                }
            }
        }

        BurrowState { hallway, rooms }
    }
}

impl Graph<BurrowState> for Burrow {
    fn edges(&self, node: &BurrowState) -> Vec<(BurrowState, usize)> {
        let mut v = vec![];

        for &x in &self.room_positions {
            let amphipods_in_room = node.rooms[x].len();
            if amphipods_in_room > 0 && (0..amphipods_in_room).any(|a| home(node.rooms[x][a]) != x) {
                for direction in [-1, 1] {
                    // Consider moving the top amphipod out of the room and to the left
                    // (direction = -1) or to the right (direction = 1).
                    let mut new_x = x;
                    let mut steps = self.room_depth - amphipods_in_room + 1;
                    loop {
                        if node.hallway[new_x].is_some() {
                            break;
                        }
                        // Move only to positions that are not directly above a room.
                        if !self.room_positions.contains(&new_x) {
                            let mut new_node = node.clone();
                            let amphipod = new_node.rooms[x].pop().unwrap();
                            new_node.hallway[new_x] = Some(amphipod);
                            v.push((new_node, steps * energy(amphipod)));
                        }
                        if new_x == 0 || new_x == 10 {
                            break;
                        }
                        if direction == -1 { new_x -= 1 } else { new_x += 1 };
                        steps += 1;
                    }
                }
            }
        }

        for x in 0..11 {
            if let Some(amphipod) = node.hallway[x] {
                // Consider moving this amphipod from the hallway to its home.
                let direction = if x > home(amphipod) { -1 } else { 1 };
                let mut new_x = if direction == -1 { x - 1 } else { x + 1 };
                let mut steps = 1;
                loop {
                    if node.hallway[new_x].is_some() {
                        break;
                    }
                    if new_x == home(amphipod) {
                        // Move only if the home room has all correct amphipods inside.
                        if node.rooms[new_x].iter().all(|a| *a == amphipod) {
                            let mut new_node = node.clone();
                            new_node.hallway[x] = None;
                            new_node.rooms[new_x].push(amphipod);
                            let d = self.room_depth - new_node.rooms[new_x].len() + 1;
                            v.push((new_node, (steps + d) * energy(amphipod)));
                        }
                        break;
                    }
                    if new_x == 0 || new_x == 10 {
                        break;
                    }
                    if direction == -1 { new_x -= 1 } else { new_x += 1 };
                    steps += 1;
                }
            }
        }

        v
    }
}

fn solve(input: &str) -> (usize, usize) {
    let burrow = Burrow {
        room_positions: vec![home('A'), home('B'), home('C'), home('D')],
        room_depth: 2,
    };

    let start = BurrowState::create_from(input);

    let target = BurrowState::create_from("\
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
");

    let burrow_extra = Burrow {
        room_positions: vec![home('A'), home('B'), home('C'), home('D')],
        room_depth: 4,
    };

    let mut start_extra = start.clone();
    start_extra.rooms[home('A')].splice(1..1, ['D', 'D']);
    start_extra.rooms[home('B')].splice(1..1, ['B', 'C']);
    start_extra.rooms[home('C')].splice(1..1, ['A', 'B']);
    start_extra.rooms[home('D')].splice(1..1, ['C', 'A']);

    let mut target_extra = target.clone();
    target_extra.rooms[home('A')].splice(1..1, ['A', 'A']);
    target_extra.rooms[home('B')].splice(1..1, ['B', 'B']);
    target_extra.rooms[home('C')].splice(1..1, ['C', 'C']);
    target_extra.rooms[home('D')].splice(1..1, ['D', 'D']);

    let distances = burrow.shortest_paths(start);
    let distances_extra = burrow_extra.shortest_paths(start_extra);

    (*distances.get(&target).unwrap(), *distances_extra.get(&target_extra).unwrap())
}

fn main() {
    let input = std::fs::read_to_string("input/23.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(solve("\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"), (12521, 44169));
    }
}
