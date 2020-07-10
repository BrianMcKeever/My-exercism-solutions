pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if list.is_empty() {
        return proverb;
    }
    let mut iter = list.iter();
    let mut previous_word = iter.next().expect("Empty list");
    for word in iter {
        proverb.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            previous_word, word
        ));
        previous_word = word;
    }
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    proverb
}
