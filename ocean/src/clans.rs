use std::cell::RefCell;
use std::collections::HashMap;
#[derive(Debug)]
pub struct ClanSystem {
    clans: RefCell<HashMap<String, Vec<String>>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem{
            clans: RefCell::new(HashMap::new())
        }
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        let clan = self.clans.borrow().get(clan_id).unwrap_or(&Vec::new()).clone();
        clan
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clans.borrow().len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        let count = self.clans.borrow().get(clan_id).unwrap_or(&Vec::new()).clone().len();
        count
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        let mut count = 0;
        let mut id = Option::None;
        for (key, _value) in self.clans.borrow().iter() {
            let c = self.get_clan_member_count(key);
            if c > count {
                count = c;
                id = Some(key.to_string());
            }
        }
        id
    }

    pub fn add_to_clan(&self, clan_id: &String, name: &String) {
        self.clans.borrow_mut().entry(clan_id.clone()).or_insert_with(Vec::new).push(name.clone());
    }
}