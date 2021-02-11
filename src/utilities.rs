use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Lines;
use std::io::{BufRead, BufReader};

pub struct LimitedMinHeap {
    min_heap: BinaryHeap<Reverse<u32>>,
    max_size: usize,
}

impl LimitedMinHeap {
    /// Create a new min_heap with its size limited
    pub fn new(max_size: usize) -> Self {
        let min_heap = BinaryHeap::new();
        Self { min_heap, max_size }
    }

    // Tries to add the count to the min_heap
    // Returns None, if it was not added because the min_heap is full and the count was not greater than the heaps lowest value
    pub fn insert(&mut self, value: u32) -> Option<u32> {
        // If the heap is not full, we add the new value to it
        if self.min_heap.len() < self.max_size {
            self.min_heap.push(Reverse(value));
        } else {
            // Get the lowest value from the heap
            // Return None, if there was no lowest value (This should only happen, when the heap is empty and has a max size of 0)
            let min_value = if let Some(Reverse(min)) = self.min_heap.peek() {
                *min
            } else {
                return None;
            };
            // If the new value is greater than the heaps lowest value, we add the new value and
            // remove the heap's lowest value so that we don't exceed the heaps maximum size
            // If it is not greater, we can not insert the new value and return None
            if value > min_value {
                self.min_heap.push(Reverse(value));
                self.min_heap.pop();
            } else {
                return None;
            };
        }
        // We return the new lowest value
        if let Some(Reverse(min)) = self.min_heap.peek() {
            Some(*min)
        } else {
            None
        }
    }
    pub fn peek(&mut self) -> Option<u32> {
        if let Some(Reverse(value)) = self.min_heap.peek() {
            Some(*value)
        } else {
            None
        }
    }
}

pub struct LinesIterator {
    lines: Lines<BufReader<File>>,
}

impl LinesIterator {
    pub fn new(filename: &str) -> Self {
        // Open the file in read-only mode.
        let file = File::open(filename).unwrap();
        let buf_reader = BufReader::new(file);
        let lines = buf_reader.lines();
        LinesIterator { lines }
    }
}

impl Iterator for LinesIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if let Some(Ok(line)) = self.lines.next() {
            Some(line)
        } else {
            None
        }
    }
}

pub struct WordListIterator {
    lines_iterator: LinesIterator,
}

impl WordListIterator {
    pub fn new(filename: &str) -> Self {
        WordListIterator {
            lines_iterator: LinesIterator::new(filename),
        }
    }
}

impl Iterator for WordListIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        // If the end of the file was reached, return None
        if let Some(line) = self.lines_iterator.next() {
            let word = line.trim();
            Some(word.to_string())
        } else {
            None
        }
    }
}

pub struct NGramIterator {
    lines_iterator: LinesIterator,
    n: usize,
}

impl NGramIterator {
    pub fn new(filename: &str, n: usize) -> Self {
        NGramIterator {
            lines_iterator: LinesIterator::new(filename),
            n,
        }
    }
}

impl Iterator for NGramIterator {
    type Item = (Vec<String>, u32);
    fn next(&mut self) -> Option<(Vec<String>, u32)> {
        // If the end of the file was reached, return None
        if let Some(line) = self.lines_iterator.next() {
            let mut token = line.split_whitespace();
            let mut words = Vec::new();
            for _ in 0..self.n as usize {
                words.push(token.next().unwrap().trim().parse::<String>().unwrap())
            }
            let count = token.next().unwrap().parse::<u32>().unwrap();
            Some((words, count))
        } else {
            None
        }
    }
}
pub struct NGramProcessedIterator {
    lines_iterator: LinesIterator,
    n: usize,
    is_longest_ngram: bool,
}

impl NGramProcessedIterator {
    pub fn new(filename: &str, n: usize, is_longest_ngram: bool) -> Self {
        Self {
            lines_iterator: LinesIterator::new(filename),
            n,
            is_longest_ngram,
        }
    }
}

pub type StateId = usize;
pub type Label = usize;
pub type Count = usize;

impl Iterator for NGramProcessedIterator {
    type Item = (Option<StateId>, Vec<Label>, Count);
    fn next(&mut self) -> Option<(Option<StateId>, Vec<Label>, Count)> {
        // If the end of the file was reached, return None
        if let Some(line) = self.lines_iterator.next() {
            let mut token = line.split_whitespace();
            let state_id = if self.is_longest_ngram {
                None
            } else {
                Some(token.next().unwrap().trim().parse::<StateId>().unwrap())
            };
            let label = if self.n as usize == 1 {
                vec![state_id.unwrap() - 1]
            } else {
                let mut label = Vec::new();
                for _ in 0..self.n as usize {
                    label.push(token.next().unwrap().trim().parse::<Label>().unwrap())
                }
                label
            };
            let count = token.next().unwrap().parse::<Count>().unwrap();
            Some((state_id, label, count))
        } else {
            None
        }
    }
}
