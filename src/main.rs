use std::collections::HashSet;

fn pattern_count(text: &str, pattern: &str) -> u32 {
    let mut count = 0;

    // Find the number of occurrences of the pattern in the text
    for i in 0..=text.len() - pattern.len() {
        // println!("i: {} [{} - {}]",i,  &text[i..i + pattern.len()], pattern);
        if &text[i..i + pattern.len()] == pattern {
            count += 1;
        }
    }
    count
}

fn frequent_words (text: &str, k: usize) -> Vec<String> {
    let mut frequent_patterns = HashSet::new();
    //create empty array of integers
    for i in 0..=text.len() - k {
        let pattern = &text[i..=i + k];


    }
    frequent_patterns

}

fn main() {
    println!("Pattern Count: {}", pattern_count("AATCTGATTAAGCTGCTATTAAGCATGAAACTGATTAAGCTTTGAGCTATTAAGCTACACAATTAAGCTCATTAAGCAAATTAAGCTCATTAAGCGATTAAGCCCCATTAAGCTATTAAGCATTAAGCAGTATTAAGCACAATTAAGCGGATTAAGCGTATTAAGCGCCATTAAGCTATTAAGCGGATCATTAAGCATTAAGCGCATTAAGCCATTAAGCTATCAATTAAGCTCTAATTAAGCCCACATTATTAAGCTGATATTAAGCACATTAAGCCCATTAAGCCAGGGGCCCCTCGCTGATTAAGCTTTATTAAGCCGATGATTAAGCCATTAAGCATTAAGCTATTAAGCGATTAAGCATTAAGCACCCATTAAGCGATTAAGCATTAAGCATTAAGCCCGGAATTTCCCATTAAGCATTAAGCAGATTAAGCACTCGATTAAGCCAATATTAAGCATTAAGCCATTAAGCCACGGTTATTAAGCTATTAAGCATTAAGCGCTCCATTAAGCACGCCATTAAGCTCATTAAGCATTAAGCTGGTTATTAAGCGGATTAAGCATTAAGCCCATTAAGCATTAAGCAATATTAAGCATTAAGCTCCTTGTATTAAGCGCATTAAGCGATTAAGCTTATTAAGCATTAAGCGCCTCCAATCGATTAAGCGCCGAATTAAGCGAATGGAATTAAGCGATTAAGCGATTAAGCCAATTAAGCGTATTAAGCATAGATTAAGCTGTAAGCGATCCCAATTAAGCATTAAGCATTAAGCGATTAAGCATTAAGCTTGATTAAGCATTAAGCATAGCTATTAAGCCCTCCCATTAAGCGATGTCCTGCTCGATTAAGCCAGATTAAGCATAATTAAGCGGATCATTAAGCCATTAAGCATTAAGCATTAAGCCCGATTAAGCATTAAGCCTGGTATTAAGCATTAAGC", "ATTAAGCAT"));
    println!("Frequent Words: {:?}", frequent_words("ACTGACTCCCACCCC ", 3));
}
