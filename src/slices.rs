fn first_word (s: &String) -> usize {
    /*
    write a function that takes a string and returns the first word it finds in that string.
    If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned
    */
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_world_with_slice (s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // == [0..i]
            return &s[..i];
        }
    }
    // == [0..s.len()]
    &s[..]
}

pub fn try_slices() {
    let my_string = "okok fo";
    let word = first_world_with_slice(&my_string);
    println!("Ok {}", word);

}