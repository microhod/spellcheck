use std::fs::read_to_string;

mod bloom;

fn main() -> Result<(), String> {
    let dict = match load_dictionary("/usr/share/dict/words") {
        Ok(dict) => dict,
        Err(err) => return Err(err),
    };

    let mut filter = bloom::BloomFilter::new(2 * dict.len());
    for word in dict {
        filter.add(&word);
    }

    println!("cat: {}", filter.query("cat"));
    println!("dog: {}", filter.query("dog"));
    println!("fish: {}", filter.query("fish"));
    println!("poop: {}", filter.query("poop"));
    println!("hey: {}", filter.query("hey"));
    println!("ohdchochhios: {}", filter.query("ohdchochhios"));
    println!("pooper the goober: {}", filter.query("pooper the goober"));

    return Ok(())
}

fn load_dictionary(path: &str) -> Result<Vec<String>, String> {
    let data = match read_to_string(path) {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };

    let mut words = Vec::new();
    for line in data.lines() {
        words.push(line.to_string())
    }
    return Ok(words)
}
