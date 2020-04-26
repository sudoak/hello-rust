use std::collections::HashMap;

pub fn get_hash_map_color(){
    let mut colors = HashMap::new();
    colors.insert(String::from("Yellow"), 20);
    colors.insert(String::from("Pink"), 200);
    println!("Al colors are = {:?}", colors);

    let some_color = colors.get(&String::from("Pink"));
    println!("Some color {:?}", some_color);
}

pub fn has_map_from_vec() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}",scores);
}

pub fn count_words_in_a_sentence() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}