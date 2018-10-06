fn main() {
    let s = String::from("hello world pewpew");
    let fw = first_word(&s);
    let sw = second_word(&s);
    let lw = nth_word(&s, 3);
    println!("first word: {}, second word: {}, last word: {}", fw, sw, lw);
}

fn first_word(s: &str) -> &str {
    nth_word(s, 1)
}

fn second_word(s: &str) -> &str {
    nth_word(s, 2)
}

fn nth_word(s: &str, n: usize) -> &str {
    let bytes = s.trim().as_bytes();
    let len = s.len();
    let mut count = 1;
    let mut start = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if count == n {
                return &s[start..i];
            }
            start = i + 1;
            count += 1;
        }

        if i == len - 1 {  // last word
            return &s[start..];
        }
    }

    &s[..]
}
