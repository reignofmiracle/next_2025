fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("{word}");

    let word2 = first_word2(&s);

    println!("{word2}");

    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn f1() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn f2() {
    let my_string = String::from("hello world");

    let word = first_word2(&my_string[0..6]);
    let word = first_word2(&my_string[..]);

    let word = first_word2(&my_string);

    let my_string_literal = "hello world";

    let word = first_word2(&my_string_literal[0..6]);
    let word = first_word2(&my_string_literal[..]);

    let word = first_word2(&my_string_literal);
}
