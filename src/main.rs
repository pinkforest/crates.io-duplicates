use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct VersionHash {
    checksum: [u8; 32],
}

fn main() {

    let index = crates_index::Index::new_cargo_default().unwrap();
    let mut total_duplicate_crates = 0;   // Amount of crates with duplicate version entries
    let mut total_conflict_crates = 0;    // .. as well as conflicting checksums

    for crate_from_idx in index.crates() {
    
        let mut versions = HashMap::<String, VersionHash>::new();
        let mut duplicates = 0;
        let mut conflicts = 0;
        let mut duplicate_list = vec![];
        
        for version in crate_from_idx.versions() {

            if versions.contains_key(&version.version().to_string()) {
                
                let version_cmp = (&mut versions).get(version.version()).unwrap().checksum.clone();
                
                duplicates += 1;
                duplicate_list.push(version.version());
                
                if &version_cmp != version.checksum() {

                    conflicts += 1;
                }
            }
            versions.insert(version.version().to_string(), VersionHash { checksum: *version.clone().checksum() });
        }
        
        if duplicates > 0 {
            total_duplicate_crates += 1;
            
            if conflicts > 0  {
                total_conflict_crates += 1;
            }
            println!(" - [ ] - {}   - {}   - duplicates {} conflicts {}", crate_from_idx.name(), duplicate_list.join(","), duplicates, conflicts);
        }
    }
    println!("Total crates with duplicates: {}, checksum conflicts: {}", total_duplicate_crates, total_conflict_crates);
}
