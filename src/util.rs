//! ark model — ARK: Survival Evolved dedicated server.

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub steam_id: String,
    pub tribe: String,
}

#[derive(Debug, Clone)]
pub struct Tribe {
    pub name: String,
    pub owner: String,
    pub members: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub map: String,
    pub players: Vec<Player>,
    pub tribes: Vec<Tribe>,
    pub running: bool,
}

impl Default for Model {
    fn default() -> Self { Self { map: "TheIsland".into(), players: vec![], tribes: vec![], running: false } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn start(&mut self) -> bool {
        if !self.running { self.running = true; true } else { false }
    }

    pub fn join(&mut self, player: Player) -> bool {
        if self.running { self.players.push(player); true } else { false }
    }

    pub fn create_tribe(&mut self, name: &str, owner: &str) -> bool {
        self.tribes.push(Tribe { name: name.into(), owner: owner.into(), members: vec![owner.into()] });
        true
    }

    pub fn list_players(&self) -> &[Player] { &self.players }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn start_join_tribe() {
        let mut a = Model::new();
        assert!(a.start());
        assert!(a.join(Player { name: "P1".into(), steam_id: "s1".into(), tribe: "T1".into() }));
        assert!(a.create_tribe("T1", "P1"));
        assert_eq!(a.list_players().len(), 1);
    }
}
