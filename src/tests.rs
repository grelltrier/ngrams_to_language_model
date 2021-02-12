use super::*;
use crate::utilities::LinesIterator;

#[test]
fn test_generation() {
    let test_mode = true;
    let max_no_words = 100_000;

    let root = "./LanguageModel_TEST/";
    let folder_result = "ngrams_result/";

    generate(test_mode, max_no_words);

    // Check processing 1-grams
    let correct_content = vec!["-0.6931472 0 1".to_string(), "-0.6931472 1 2".to_string()];
    let fname = format! {"{}{}{}", root,folder_result,"1gms.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);

    // Check processing 2-grams
    let correct_content = vec![
        "1 -0.40546507 0 2".to_string(),
        "0 -0.40546507 2 1".to_string(),
        "1 -1.0986123 3 1".to_string(),
    ];
    let fname = format! {"{}{}{}", root,folder_result,"2gms.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);

    // Check processing 3-grams
    let correct_content = vec![
        "0 -0.6931472 1".to_string(),
        "1 -0.6931472 2".to_string(),
        "1 -0.6931472 0".to_string(),
        "0 0 1".to_string(),
    ];
    let fname = format! {"{}{}{}", root,folder_result,"3gms.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);

    // Check result for symbt
    let correct_content = vec!["a".to_string(), "b".to_string()];
    let fname = format! {"{}{}{}", root,folder_result,"symt.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);
}
