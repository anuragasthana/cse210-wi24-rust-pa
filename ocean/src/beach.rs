use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    size: usize,
    crabs: Vec<Crab>,
    clans: ClanSystem
}

impl Beach {
    pub fn new() -> Beach {
        Beach {
            size: 0,
            crabs: Vec::new(),
            clans: ClanSystem::new()
        }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.size
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.size += 1;
        self.crabs.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        let mut max_speed = 0;
        let mut ret = Option::None;
        for crab in self.crabs.iter(){
            if crab.speed() > max_speed{
                max_speed = crab.speed();
                ret = Some(crab)
            }
        }
        ret
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut named = Vec::new();
        for crab in self.crabs.iter(){
            if crab.name() == name {
                named.push(crab)
            }
        }
        named
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        if i >= self.size() || j >= self.size() {
            panic!("Invalid indices for chosen crabs.")
        } else{
            let c1 = &self.crabs[i];
            let c2 = &self.crabs[j];
            let baby = Crab::breed(c1, c2, name);
            Beach::add_crab(self, baby)
        }
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clans
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        let c: String = clan_id.to_string();
        let n: String = crab_name.to_string();
        self.clans.add_to_clan(&c, &n);
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let c1 = self.clans.get_clan_member_names(id1);
        let c2 = self.clans.get_clan_member_names(id2);
        let mut r: Result<Option<String>, String> = Err("Invalid ids".parse().unwrap());
        if c1.len() > 0 && c2.len() > 0 {
            let mut s1t = 0;
            let mut s2t = 0;
            for crab in self.crabs.iter(){
                if c1.contains(&crab.name().to_string()) {
                    s1t += crab.speed()
                } else if c2.contains(&crab.name().to_string()) {
                    s2t += crab.speed()
                }
            }
            let s1 = (s1t as f32) / (c1.len() as f32);
            let s2 = (s2t as f32)  / (c2.len() as f32);
            if s1 == s2 {
                r = Result::Ok(Option::None);
            } else if s1 > s2 {
                r = Result::Ok(Some(id1.to_string()))
            } else {
                r = Result::Ok(Some(id2.to_string()))
            }
        }
        r
    }
}
