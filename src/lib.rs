use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::time::Instant;

#[cfg(test)]
mod tests;
pub mod utilities;

use utilities::*;

pub fn generate(test_mode: bool, max_no_words: usize) {
    // start the clock
    let time_start = Instant::now();

    let root = if test_mode {
        "./LanguageModel_TEST/"
    } else {
        "./LanguageModel/"
    };

    // Folders in which the ngrams and the dictionary resides in
    let folder_all = "ngrams_ALL/";
    //let folder_processing = "ngrams_processing/";
    let folder_result = "ngrams_result/";
    let folder_dict = "dict/";

    // Create the directory for the translated ngrams if it does not exist
    if fs::create_dir(&format!("{}{}", root, folder_result)).is_err() {
        println!("Folder \'./{}\' already existed!", folder_result)
    };

    // Open the file with the dictioary
    let fname_dict = format! {"{}{}{}", root,folder_dict,"words_allow.txt"};
    let all_allowed_words = WordListIterator::new(&fname_dict);

    // Open the file with the unigrams
    let fname_read_unigrams = format!("{}{}{}gms.txt", root, folder_all, 1);
    let all_unigrams = NGramIterator::new(&fname_read_unigrams, 1);

    // Open the file with the bigrams
    let fname_read_bigrams = format!("{}{}{}gms.txt", root, folder_all, 2);
    let all_bigrams = NGramIterator::new(&fname_read_bigrams, 2);

    // Open the file with the trigrams
    let fname_read_trigrams = format!("{}{}{}gms.txt", root, folder_all, 3);
    let all_trigrams = NGramIterator::new(&fname_read_trigrams, 3);

    // Create file to write the unigrams to
    let fname_write_unigrams = format!("{}{}{}gms.txt", root, folder_result, 1);
    let mut f_write_unigrams = std::fs::File::create(fname_write_unigrams).expect("create failed");

    // Create file to write the bigrams to
    let fname_write_bigrams = format!("{}{}{}gms.txt", root, folder_result, 2);
    let mut f_write_bigrams = std::fs::File::create(fname_write_bigrams).expect("create failed");

    // Create file to write the trigrams to
    let fname_write_trigrams = format!("{}{}{}gms.txt", root, folder_result, 3);
    let mut f_write_trigrams = std::fs::File::create(fname_write_trigrams).expect("create failed");

    // Create file to write the symbol table to
    let fname_write_symt = format!("{}{}symt.txt", root, folder_result);
    let mut f_write_symt = std::fs::File::create(fname_write_symt).expect("create failed");

    // Process n-grams of lengths up to
    let max_ngram_len = 3;

    // Create Vec to keep track of how many ngrams were read in total and how big their accumulated count was
    // and a vec to store the same values, but only for the ngrams that were kept
    let mut ngrams_kept = vec![(0, 0); max_ngram_len];
    let mut ngrams_total = vec![(0, 0); max_ngram_len];

    // Load the dictionary of allowed words from its file
    println!("Load dictionary");
    let dictionary: HashSet<String> = all_allowed_words.collect();

    // Intersect the allowed words from the dictionary with the unigrams
    println!("Intersecting dictionary with unigrams");

    let mut threshold = 0;
    let mut allowed_unigrams = Vec::new();
    allowed_unigrams.reserve(max_no_words); // Reserve space for the specified max
    let mut min_heap = LimitedMinHeap::new(max_no_words);
    // We go through all of the unigrams and for each of them..
    for (ngram, ngram_count) in all_unigrams {
        ngrams_total[0].0 += 1;
        ngrams_total[0].1 += ngram_count;
        // We check if the unigram is in our list of allowed words
        // If it is not in the list, we ignore it and go to the next unigram
        if !dictionary.contains(&ngram[0]) {
            continue;
        }
        // We only reach this part if the unigram is one of the allowed words

        // We build a list of the k highest occurrences of the ngrams
        if let Some(new_k_highest_count) = min_heap.insert(ngram_count) {
            threshold = new_k_highest_count;
        }
        // If the count of the ngram is lower than the count of the theshold we can already ignore the ngram and skip to the next
        if ngram_count < threshold {
            continue;
        }
        // At this point it is guaranteed the unigram is in the list of allowed words and it's count is greater than the current threshold
        // The threshold can potentially increase with later unigrams
        // We temporarily store the unigrams in a Vec because we need to check if they truely meet the threshold again after going through all of them
        allowed_unigrams.push((ngram[0].clone(), ngram_count));
    }

    // All unigrams that don't meet the final threshold are removed and the SymbolTable created. It is kept in a HashMap and is also written to a file
    let mut unigrams = Vec::new();
    let mut sybt = HashMap::new();
    for (unigram, count) in allowed_unigrams {
        if count >= threshold {
            sybt.insert(unigram.clone(), ngrams_kept[0].0);
            writeln!(f_write_symt, "{}", unigram).expect("write failed");
            unigrams.push(count);
            ngrams_kept[0].0 += 1;
            ngrams_kept[0].1 += count;
        }
    }
    let allowed_unigrams = unigrams;

    // Mapping all symbols of the unigrams that meet the threshold to an integer value to save space and writing the unigrams to a file
    let mut unigrams: Vec<(f32, u32, u32, u16)> = Vec::new();
    let mut log_prob;
    for ngram_count in allowed_unigrams {
        // Calculate the log probability
        log_prob = (ngram_count as f32 / ngrams_kept[0].1 as f32).ln();
        // Insert the infos for the unigram (label, log_probability, offset_bigram, no_bigrams)
        unigrams.push((log_prob, ngram_count, 0, 0));
    }
    println!("Done reading the 1grams!");
    print_stats(time_start, ngrams_kept[0], ngrams_total[0]);

    // ########## Starting with bigrams ##############
    println!("Translating bigrams");

    // Go through the ngrams with increasing lengths
    let mut bigrams: BTreeMap<(u32, u32), (u32, f32, u32, u32, u16)> = BTreeMap::new();
    let mut translated_symbols = Vec::new(); // Temporarily store the translated symbols for the ngrams
    let mut last_found_prefix = None;
    let mut no_bigrams = 1;
    let mut count_prefix = 0;
    'bigram_loop: for (words, ngram_count) in all_bigrams {
        translated_symbols.clear();
        ngrams_total[1].0 += 1;
        ngrams_total[1].1 += ngram_count;
        for word in words {
            if let Some(&id) = sybt.get(&word as &str) {
                translated_symbols.push(id);
            } else {
                continue 'bigram_loop;
            }
        }
        // If all of the words are valid, we found another valid ngram

        // If the last prefix was not the same as the current one,
        if Some(translated_symbols[0]) != last_found_prefix {
            // Since the prefix changed, we know we found the last bigram with the prefix so we write the number of bigrams to the previous unigram
            if let Some(prev_unigram) = last_found_prefix {
                unigrams[prev_unigram as usize].3 = no_bigrams;
                no_bigrams = 1; // We reset the number of unigrams with that prefix
            }

            last_found_prefix = Some(translated_symbols[0]); // we store the new found prefix
            count_prefix = unigrams[translated_symbols[0] as usize].1;
            unigrams[translated_symbols[0] as usize].2 = ngrams_kept[1].0; // we found the offset for the unigram table
        } else {
            no_bigrams += 1; // If the prefix did not change, we found another one with the same prefix, so we increase the number by one
        }

        log_prob = (ngram_count as f32 / count_prefix as f32).ln();

        bigrams.insert(
            (translated_symbols[0], translated_symbols[1]),
            (ngrams_kept[1].0, log_prob, ngram_count, 0, 0),
        );
        ngrams_kept[1].0 += 1;
        ngrams_kept[1].1 += ngram_count;
    }

    // Add the offset and the no of bigrams for the last bigram to the unigram table
    if let Some(prev_unigram) = last_found_prefix {
        unigrams[prev_unigram as usize].3 = no_bigrams;
    }

    println!("Done reading the bigrams!");
    print_stats(time_start, ngrams_kept[1], ngrams_total[1]);

    println!("Writing unigrams to file");
    for (log_prob, _, offset_longer_ngram, no_longer_ngram) in unigrams {
        writeln!(
            f_write_unigrams,
            "{} {} {}",
            log_prob, offset_longer_ngram, no_longer_ngram,
        )
        .expect("write failed");
    }
    println!("Done writing unigrams to file");
    println!();

    // ########## Starting with trigrams ##############
    println!("Translating trigrams");

    // Go through the ngrams with increasing lengths
    let mut trigrams = Vec::new();
    let mut translated_symbols = Vec::new(); // Temporarily store the translated symbols for the ngrams
    let mut last_found_prefix: Option<(u32, u32)> = None;
    let mut no_trigrams = 1;
    let mut count_prefix = 0;
    'trigram_loop: for (words, ngram_count) in all_trigrams {
        translated_symbols.clear();
        ngrams_total[2].0 += 1;
        ngrams_total[2].1 += ngram_count;
        for word in words {
            if let Some(&id) = sybt.get(&word as &str) {
                translated_symbols.push(id);
            } else {
                continue 'trigram_loop;
            }
        }
        // If all of the words are valid, we found another valid trigram

        // If the last prefix was not the same as the current one,
        //                     if let Some(prev_prefix) = last_found_prefix {
        //                       if (translated_symbols[0], translated_symbols[1]) != prev_prefix {

        if Some((translated_symbols[0], translated_symbols[1])) != last_found_prefix {
            // Since the prefix changed, we know we found the last trigram with the prefix so we write the number of trigrams to the previous bigram
            if let Some(prev_bigram) = last_found_prefix {
                let mut prev_bigram = bigrams.get_mut(&prev_bigram).unwrap();
                (*prev_bigram).4 = no_trigrams;
                no_trigrams = 1; // We reset the number of trigrams with that prefix
            }

            // Since the prefix changed, we know we found the last bigram with the prefix so we write the number of bigrams to the previous unigram
            // bigrams.get_mut(&prev_prefix).unwrap().3 = no_bigrams;
            last_found_prefix = Some((translated_symbols[0], translated_symbols[1])); // we store the new found prefix
            let bigram_entry = bigrams
                .entry((translated_symbols[0], translated_symbols[1]))
                .or_default();

            bigram_entry.3 = ngrams_kept[2].0; // we found the offset for the bigram table
            count_prefix = bigram_entry.2;
        } else {
            no_trigrams += 1; // If the prefix did not change, we found another one with the same prefix, so we increase the number by one
        }
        log_prob = (ngram_count as f32 / count_prefix as f32).ln();

        let idx_suffix = bigrams
            .get_mut(&(translated_symbols[1], translated_symbols[2]))
            .unwrap()
            .0;

        trigrams.push((translated_symbols[2], log_prob, idx_suffix));

        ngrams_kept[2].0 += 1;
        ngrams_kept[2].1 += ngram_count;
    }

    // Add the offset and the no of trigrams for the last trigram to the bigram table
    if let Some(prev_bigram) = last_found_prefix {
        let mut prev_bigram = bigrams.get_mut(&prev_bigram).unwrap();
        (*prev_bigram).4 = no_trigrams;
    }

    println!("Done reading the trigrams!");
    print_stats(time_start, ngrams_kept[2], ngrams_total[2]);

    // Writing to files
    println!();
    println!("Starting to write ngrams to files");
    println!("Writing bigrams to file!");

    // Write bigrams to file
    for ((_, label), (_, log_prob, _, offset_longer_ngram, no_longer_ngram)) in bigrams {
        writeln!(
            f_write_bigrams,
            "{} {} {} {}",
            label, log_prob, offset_longer_ngram, no_longer_ngram,
        )
        .expect("write failed");
    }
    println!("Done writing bigrams to file!");

    println!("Writing trigrams to file!");
    // Write trigrams to file
    for (label, log_prob, offset_unigram_referring_to_bigram) in trigrams {
        writeln!(
            f_write_trigrams,
            "{} {} {} ",
            label, log_prob, offset_unigram_referring_to_bigram,
        )
        .expect("write failed");
    }
    println!("Done writing trigrams to file!");
}

fn print_stats(time_start: Instant, ngrams_kept: (u32, u32), ngrams_total: (u32, u32)) {
    let time_end = Instant::now();
    let duration = time_end.saturating_duration_since(time_start);
    println!("Time passed since start: {:?}", duration);

    let ngrams_skipped = (
        ngrams_total.0 - ngrams_kept.0,
        ngrams_total.1 - ngrams_kept.1,
    );
    println!(
        "{} ngrams with a cumulative count of {} were skipped",
        ngrams_skipped.0, ngrams_skipped.1
    );
    println!(
        "{} ngrams with a cumulative count of {} were kept",
        ngrams_kept.0, ngrams_kept.1
    );
    println!(
        "In other words {:.3}% of the ngrams were skipped, which made up {:.3}% of the total count",
        ngrams_skipped.0 as f32 / (ngrams_kept.0 as f32 + ngrams_skipped.0 as f32) * 100.0,
        ngrams_skipped.1 as f32 / (ngrams_kept.1 as f32 + ngrams_skipped.1 as f32) * 100.0,
    );
    println!();
}
