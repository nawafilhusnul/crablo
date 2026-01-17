use std::collections::VecDeque;

use crate::core::constants::MAP_SIZE;
use crate::world::map::Tile;

pub fn bfs(
    map: &[[Tile; MAP_SIZE]; MAP_SIZE],
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut q = VecDeque::from([start]);
    let mut visited = [[false; MAP_SIZE]; MAP_SIZE];
    visited[start.1][start.0] = true;

    let mut parent = [[None; MAP_SIZE]; MAP_SIZE];

    while let Some(curr) = q.pop_front() {
        if curr == goal {
            let mut path = vec![];
            let mut c = goal;
            while c != start {
                path.push(c);

                c = parent[c.1][c.0].unwrap()
            }

            path.reverse();

            return path;
        }

        // check neighbors - 8 directions (including diagonals)
        for (dx, dy) in [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0), // cardinal
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1), // diagonal
        ] {
            let nx = curr.0 as i32 + dx;
            let ny = curr.1 as i32 + dy;

            // bounds check before casting to usize
            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if nx < MAP_SIZE && ny < MAP_SIZE && !visited[ny][nx] && map[ny][nx] == Tile::Floor
                {
                    // For diagonal movement, check that we're not cutting through walls
                    let is_diagonal = dx != 0 && dy != 0;
                    if is_diagonal {
                        let cx = curr.0;
                        let cy = curr.1;
                        // Check both adjacent cardinal tiles are walkable
                        let adj1 = map[cy][(cx as i32 + dx) as usize];
                        let adj2 = map[(cy as i32 + dy) as usize][cx];
                        if adj1 != Tile::Floor || adj2 != Tile::Floor {
                            continue; // Can't cut through wall corners
                        }
                    }

                    visited[ny][nx] = true;
                    parent[ny][nx] = Some(curr);
                    q.push_back((nx, ny));
                }
            }
        }
    }

    vec![]
}

pub fn dist(p1: (usize, usize), p2: (usize, usize)) -> i32 {
    (p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()
}
