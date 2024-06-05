pub fn count_words(text: &str) -> i32 {
    text.split(" ").count() as i32
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_little_lamb_poem() {
        assert_eq!(5, count_words("marry had a little lamb"))
    }


}