use std::collections::HashMap;

fn main() {
    let mut missions_flown = HashMap::new();
    missions_flown.insert("Hadfield", 3);
    missions_flown.insert("Hurley", 3);
    missions_flown.insert("Barron", 0);
    println!("missions_flown is {:?}", missions_flown);

    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {:?}", barron_missions);
    // Update hash map
    missions_flown.insert("Barron", 1);
    // Overwrites existing entry
    println!("missions_flown is {:?}", missions_flown);
    // Adds new entry if it does not already exist
    missions_flown.entry("Barron").or_insert(2);
    println!("missions_flown is {:?}", missions_flown);
    missions_flown.entry("Stone").or_insert(2);
    println!("missions_flown is {:?}", missions_flown);
    // Modify the existing data in the Barron entry by modifying the data directly, as opposed to
    // through the key
    let barron = missions_flown.entry("Barron").or_insert(0);
    *barron += 1;
    println!("missions_flown is {:?}", missions_flown);
}
