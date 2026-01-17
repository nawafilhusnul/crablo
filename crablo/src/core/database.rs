use rusqlite::{Connection, Result};

pub struct ScoreEntry {
    pub rank: i32,
    pub name: String,
    pub score: i32,
}

pub struct SaveData {
    pub floor: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub score: i32,
    pub difficulty: i32,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("crablo_scores.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS scores (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                score INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS saves (
                id INTEGER PRIMARY KEY,
                floor INTEGER NOT NULL,
                hp INTEGER NOT NULL,
                max_hp INTEGER NOT NULL,
                score INTEGER NOT NULL,
                difficulty INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn save_score(&self, name: &str, score: i32) -> Result<()> {
        self.conn.execute(
            "INSERT INTO scores (name, score) VALUES (?1, ?2)",
            [name, &score.to_string()],
        )?;
        Ok(())
    }

    pub fn get_top_scores(&self, limit: i32) -> Result<Vec<ScoreEntry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, score FROM scores ORDER BY score DESC LIMIT ?1")?;

        let scores = stmt.query_map([limit], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        })?;

        let mut result = Vec::new();
        for (i, score) in scores.enumerate() {
            let (name, score_val) = score?;
            result.push(ScoreEntry {
                rank: (i + 1) as i32,
                name,
                score: score_val,
            });
        }

        Ok(result)
    }

    #[allow(dead_code)]
    pub fn is_high_score(&self, score: i32) -> Result<bool> {
        let top_scores = self.get_top_scores(10)?;
        if top_scores.len() < 10 {
            return Ok(true);
        }
        Ok(score > top_scores.last().map(|s| s.score).unwrap_or(0))
    }

    pub fn reset_scores(&self) -> Result<()> {
        self.conn.execute("DELETE FROM scores", [])?;
        Ok(())
    }

    pub fn save_game(
        &self,
        floor: i32,
        hp: i32,
        max_hp: i32,
        score: i32,
        difficulty: i32,
    ) -> Result<()> {
        // Delete old save first (only one save slot)
        self.conn.execute("DELETE FROM saves", [])?;
        self.conn.execute(
            "INSERT INTO saves (floor, hp, max_hp, score, difficulty) VALUES (?1, ?2, ?3, ?4, ?5)",
            [floor, hp, max_hp, score, difficulty],
        )?;
        Ok(())
    }

    pub fn load_game(&self) -> Result<Option<SaveData>> {
        let mut stmt = self.conn.prepare(
            "SELECT floor, hp, max_hp, score, difficulty FROM saves ORDER BY id DESC LIMIT 1",
        )?;

        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(Some(SaveData {
                floor: row.get(0)?,
                hp: row.get(1)?,
                max_hp: row.get(2)?,
                score: row.get(3)?,
                difficulty: row.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn delete_save(&self) -> Result<()> {
        self.conn.execute("DELETE FROM saves", [])?;
        Ok(())
    }

    pub fn has_save(&self) -> Result<bool> {
        let count: i32 = self
            .conn
            .query_row("SELECT COUNT(*) FROM saves", [], |row| row.get(0))?;
        Ok(count > 0)
    }
}
