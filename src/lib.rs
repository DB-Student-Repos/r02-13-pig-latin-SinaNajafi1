const VOWEL: [&str; 7] = ["a", "e", "i", "o", "u", "xr", "yt"];

pub fn translate(input: &str) -> String {
    let words = input.split_whitespace().filter(|x| !x.is_empty());
    let mut result = String::new();
    for word in words {
        let mut w = word.to_string();
        if !VOWEL.iter().any(|v| w.starts_with(v)) {
            let range = match w.chars().collect::<Vec<char>>()[..3] {
                [ _ , 'q', 'u'] => 3,
                [ _ , 'c', 'h'] => 3,
                ['t', 'h', 'r'] => 3,
                ['q', 'u',  _ ] => 2,
                [ _ , 'h',  _ ] => 2,
                _ => 1,
            };
            for _ in 0..range {
                let c = w.remove(0);
                w.push(c);
            }
        }
        
        w.push_str("ay ");
        result.push_str(&w);
    } //for the whole phrase test ("ay ") and .trim()
    result.trim().to_string()
}
