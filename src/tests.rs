use super::*;
use crate::utilities::LinesIterator;

#[test]
fn test_generation() {
    let root = "./LanguageModel_TEST/";
    let folder_processing = "ngrams_processing/";
    let folder_result = "ngrams_result/";

    let test_mode = true;
    let max_no_words = 100_000;
    generate(test_mode, max_no_words);

    // Check processing 2-grams
    let correct_content = vec![
        "4 1 2 2".to_string(),
        "5 2 1 2".to_string(),
        "6 2 2 1".to_string(),
    ];
    let fname = format! {"{}{}{}", root,folder_processing,"2gms.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);

    // Check processing 3-grams
    let correct_content = vec![
        "1 2 1 1".to_string(),
        "1 2 2 1".to_string(),
        "2 1 2 1".to_string(),
        "2 2 1 1".to_string(),
    ];
    let fname = format! {"{}{}{}", root,folder_processing,"3gms.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);

    // Check result for 1-grams
    let correct_content = vec!["2 3".to_string(), "3 3".to_string()];
    let fname = format! {"{}{}{}", root,folder_result,"1gms.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);

    // Check result for 2-grams
    let correct_content = vec![
        "2 4 2 2".to_string(),
        "3 5 1 2".to_string(),
        "3 6 2 1".to_string(),
    ];
    let fname = format! {"{}{}{}", root,folder_result,"2gms.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);

    // Check result for 2-grams_eps
    let correct_content = vec![
        "4 3 0 0".to_string(),
        "5 2 0 0".to_string(),
        "6 3 0 0".to_string(),
    ];
    let fname = format! {"{}{}{}", root,folder_result,"2gms_eps.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);

    // Check result for 3-grams
    let correct_content = vec![
        "4 5 1 1".to_string(),
        "4 6 2 1".to_string(),
        "5 4 2 1".to_string(),
        "6 5 1 1".to_string(),
    ];
    let fname = format! {"{}{}{}", root,folder_result,"3gms.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);

    // Check result for 3-grams_eps
    let fname = format! {"{}{}{}", root,folder_result,"3gms_eps.txt"};
    let mut lines = LinesIterator::new(&fname);
    assert!(lines.next() == None);

    // Check result for symbt
    let correct_content = vec!["a	1".to_string(), "b	2".to_string()];
    let fname = format! {"{}{}{}", root,folder_result,"symbt.txt"};
    let mut lines = LinesIterator::new(&fname);
    for line in correct_content {
        assert!(lines.next() == Some(line));
    }
    assert!(lines.next() == None);
}
