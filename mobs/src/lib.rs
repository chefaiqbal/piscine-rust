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
            // Attacker loses
            self.remove_youngest_member();
        } else {
            // Defender loses
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
        if let Some((youngest_name, _)) = self
            .members
            .iter()
            .min_by_key(|(_, member)| member.age)
            .map(|(name, member)| (name.clone(), member.clone()))
        {
            self.members.remove(&youngest_name);
        }
    }
}


#[test]
fn create_boss() {
    assert_eq!(
        Boss::new("Scarface", 43),
        Boss {
            name: "Scarface".to_owned(),
            age: 43
        }
    );
}

#[inline]
fn mobs() -> (Mob, Mob) {
    (
        Mob {
            name: "Hairy Giants".to_owned(),
            boss: Boss::new("Louie HaHa", 36),
            cities: ["San Francisco"].map(str::to_owned).into(),
            members: [
                (
                    "Benny Eggs",
                    Member {
                        role: Role::Soldier,
                        age: 28,
                    },
                ),
                (
                    "Jhonny",
                    Member {
                        role: Role::Associate,
                        age: 17,
                    },
                ),
                (
                    "Greasy Thumb",
                    Member {
                        role: Role::Soldier,
                        age: 30,
                    },
                ),
                (
                    "No Finger",
                    Member {
                        role: Role::Caporegime,
                        age: 32,
                    },
                ),
            ]
            .map(|(n, d)| (n.to_owned(), d))
            .into(),
            wealth: 100000,
        },
        Mob {
            name: "Red Thorns".to_owned(),
            boss: Boss::new("Big Tuna", 30),
            cities: ["San Jose"].map(str::to_owned).into(),
            members: [
                (
                    "Knuckles",
                    Member {
                        role: Role::Soldier,
                        age: 25,
                    },
                ),
                (
                    "Baldy Dom",
                    Member {
                        role: Role::Caporegime,
                        age: 36,
                    },
                ),
                (
                    "Crazy Joe",
                    Member {
                        role: Role::Underboss,
                        age: 23,
                    },
                ),
            ]
            .map(|(n, d)| (n.to_owned(), d))
            .into(),
            wealth: 70000,
        },
    )
}

#[test]
fn mob_recruit() {
    let (mut mob, _) = mobs();
    mob.recruit(("Rusty", 37));

    assert_eq!(
        mob.members.get("Rusty"),
        Some(&Member {
            role: Role::Associate,
            age: 37,
        })
    );

    mob.recruit(("Pedro", 14));

    assert_eq!(
        mob.members.get("Pedro"),
        Some(&Member {
            role: Role::Associate,
            age: 14,
        })
    );
}

#[test]
fn member_get_promotion() {
    let (mut mob, _) = mobs();

    let member = mob.members.get_mut("Benny Eggs").unwrap();

    member.get_promotion();
    assert_eq!(member.role, member::Role::Caporegime);
    member.get_promotion();
    assert_eq!(member.role, member::Role::Underboss);
}

#[test]
#[should_panic]
fn member_get_promotion_panic() {
    let (_, mut mob) = mobs();

    let member = mob.members.get_mut("Crazy Joe").unwrap();

    member.get_promotion();
}

#[test]
fn mob_steal() {
    let (mut a, mut b) = mobs();
    b.steal(&mut a, 10000);
    assert_eq!(a.wealth, 90000);
    assert_eq!(b.wealth, 80000);

    b.steal(&mut a, 1000000);
    assert_eq!(a.wealth, 0);
    assert_eq!(b.wealth, 170000);
}

#[test]
fn mob_attack() {
    let (mut a, mut b) = mobs();
    a.attack(&mut b);

    assert_eq!(a.members.len(), 3);
    assert_eq!(b.members.len(), 3);
}

#[test]
fn same_combat_power() {
    let (mut a, mut b) = mobs();

    a.recruit(("Stitches", 28));
    a.attack(&mut b);

    assert_eq!(a.members.len(), 4);
    assert_eq!(b.members.len(), 3);
}

#[test]
fn no_members_mob() {
    let (mut a, mut b) = mobs();
    b.attack(&mut a);
    a.attack(&mut b);
    b.attack(&mut a);
    b.attack(&mut a);

    assert_eq!(a.members.len(), 0);
    assert_eq!(a.cities.len(), 0);
    assert_eq!(a.wealth, 0);

    assert!(b
        .cities
        .is_superset(&["San Jose", "San Francisco"].map(str::to_owned).into()));
    assert_eq!(b.wealth, 170000);
}

#[test]
fn mob_conquer_city() {
    let (mut a, mut b) = mobs();

    b.conquer_city(&[&a], "Las Vegas".to_owned());
    assert!(b.cities.contains("Las Vegas"));

    a.conquer_city(&[&b], "Las Vegas".to_owned());
    assert_eq!(a.cities.len(), 1);
}
