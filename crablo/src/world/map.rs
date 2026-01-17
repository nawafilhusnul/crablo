use crate::core::constants::MAP_SIZE;
use macroquad::rand::gen_range;

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

#[derive(Clone, Copy)]
struct Room {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Room {
    fn center(&self) -> (usize, usize) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }

    fn intersects(&self, other: &Room) -> bool {
        // Allow rooms to overlap/merge for more open space
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

pub fn create_map() -> (
    [[Tile; MAP_SIZE]; MAP_SIZE],
    Vec<(usize, usize)>,
    Vec<(usize, usize, u8)>,
) {
    let mut map = [[Tile::Wall; MAP_SIZE]; MAP_SIZE];
    let mut rooms: Vec<Room> = vec![];

    let room_attempts = 8;
    let min_room_size = 4;
    let max_room_size = 7;

    // Generate rooms
    for _ in 0..room_attempts {
        let w = gen_range(min_room_size, max_room_size);
        let h = gen_range(min_room_size, max_room_size);
        let x = gen_range(1, MAP_SIZE - w - 1);
        let y = gen_range(1, MAP_SIZE - h - 1);

        let new_room = Room { x, y, w, h };

        let mut ok = true;
        for room in &rooms {
            if new_room.intersects(room) {
                ok = false;
                break;
            }
        }

        if ok {
            // Carve out the room
            for ry in new_room.y..new_room.y + new_room.h {
                for rx in new_room.x..new_room.x + new_room.w {
                    map[ry][rx] = Tile::Floor;
                }
            }

            // Connect to previous room with corridors
            if !rooms.is_empty() {
                let (new_cx, new_cy) = new_room.center();
                let (prev_cx, prev_cy) = rooms.last().unwrap().center();

                // Randomly choose horizontal-first or vertical-first
                if gen_range(0, 2) == 0 {
                    carve_h_corridor(&mut map, prev_cx, new_cx, prev_cy);
                    carve_v_corridor(&mut map, prev_cy, new_cy, new_cx);
                } else {
                    carve_v_corridor(&mut map, prev_cy, new_cy, prev_cx);
                    carve_h_corridor(&mut map, prev_cx, new_cx, new_cy);
                }
            }

            rooms.push(new_room);
        }
    }

    // Ensure we have at least 2 rooms
    if rooms.len() < 2 {
        return create_map(); // Retry
    }

    // Place gold in random rooms (not the first room where player spawns)
    let mut gold_positions = vec![];
    for room in rooms.iter().skip(1) {
        if gen_range(0, 3) < 2 {
            // 66% chance
            let gx = gen_range(room.x + 1, room.x + room.w - 1);
            let gy = gen_range(room.y + 1, room.y + room.h - 1);
            gold_positions.push((gx, gy));
        }
    }

    // Place monsters in rooms (not the first room)
    // Returns (x, y, monster_type) where type: 0=normal, 1=fast, 2=tank
    let mut monster_positions = vec![];
    for room in rooms.iter().skip(1) {
        let num_monsters = gen_range(1, 3);
        for _ in 0..num_monsters {
            let mx = gen_range(room.x, room.x + room.w);
            let my = gen_range(room.y, room.y + room.h);
            if !monster_positions
                .iter()
                .any(|(x, y, _)| *x == mx && *y == my)
            {
                // Randomly choose monster type: 60% normal, 25% fast, 15% tank
                let roll = gen_range(0, 100);
                let mtype: u8 = if roll < 60 {
                    0
                } else if roll < 85 {
                    1
                } else {
                    2
                };
                monster_positions.push((mx, my, mtype));
            }
        }
    }

    // Return player spawn position (center of first room)
    (map, gold_positions, monster_positions)
}

pub fn get_player_spawn(map: &[[Tile; MAP_SIZE]; MAP_SIZE]) -> (usize, usize) {
    // Find first floor tile (should be in first room)
    for y in 1..MAP_SIZE - 1 {
        for x in 1..MAP_SIZE - 1 {
            if map[y][x] == Tile::Floor {
                return (x, y);
            }
        }
    }
    (2, 2) // Fallback
}

fn carve_h_corridor(map: &mut [[Tile; MAP_SIZE]; MAP_SIZE], x1: usize, x2: usize, y: usize) {
    let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    for x in start..=end {
        if y > 0 && y < MAP_SIZE - 1 {
            map[y][x] = Tile::Floor;
            // Make corridor 2 tiles wide
            if y > 1 {
                map[y - 1][x] = Tile::Floor;
            }
        }
    }
}

fn carve_v_corridor(map: &mut [[Tile; MAP_SIZE]; MAP_SIZE], y1: usize, y2: usize, x: usize) {
    let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
    for y in start..=end {
        if x > 0 && x < MAP_SIZE - 1 {
            map[y][x] = Tile::Floor;
            // Make corridor 2 tiles wide
            if x > 1 {
                map[y][x - 1] = Tile::Floor;
            }
        }
    }
}
