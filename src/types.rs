#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
pub struct Crabs(pub i32);

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
pub struct ClickPower(pub i32);

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
pub struct Upgrades {
    pub claws: i32,
    pub nets: i32,
    pub buckets: i32,
    pub traps: i32,
    pub boats: i32,
    pub divers: i32,
    pub subs: i32,
    pub islands: i32,
    pub reefs: i32,
    pub oceans: i32,
}

impl Upgrades {
    pub fn init() -> Self {
        Self {
            claws: 0,
            nets: 0,
            buckets: 0,
            traps: 0,
            boats: 0,
            divers: 0,
            subs: 0,
            islands: 0,
            reefs: 0,
            oceans: 0,
        }
    }

    pub fn rate(&self) -> i32 {
        self.claws * 1
            + self.nets * 10
            + self.buckets * 100
            + self.traps * 1000
            + self.boats * 10000
            + self.divers * 100000
            + self.subs * 1000000
            + self.islands * 10000000
            + self.reefs * 100000000
            + self.oceans * 1000000000
    }
}
