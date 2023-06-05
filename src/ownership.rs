#![allow(dead_code)]

pub fn introduce() {
    println!("Ownership is a method to ensure the safety of Rust code.");
    
    //if a program is safe, all of its behaviors are defined

    //this program is safe, but if these two lines were swapped it wouldn't be
    let msg = "Hello, World!";
    this_is_safe(msg);

    explain_ownership();
}


//other languages check if variables are defined at run-time: rust does at compile time
//good because it's faster: program doesn't need to check if a var exists, which costs performance

//Some of Rust's big goals are:
    //ensure all programs are safe (no undefined behavior)
    //check for undefined behavior at compile-time, instead of runtime (so you don't have to look for it)

//see explanation slides for more
fn this_is_safe(msg: &str) {
    if msg.len() < 9 {
        println!("Here's a short message: {msg}");
    } else {
        println!("Here's a long message: {msg}");
    }
}


fn explain_ownership() {
    println!("So how does ownership help us prevent these 'undefined behaviors?'");

    //Rust provides a new way of thinking about memory, 
    //and ownership allows safe memory use within that way of thinking.

    explain_rust_memory();
}


fn explain_rust_memory() {
    println!("Check out the explanation document to find out how Rust handles memory!");

    //1: String "nine" has owner no_im_not
    let no_im_not = String::from("nine");

    //4: number's ownership has been transferred to stupid
    let stupid = add_ten(no_im_not);
    println!("What's {stupid}");

    //This is an error - remember no_im_not has been dealloc'
    //println!("{no_im_not}");
    //It's okay that it points to nothing, but we cannot use it no matter what

    //You CANNOT USE variables that have their OWNERSHIP MOVED

    make_it_safe();
}


//2: no_im_not ownership transferred to number
fn add_ten(mut number: String) -> String {
    //3: this creates a new heap alloc and deletes the original: now no_im_not points to dealloc'd memory
    number.push_str(" plus ten"); 
    number
}


//However, we can use cloning to make the program safe
fn make_it_safe() {
    let quick_maths = String::from("nine");
    let quick_maths_clone = quick_maths.clone();
    let im_smart = add_ten(quick_maths_clone);

    println!("What's {im_smart}?");
    println!("It's {quick_maths}teen.");

    //The clone can't be referenced now, though.
    //println!("{quick_maths_clone}");

    //END
} 

/*
 *
 * GO
 * BACK
 * TO
 * THE
 * MAIN
 * SLIDE
 * 
 */


pub fn recap() {
    //But, why do all of these exist in the first place?

    the_alternative();
}


fn the_alternative() {
    //Garbage collection.
    //Usually, these work by scanning through memory to find data that is no longer used, then deallocs
    //While these avoid undefined behavior like in C/C++, they have a performance cost
    //They also can be unpredictable, because the language masks pointers
    
    //Rust's ownership model 
    type Document = Vec<String>;

    fn new_document(words: Vec<String>) -> Document {
        words
    }
    
    fn add_word(this: &mut Document, word: String) {
        this.push(word);
    }
    
    fn get_words(this: &Document) -> &[String] {
        this.as_slice()
    }
}