mod boss;
mod member;

use boss::Boss;
use member::{Member, Role};
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


/*
Level 8
required
XP
Files to submit
Allowed functions
0.00 B
mobs/src/lib.rs, mobs/src/mobs.rs, mobs/src/mobs/
—
Instructions
Create a structure Mob which has:

name: String

boss: Boss

members: a HashMap of Members keyed by Strings

cities: a HashSet of city names

wealth: u64

recruit: a method which adds a Member to the members map. It should accept a tuple with the member's information, name and age ((&str, u32)). The member's role should be set to the lowest one, Associate.

attack: a method which receives another Mob as reference. It will remove the youngest member(s) from the members from whichever mob has the least power combat score. In the case of a draw, the attacker loses. Furthermore, if the loser is left with zero members, the victorious mob will also take the cities and wealth from the losing mob. The power combat score is calculated from the sum of the role of each mob member:

Underboss: 4
Caporegime: 3
Soldier: 2
Associate: 1
steal: a method which receives a Mob to target, and a u64 value to steal. The self mob steals the value from the wealth of the target mob, and adds the value to its own wealth. Naturally, any mob can only be stolen as much money as they have in possession.

conquer_city: a method which receives a slice of references of Mob, and a String city name. The city is added to its list of cities if none of the other mobs already have it.

You will also need to create two submodules:

boss: which should contain:

Boss: a struct which consists of:
name: String
age: u32
new: an associated function which accepts a name: &str and age: u32, and returns a Boss.
member submodule which consists of:

Role: an enum with the variants:

Underboss
Caporegime
Soldier
Associate
Member: a struct which consists of:

role: Role

age: u32

get_promotion: a method which when invoked should promote the member from:

Associate -> Soldier
Soldier -> Caporegime
Caporegime -> Underboss
Underboss -> Impossibility. Panic.
We advise you to have each submodule inside their own respective file, but that is up to you.

Every data structure should derive at least Debug and PartialEq.

Expected Function
You'll need to work out the function signatures for yourself.

Usage
You'll need to create your own tests and samples. Make sure you cover every possibility!

Notions
Packages, Crates and Modules
*/