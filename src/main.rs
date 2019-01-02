use std::cmp;

const EXPRESSION_MAX_LENGTH: u8 = 4;
const WORDS: [&str; 34] = [
    "hello",
    "how",
    "are",
    "you",
    "hi",
    "hello",
    "hi",
    "you",
    "hi",
    "hello",
    "what's",
    "up",
    "hi",
    "bro",
    "what",
    "what's up",
    "up",
    "up",
    "uhh",
    "hello?",
    "hmm",
    "bye",
    "noob",
    "idiot",
    "noob",
    "ples",
    "pls",
    "yeah",
    "ik",
    "uh",
    "btw",
    "no",
    "maybe",
    "hello",
];

fn main() {
    let mut expressions_agg: Vec<(String, u32)> = Vec::new();
    let mut word_index: usize = 0;

    if EXPRESSION_MAX_LENGTH as u32 > WORDS.len() as u32 {
        println!("Expression length must be smaller than sample size");
        return;
    }

    // Iterate through every individual word
    for word in WORDS.iter() {

        // Get n expressions for each word, i.e. the word plus the n-1 following words,
        // where n is EXPRESSION_MAX_LENGTH
        // e.g. ["hi", "what's", "up", "hi"] with EXPRESSION_MAX_LENGTH=2 yields
        // [("hi", 2), ("hi what's", 1), ("what's", 1), ("what's up", 1), ("up", 1), ("up hi", 1)]

        for size in 0..cmp::min(EXPRESSION_MAX_LENGTH as usize, WORDS.len() - word_index) {
            let mut expression = word.to_string();
            for i in 1..cmp::min(size as usize + 1, WORDS.len() - word_index) {
                let index = word_index + (i as usize);
                if index <= WORDS.len() - 1 {
                    expression.push_str(" ");
                    expression.push_str(WORDS[index]);
                }
            }
            let expressions_flat: Vec<String> = expressions_agg
                .iter()
                .map(|tup| tup.0.to_string())
                .collect::<Vec<String>>();
            if expressions_flat.contains(&expression) {
                // find it and increment count
                let ind = expressions_flat.iter().position(|r| r == &expression).unwrap();
                expressions_agg[ind].1 += 1
            } else {
                // add
                expressions_agg.push((expression, 0));
            }
        }

        // track word index manually to avoid expensive index calculations
        word_index += 1;
    }

}
