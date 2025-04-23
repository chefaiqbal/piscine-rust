pub mod boss;
pub mod member;

pub use boss::Boss;
pub use member::{Member, Role};

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, info: (&str, u32)) {
        let (name, age) = info;
        self.members.insert(
            name.to_string(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_score: u32 = self.members.values().map(|m| m.role.score()).sum();
        let other_score: u32 = other.members.values().map(|m| m.role.score()).sum();

        if self_score <= other_score {
            self.remove_youngest_member();
        } else {
            other.remove_youngest_member();
            if other.members.is_empty() {
                self.cities.extend(other.cities.drain());
                self.wealth += other.wealth;
                other.wealth = 0;
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let stolen = amount.min(target.wealth);
        self.wealth += stolen;
        target.wealth -= stolen;
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], city: String) {
        if mobs.iter().all(|mob| !mob.cities.contains(&city)) {
            self.cities.insert(city);
        }
    }

    fn remove_youngest_member(&mut self) {
        if let Some(youngest) = self
            .members
            .iter()
            .min_by_key(|(_, m)| m.age)
            .map(|(n, _)| n.clone())
        {
            self.members.remove(&youngest);
        }
    }
}
