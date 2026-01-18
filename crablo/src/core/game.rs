use macroquad::prelude::*;

use crate::core::constants::*;
use crate::core::database::SaveData;
use crate::core::player::Player;
use crate::core::shop::{create_shop_items, try_purchase};
use crate::core::traits::{DamageDealer, Damageable};
use crate::systems::game_renderer::{render_game, GameRenderData};
use crate::systems::pathfinding::{bfs, dist};
use crate::systems::rendering::{to_screen, to_tile};
use crate::world::entities::{DmgText, EquipmentType, Monster, ShopItem};
use crate::world::map::{create_map, get_player_spawn, Tile};

#[derive(Default)]
pub struct GameEvents {
    pub player_hit: bool,
    pub monster_hit: bool,
    pub gold_collected: bool,
    pub monster_killed: bool,
    pub floor_completed: bool,
    pub game_over: bool,
}

pub struct Game {
    pub map: [[Tile; MAP_SIZE]; MAP_SIZE],
    pub cam: (f32, f32),
    pub player: Player,
    pub monsters: Vec<Monster>,
    pub texts: Vec<DmgText>,
    pub gold: Vec<(usize, usize)>,
    pub potions: Vec<(usize, usize)>,
    pub score: i32,
    pub difficulty: Difficulty,
    pub floor: i32,
    pub shake: f32,
    pub explored: [[bool; MAP_SIZE]; MAP_SIZE],
    // Traps
    pub spikes: Vec<(usize, usize)>,
    pub poison: Vec<(usize, usize)>,
    // Equipment drops on ground
    pub equipment_drops: Vec<(usize, usize, EquipmentType)>,
    // Stats
    pub total_kills: i32,
    pub total_gold: i32,
    pub total_damage_dealt: i32,
    // Shop
    pub in_shop: bool,
    pub shop_items: Vec<ShopItem>,
}

impl Game {
    pub fn new() -> Self {
        Self::with_difficulty(Difficulty::Normal)
    }

    pub fn from_save(save: &SaveData) -> Self {
        let difficulty = match save.difficulty {
            0 => Difficulty::Easy,
            2 => Difficulty::Hard,
            _ => Difficulty::Normal,
        };
        let mut game = Self::with_difficulty(difficulty);
        game.floor = save.floor;
        game.player.hp = save.hp;
        game.player.max_hp = save.max_hp;
        game.score = save.score;
        // Regenerate the floor with proper scaling
        if save.floor > 1 {
            game.floor = save.floor - 1;
            game.next_floor();
            game.player.hp = save.hp; // Restore HP after next_floor
        }
        game
    }

    pub fn get_difficulty_id(&self) -> i32 {
        match self.difficulty {
            Difficulty::Easy => 0,
            Difficulty::Normal => 1,
            Difficulty::Hard => 2,
        }
    }

