use std::collections::HashMap;

pub fn create_hasp_map() {
    let mut hm: HashMap<&str, i8> = HashMap::new();
    hm.insert("a", 10);
    hm.entry("b").or_insert(1);

    println!("hm value: {}", hm.get("b").copied().unwrap());
}
