pub fn generate_unique_word(list: Vec<String>) -> String {
    let mut data_words: Vec<String> = Vec::new();
    for word in list {
        data_words.push(word.to_string());
    }
    data_words.sort();
    data_words.dedup();
    data_words.join(", ")
}
