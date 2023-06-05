#![allow(dead_code)]
#![allow(unused)]

pub fn explain() {
    //A slice is a reference to a contiguous series of elements in a collection. 
    //It's a type of reference (non-owning pointer)
    let mut str = String::from("barfoo foo baz bar qux fooqux barbaz");

    //Seems okay, but there's a problem: the usize returned only matters if we know what was passed in
    let first_word_end_pos = the_problem(&str);

    //Now it means nothing!
    str.clear();
    //How would we keep the first word in sync?

    let str2 = String::from("会ったことがない|Never met 'em");
    let slices = showcase_string_slices(&str2);
    println!("Japanese: {}\nEnglish: {}", slices.0, slices.1);

    explain_string_literals();
}


//This function accepts a list of words separated by spaces, and returns the first word...
//Wait, how would we return a part of a string?
//You know what, whatever. We'll just return the index position where the word ends (the space after).  
fn the_problem(s: &String) -> usize {
    //Gonna check every string
    let bytes = s.as_bytes();

    //check it's byte
    for (i, &item) in bytes.iter().enumerate() {
        //see if the character is space
        if item == b' ' {
            //then return the index position.
            return i;
        }
    }
    //We'll talk about iterators more later.

    //Oh, if you can't find a space just return the full length of the string
    s.len()
}


//A reference to a part of a string
fn showcase_string_slices(string: &String) -> (&str, &str) {
    let sep_index = string.clone()
        .find('|')
        .expect("Expected to find first '|' in input String");

    let slice: &str = &string[0..sep_index];
    let slice2: &str = &string[sep_index + 1..string.len()];

    //Slices are special references called 'fat pointers,' which means they have metadata
    //in a String slice, the metadata is the length of the slice
    //Because of this, it takes more bytes to make a string slice than a normal reference to a string
    //(extra 8 bytes for the length of the slice)

    //(unrelated: A string is actually a vector of u8's (bytes) with a ptr, cap, and len)
    //(also a range (the thing used to declare where a string slice should start and end) can
    // drop the 0/ending index if its the first/last index in the range)

    //So by doing this nothing actually changes
    let slice = &string[..sep_index];
    let slice2 = &string[sep_index + 1..];

    //And this is a slice of the entire string
    let full_slice = &string[..];

    let mut generic_string = String::from("I_can't_think_of_any_other_ways_to_say 'Hello World!'");
    
    //Now this references a value tied to the underlying (generic) string
    let ecils_backwards: &str = gimme_a_slice(&generic_string);
    
    //So we can still do this:
    println!("This is what we cut off: {ecils_backwards}");
    //But we can't do this and get confused when ecils doesn't give us the right part of generic_string
    generic_string.clear();
    //println!("For the people in the back, THIS IS WHAT WE CUT OFF: {ecils_backwards}");

    return (slice, slice2)
}


//&str is the string slice type (not str believe it or not)
fn gimme_a_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut slice_point = s.len() / 2;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            slice_point = i / 2;
        }
    }

    &s[..slice_point]
}


fn explain_string_literals() {
    //String literals are slices. It points to the specific point of binary that would form this string
    //That's why they're also immutable: to change this would be to make it point to different memory
    let bad_joke = "a literal slice";

    //Any function that uses String should use &str instead for better functionality 
    //(this works because of dereference coercions we'll get to that later)
    showcase_other_slices()
}


fn showcase_other_slices() {
    //Obviously string slices are unique to strings, but there's a general slice type:
    let slice_me: [u8; 6] = [7, 6, 11, 16, 0, 206]; //All these numbers have something to do with cutting

    //Ka Ta Na
    //Works the same way as string slices: stores reference to the part of the array
    let slice: &[u8] = &slice_me[1..4];
}