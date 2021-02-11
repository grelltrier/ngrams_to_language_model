fn main() {
    let max_no_words = 30_000;
    let test_mode = false;
    ngrams_to_language_model::generate(test_mode, max_no_words);
}
