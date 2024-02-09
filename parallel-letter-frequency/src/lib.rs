use std::collections::HashMap;
use std::thread;
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => HashMap::new(), // if there are no lines, there's no frequencies
        n if n < 100 => single_thread_freq(input), // not worth spinning up for fewer lines
        _ => {
            // if more than 100 lines
            // here we use the multi thread approach
            thread::scope(|s| {
                // scope makes sure references are valid throughout all threads until scope ends
                let mut handles = Vec::with_capacity(worker_count); // enough handles cap for num workers
                let chunk_size = input.len() / worker_count + 1; // +1 makes num chunks == num workers
                for lines in input.chunks(chunk_size) {
                    handles.push(s.spawn(|| single_thread_freq(lines)))
                }

                // initialise empty map
                let mut map: HashMap<char, usize> = HashMap::new();
                // now we join handles
                for handle in handles {
                    handle.join().unwrap().into_iter().for_each(|(k, v)| {
                        *map.entry(k).or_insert(0) += v;
                    })
                }
                map
            })
        }
    }
}

fn single_thread_freq(input: &[&str]) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    let combined_string = input.iter().fold(String::new(), |acc, x| acc + x); // fold into single string
    for letter in combined_string.chars() {
        if letter.is_alphabetic() {
            *result.entry(letter.to_ascii_lowercase()).or_insert(0) += 1
        }
    }

    result
}