    pub fn with_difficulty(difficulty: Difficulty) -> Self {
        let (map, gold_positions, monster_positions) = create_map();
        let (px, py) = get_player_spawn(&map);

        let hp_mult = difficulty.monster_hp_mult();
        let monsters = monster_positions
            .into_iter()
            .map(|(x, y, mtype)| {
                let mut m = match mtype {
                    1 => Monster::new_fast(x, y),
                    2 => Monster::new_tank(x, y),
                    _ => Monster::new(x, y),
                };
                m.hp = (m.hp as f32 * hp_mult) as i32;
                m.damage = (m.damage as f32 * difficulty.monster_damage_mult()) as i32;
                m
            })
            .collect();

        // Generate potion positions in some rooms
        let potions = gold_positions
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, &pos)| (pos.0.saturating_add(1), pos.1))
            .filter(|&(x, y)| x < MAP_SIZE && y < MAP_SIZE && map[y][x] == Tile::Floor)
            .collect();

        let _player_hp = (PLAYER_START_HP as f32 * difficulty.player_hp_mult()) as i32;

        // Initialize explored map - start with player's area visible
        let mut explored = [[false; MAP_SIZE]; MAP_SIZE];
        for dy in -3i32..=3 {
            for dx in -3i32..=3 {
                let ex = px as i32 + dx;
                let ey = py as i32 + dy;
                if ex >= 0 && ey >= 0 && (ex as usize) < MAP_SIZE && (ey as usize) < MAP_SIZE {
                    explored[ey as usize][ex as usize] = true;
                }
            }
        }

        // Generate traps
        let spikes: Vec<(usize, usize)> = gold_positions
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 4 == 0)
            .map(|(_, &pos)| (pos.0.saturating_sub(1), pos.1.saturating_add(1)))
            .filter(|&(x, y)| x < MAP_SIZE && y < MAP_SIZE && map[y][x] == Tile::Floor)
            .collect();

        let poison: Vec<(usize, usize)> = gold_positions
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 5 == 0)
            .map(|(_, &pos)| (pos.0.saturating_add(2), pos.1))
            .filter(|&(x, y)| x < MAP_SIZE && y < MAP_SIZE && map[y][x] == Tile::Floor)
            .collect();

        Game {
            map,
            cam: (screen_width() / 2., 50.),
            player: Player::new(px, py, difficulty),
            monsters,
            texts: vec![],
            score: 0,
            gold: gold_positions,
            potions,
            difficulty,
            floor: 1,
            shake: 0.,
            explored,
            spikes,
            poison,
            equipment_drops: vec![],
            total_kills: 0,
            total_gold: 0,
            total_damage_dealt: 0,
            in_shop: false,
            shop_items: vec![],
        }
    }

    pub fn next_floor(&mut self) {
        self.floor += 1;
        let (map, gold_positions, monster_positions) = create_map();
        let (px, py) = get_player_spawn(&map);

        let hp_mult = self.difficulty.monster_hp_mult() * (1.0 + self.floor as f32 * 0.1);
        let dmg_mult = self.difficulty.monster_damage_mult() * (1.0 + self.floor as f32 * 0.1);
        let is_boss_floor = self.floor % 5 == 0;

        self.monsters = monster_positions
            .into_iter()
            .enumerate()
            .map(|(i, (x, y, mtype))| {
                // First monster on boss floors is a boss
                let mut m = if is_boss_floor && i == 0 {
                    Monster::new_boss(x, y)
                } else {
                    match mtype {
                        1 => Monster::new_fast(x, y),
                        2 => Monster::new_tank(x, y),
                        _ => Monster::new(x, y),
                    }
                };
                m.hp = (m.hp as f32 * hp_mult) as i32;
                m.damage = (m.damage as f32 * dmg_mult) as i32;
                m
            })
            .collect();

        self.potions = gold_positions
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, &pos)| (pos.0.saturating_add(1), pos.1))
            .filter(|&(x, y)| x < MAP_SIZE && y < MAP_SIZE && map[y][x] == Tile::Floor)
            .collect();

        self.map = map;
        self.gold = gold_positions;
        self.player.x = px;
        self.player.y = py;
        self.player.path.clear();
        self.explored = [[false; MAP_SIZE]; MAP_SIZE];

        // Reveal starting area
        for dy in -3i32..=3 {
            for dx in -3i32..=3 {
                let ex = px as i32 + dx;
                let ey = py as i32 + dy;
                if ex >= 0 && ey >= 0 && (ex as usize) < MAP_SIZE && (ey as usize) < MAP_SIZE {
                    self.explored[ey as usize][ex as usize] = true;
                }
            }
        }

        // Bonus score for completing floor
        self.score += 500 * (self.floor - 1);

        // Generate new traps
        self.spikes = self
            .gold
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 4 == 0)
            .map(|(_, &pos)| (pos.0.saturating_sub(1), pos.1.saturating_add(1)))
            .filter(|&(x, y)| x < MAP_SIZE && y < MAP_SIZE && self.map[y][x] == Tile::Floor)
            .collect();

        self.poison = self
            .gold
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 5 == 0)
            .map(|(_, &pos)| (pos.0.saturating_add(2), pos.1))
            .filter(|&(x, y)| x < MAP_SIZE && y < MAP_SIZE && self.map[y][x] == Tile::Floor)
            .collect();

        self.equipment_drops.clear();

        // Show shop every 3 floors
        if self.floor % 3 == 0 {
            self.in_shop = true;
            self.shop_items = create_shop_items();
        }
    }

    pub fn update(&mut self, dt: f32) -> GameEvents {
        let mut events = GameEvents::default();

        // Update screen shake
        if self.shake > 0. {
            self.shake -= dt * 10.;
            if self.shake < 0. {
                self.shake = 0.;
            }
        }

        // Update ability cooldowns
        if self.player.dash_cd > 0. {
            self.player.dash_cd -= dt;
        }
        if self.player.area_cd > 0. {
            self.player.area_cd -= dt;
        }
        if self.player.heal_cd > 0. {
            self.player.heal_cd -= dt;
        }
        if self.player.ranged_cd > 0. {
            self.player.ranged_cd -= dt;
        }

        // Update poison damage
        if self.player.poisoned > 0. {
            self.player.poisoned -= dt;
            // Poison does 2 damage per second
            if (self.player.poisoned * 10.) as i32 % 10 == 0 {
                Damageable::take_damage(&mut self.player, 1);
                self.shake = 0.2;
            }
        }

        // Shop handling - skip normal gameplay while in shop
        if self.in_shop {
            // Shop input handled in draw, just return
            return events;
        }

        if self.player.is_dead() {
            events.game_over = true;
            return events;
        }

        // All monsters dead - go to next floor
        if self.monsters.is_empty() {
            self.next_floor();
            events.floor_completed = true;
            return events;
        }

        // update text animations
        self.texts.retain_mut(|t| {
            t.life -= dt;
            t.y -= DMG_TEXT_SPEED * dt;
            t.life > 0.
        });

        // Dash ability (Space key)
        if is_key_pressed(KeyCode::Space)
            && self.player.dash_cd <= 0.
            && !self.player.path.is_empty()
        {
            self.player.dash_cd = 2.0; // 2 second cooldown
            for _ in 0..3 {
                if self.player.path.is_empty() {
                    break;
                }
                let (nx, ny) = self.player.path[0];
                if self.monsters.iter().any(|m| m.x == nx && m.y == ny) {
                    break;
                }
                self.player.path.remove(0);
                self.player.x = nx;
                self.player.y = ny;
            }
        }

        // Area attack ability (Q key) - damages all adjacent monsters
        if is_key_pressed(KeyCode::Q) && self.player.area_cd <= 0. {
            self.player.area_cd = 3.0; // 3 second cooldown
            let mut killed_any = false;
            let px = self.player.x as i32;
            let py = self.player.y as i32;

            // Find all adjacent monsters
            let adjacent: Vec<usize> = self
                .monsters
                .iter()
                .enumerate()
                .filter(|(_, m)| {
                    let dx = (m.x as i32 - px).abs();
                    let dy = (m.y as i32 - py).abs();
                    dx <= 1 && dy <= 1 && (dx + dy) > 0
                })
                .map(|(i, _)| i)
                .collect();

            // Damage them in reverse order to avoid index issues
            for i in adjacent.into_iter().rev() {
                if self.damage_monster(i, self.player.damage() / 2) {
                    killed_any = true;
                }
            }
            if killed_any {
                events.monster_killed = true;
            }
            events.monster_hit = true;
        }

        // Heal ability (E key) - restore HP
        if is_key_pressed(KeyCode::E)
            && self.player.heal_cd <= 0.
            && Damageable::hp(&self.player) < Damageable::max_hp(&self.player)
        {
            self.player.heal_cd = 5.0; // 5 second cooldown
            let heal_amount = Damageable::max_hp(&self.player) / 4;
            Damageable::heal(&mut self.player, heal_amount);

            let (sx, sy) = to_screen(self.player.x, self.player.y, self.cam);
            self.texts.push(DmgText {
                x: sx,
                y: sy - 40.,
                dmg: -heal_amount,
                life: DMG_TEXT_DURATION,
            });
        }

        // Ranged attack ability (R key) - attack nearest monster in range
        if is_key_pressed(KeyCode::R) && self.player.ranged_cd <= 0. {
            let px = self.player.x as i32;
            let py = self.player.y as i32;

            // Find nearest monster within range 5
            if let Some((idx, _)) = self
                .monsters
                .iter()
                .enumerate()
                .filter(|(_, m)| {
                    let dx = (m.x as i32 - px).abs();
                    let dy = (m.y as i32 - py).abs();
                    dx <= 5 && dy <= 5
                })
                .min_by_key(|(_, m)| {
                    let dx = (m.x as i32 - px).abs();
                    let dy = (m.y as i32 - py).abs();
                    dx + dy
                })
            {
                self.player.ranged_cd = 1.5; // 1.5 second cooldown
                if self.damage_monster(idx, self.player.damage()) {
                    events.monster_killed = true;
                }
                events.monster_hit = true;
            }
        }

        // WASD keyboard movement
        let mut move_dir: Option<(i32, i32)> = None;
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            move_dir = Some((-1, -1)); // Up-left in isometric
        } else if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            move_dir = Some((1, 1)); // Down-right in isometric
        } else if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            move_dir = Some((-1, 1)); // Down-left in isometric
        } else if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            move_dir = Some((1, -1)); // Up-right in isometric
        }

        if let Some((dx, dy)) = move_dir {
            let nx = (self.player.x as i32 + dx) as usize;
            let ny = (self.player.y as i32 + dy) as usize;
            if nx < MAP_SIZE && ny < MAP_SIZE && self.map[ny][nx] == Tile::Floor {
                if !self.monsters.iter().any(|m| m.x == nx && m.y == ny) {
                    self.player.path = vec![(nx, ny)];
                }
            }
        }

        // mouse input logic - auto attack on hold
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();

            if let Some((tx, ty)) = to_tile(mx, my, self.cam) {
                // Check if clicked on a monster - attack it directly
                if let Some(i) = self.monsters.iter().position(|m| m.x == tx && m.y == ty) {
                    let killed = self.damage_monster(i, self.player.damage());
                    events.monster_hit = true;
                    if killed {
                        events.monster_killed = true;
                    }
                    self.player.path.clear();
                }
                // Otherwise move to the clicked tile
                else if self.map[ty][tx] == Tile::Floor {
                    self.player.path = bfs(&self.map, (self.player.x, self.player.y), (tx, ty))
                }
            }
        }

        // handle movement for the player
        if !self.player.path.is_empty() {
            self.player.move_cd -= dt;

            // time to move?
            if self.player.move_cd <= 0. {
                self.player.move_cd = PLAYER_MOVE_CD;

                let (nx, ny) = self.player.path[0];

                // Check if monster is blocking the path
                if self.monsters.iter().any(|m| m.x == nx && m.y == ny) {
                    // Stop moving, player needs to click on monster to attack
                    self.player.path.clear();
                } else {
                    // move
                    self.player.path.remove(0);
                    self.player.x = nx;
                    self.player.y = ny;

                    // Update fog of war - reveal area around player
                    for dy in -4i32..=4 {
                        for dx in -4i32..=4 {
                            let ex = self.player.x as i32 + dx;
                            let ey = self.player.y as i32 + dy;
                            if ex >= 0
                                && ey >= 0
                                && (ex as usize) < MAP_SIZE
                                && (ey as usize) < MAP_SIZE
                            {
                                self.explored[ey as usize][ex as usize] = true;
                            }
                        }
                    }

                    // collect gold logic
                    if let Some(i) = self
                        .gold
                        .iter()
                        .position(|&g| g == (self.player.x, self.player.y))
                    {
                        self.gold.remove(i);
                        self.score += GOLD_VALUE;
                        self.total_gold += GOLD_VALUE;
                        events.gold_collected = true;

                        // spawn a green text
                        let (sx, sy) = to_screen(self.player.x, self.player.y, self.cam);

                        self.texts.push(DmgText {
                            x: sx,
                            y: sy - 40.,
                            dmg: -GOLD_VALUE,
                            life: DMG_TEXT_DURATION,
                        });
                    }

                    // collect potion logic
                    if let Some(i) = self
                        .potions
                        .iter()
                        .position(|&p| p == (self.player.x, self.player.y))
                    {
                        self.potions.remove(i);
                        let heal_amount = 25;
                        self.player.hp = (self.player.hp + heal_amount).min(self.player.max_hp);

                        // spawn a green text for healing
                        let (sx, sy) = to_screen(self.player.x, self.player.y, self.cam);

                        self.texts.push(DmgText {
                            x: sx,
                            y: sy - 40.,
                            dmg: -heal_amount, // negative = green text
                            life: DMG_TEXT_DURATION,
                        });
                    }

                    // Spike trap damage
                    if self.spikes.contains(&(self.player.x, self.player.y)) {
                        let damage = 15 - self.player.armor.min(10);
                        self.player.hp -= damage.max(5);
                        self.shake = 0.5;
                        events.player_hit = true;

                        let (sx, sy) = to_screen(self.player.x, self.player.y, self.cam);
                        self.texts.push(DmgText {
                            x: sx,
                            y: sy - 40.,
                            dmg: damage,
                            life: DMG_TEXT_DURATION,
                        });
                    }

                    // Poison trap
                    if self.poison.contains(&(self.player.x, self.player.y)) {
                        self.player.poisoned = 5.0; // 5 seconds of poison
                    }

                    // Collect equipment
                    if let Some(i) = self
                        .equipment_drops
                        .iter()
                        .position(|(x, y, _)| *x == self.player.x && *y == self.player.y)
                    {
                        let (_, _, eq_type) = self.equipment_drops.remove(i);
                        match eq_type {
                            EquipmentType::Sword => {
                                self.player.weapon_damage += 5;
                                let (sx, sy) = to_screen(self.player.x, self.player.y, self.cam);
                                self.texts.push(DmgText {
                                    x: sx,
                                    y: sy - 40.,
                                    dmg: -5,
                                    life: DMG_TEXT_DURATION,
                                });
                            }
                            EquipmentType::Shield => {
                                self.player.armor += 5;
                            }
                            EquipmentType::Ring => {
                                self.player.max_hp += 20;
                                self.player.hp += 20;
                            }
                        }
                    }
                }
            }
        }

        // Camera follow player
        let target_cam_x =
            screen_width() / 2. - (self.player.x as f32 - self.player.y as f32) * TILE_WIDTH;
        let target_cam_y = 100. - (self.player.x as f32 + self.player.y as f32) * TILE_HEIGHT
            + screen_height() / 2.;
        self.cam.0 += (target_cam_x - self.cam.0) * 5.0 * dt;
        self.cam.1 += (target_cam_y - self.cam.1) * 5.0 * dt;

        // Monster logic
        // calculate the occupied spots so enemies dont stack
        let occupied: Vec<_> = self
            .monsters
            .iter()
            .map(|m| (m.x, m.y))
            .chain(std::iter::once((self.player.x, self.player.y)))
            .collect();

        for i in 0..self.monsters.len() {
            self.monsters[i].cd -= dt;
            if self.monsters[i].cd <= 0. {
                self.monsters[i].cd = self.monsters[i].move_cd;

                let (mx, my) = (self.monsters[i].x, self.monsters[i].y);
                let monster_damage = self.monsters[i].damage;

                let d = dist((mx, my), (self.player.x, self.player.y));

                if d == 1 {
                    self.player.hp -= monster_damage;
                    self.shake = 1.0; // Screen shake on hit
                    events.player_hit = true;
                    let (sx, sy) = to_screen(self.player.x, self.player.y, self.cam);

                    self.texts.push(DmgText {
                        x: sx,
                        y: sy - 40.,
                        dmg: monster_damage,
                        life: DMG_TEXT_DURATION,
                    });
                } else {
                    // chase the player
                    let path = bfs(&self.map, (mx, my), (self.player.x, self.player.y));

                    if path.len() > 1 && !occupied.contains(&path[0]) {
                        self.monsters[i].x = path[0].0;
                        self.monsters[i].y = path[0].1;
                    }
                }
            }
        }

        events
    }

    fn damage_monster(&mut self, idx: usize, amount: i32) -> bool {
        self.monsters[idx].hp -= amount;

        // spawn the text
        let (sx, sy) = to_screen(self.monsters[idx].x, self.monsters[idx].y, self.cam);

        self.texts.push(DmgText {
            x: sx,
            y: sy - 40.,
            dmg: amount,
            life: DMG_TEXT_DURATION,
        });

        // kill logic
        if self.monsters[idx].hp <= 0 {
            let monster_type = self.monsters[idx].monster_type;
            let mx = self.monsters[idx].x;
            let my = self.monsters[idx].y;
            self.monsters.remove(idx);
            self.score += KILL_BONUS;
            self.total_kills += 1;
            self.total_damage_dealt += amount;

            // XP gain based on monster type
            let xp_gain = match monster_type {
                crate::world::entities::MonsterType::Fast => 15,
                crate::world::entities::MonsterType::Normal => 25,
                crate::world::entities::MonsterType::Tank => 40,
                crate::world::entities::MonsterType::Boss => 100,
            };
            self.player.xp += xp_gain;

            // Level up check
            while self.player.xp >= self.player.xp_to_next {
                self.player.xp -= self.player.xp_to_next;
                self.player.level += 1;
                self.player.xp_to_next = 100 + (self.player.level * 50);
                self.player.max_hp += 10;
                self.player.hp = (self.player.hp + 10).min(self.player.max_hp);
                self.player.weapon_damage += 1;
            }

            // Equipment drop chance (20% for normal, 50% for boss)
            let drop_chance = if monster_type == crate::world::entities::MonsterType::Boss {
                0.5
            } else {
                0.2
            };
            if macroquad::rand::gen_range(0., 1.) < drop_chance {
                let eq_type = match macroquad::rand::gen_range(0, 3) {
                    0 => EquipmentType::Sword,
                    1 => EquipmentType::Shield,
                    _ => EquipmentType::Ring,
                };
                self.equipment_drops.push((mx, my, eq_type));
            }

            return true;
        }
        self.total_damage_dealt += amount;
        false
    }

    pub fn draw(&self) {
        let render_data = GameRenderData {
            map: &self.map,
            explored: &self.explored,
            cam: self.cam,
            shake: self.shake,
            player: &self.player,
            monsters: &self.monsters,
            texts: &self.texts,
            gold: &self.gold,
            potions: &self.potions,
            spikes: &self.spikes,
            poison: &self.poison,
            equipment_drops: &self.equipment_drops,
            score: self.score,
            floor: self.floor,
            difficulty: self.difficulty,
            in_shop: self.in_shop,
            shop_items: &self.shop_items,
        };
        render_game(&render_data);
    }

    pub fn handle_shop_input(&mut self) {
        if !self.in_shop {
            return;
        }

        // Buy items with number keys
        for (i, key) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4]
            .iter()
            .enumerate()
        {
            if is_key_pressed(*key) {
                if let Some((cost, result)) = try_purchase(&mut self.shop_items, i, self.score) {
                    self.score -= cost;
                    if result.heal_full {
                        self.player.hp = self.player.max_hp;
                    }
                    self.player.max_hp += result.max_hp_bonus;
                    self.player.hp += result.max_hp_bonus;
                    self.player.weapon_damage += result.damage_bonus;
                    self.player.armor += result.armor_bonus;
                }
            }
        }

        // Exit shop
        if is_key_pressed(KeyCode::Enter) {
            self.in_shop = false;
        }
    }
}
