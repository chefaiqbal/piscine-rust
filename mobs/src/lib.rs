pub mod mobs;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn create_boss() {
        assert_eq!(
            crate::mobs::Boss::new("Scarface", 43),
            crate::mobs::Boss {
                name: "Scarface".to_owned(),
                age: 43
            }
        );
    }

    #[inline]
    fn mobs() -> (crate::mobs::Mob, crate::mobs::Mob) {
        (
            crate::mobs::Mob {
                name: "Hairy Giants".to_owned(),
                boss: crate::mobs::Boss::new("Louie HaHa", 36),
                cities: ["San Francisco"].map(str::to_owned).into(),
                members: [
                    (
                        "Benny Eggs",
                        crate::mobs::Member {
                            role: crate::mobs::Role::Soldier,
                            age: 28,
                        },
                    ),
                    (
                        "Jhonny",
                        crate::mobs::Member {
                            role: crate::mobs::Role::Associate,
                            age: 17,
                        },
                    ),
                    (
                        "Greasy Thumb",
                        crate::mobs::Member {
                            role: crate::mobs::Role::Soldier,
                            age: 30,
                        },
                    ),
                    (
                        "No Finger",
                        crate::mobs::Member {
                            role: crate::mobs::Role::Caporegime,
                            age: 32,
                        },
                    ),
                ]
                .map(|(n, d)| (n.to_owned(), d))
                .into(),
                wealth: 100000,
            },
            crate::mobs::Mob {
                name: "Red Thorns".to_owned(),
                boss: crate::mobs::Boss::new("Big Tuna", 30),
                cities: ["San Jose"].map(str::to_owned).into(),
                members: [
                    (
                        "Knuckles",
                        crate::mobs::Member {
                            role: crate::mobs::Role::Soldier,
                            age: 25,
                        },
                    ),
                    (
                        "Baldy Dom",
                        crate::mobs::Member {
                            role: crate::mobs::Role::Caporegime,
                            age: 36,
                        },
                    ),
                    (
                        "Crazy Joe",
                        crate::mobs::Member {
                            role: crate::mobs::Role::Underboss,
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
            Some(&crate::mobs::Member {
                role: crate::mobs::Role::Associate,
                age: 37,
            })
        );

        mob.recruit(("Pedro", 14));

        assert_eq!(
            mob.members.get("Pedro"),
            Some(&crate::mobs::Member {
                role: crate::mobs::Role::Associate,
                age: 14,
            })
        );
    }

    #[test]
    fn member_get_promotion() {
        let (mut mob, _) = mobs();

        let member = mob.members.get_mut("Benny Eggs").unwrap();

        member.get_promotion();
        assert_eq!(member.role, crate::mobs::Role::Caporegime);
        member.get_promotion();
        assert_eq!(member.role, crate::mobs::Role::Underboss);
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
}
