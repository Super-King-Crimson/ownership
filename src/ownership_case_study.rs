#![allow(unused)]
#![allow(dead_code)]


pub fn explain() {
    println!("How should you fix code that Rust rejects?");

    //Get ready to see a lot of code that doesn't compile.

    //Fixing unsafe programs
    case1();
    case2();
    case3();
    case4();

    //Fixing safe programs
    case5();
    case6();
}



//Case 1: Returning a reference from the heap to the stack
fn case1() {
    return_a_string();
}

//Why is this unsafe? Has to do with lifetimes.
fn return_a_string() /*-> &String*/ {
    //This string's gonna die at the end of this function,
    let s = String::from("Hello world");

    //So why are we trying to return a reference to it?
    //&s
    //To fix this issue, we have to extend the lifetime of the string  
}

//Move string ownership out of the function
fn solution1_1() -> String {
    let s = String::from("Hello world");
    s
}
//Return a string literal, which has an eternal lifetiem
fn solution1_2() -> &'static str {
    "Hello world"    
}
//defer borrow checking to runtime (basically garbage collecting)
use std::rc::Rc;
fn solution1_3() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}
//have caller of function provide a slot to put the returned string
fn solution1_4(output: &mut String) {
    output.replace_range(.., "Hello world");
}



//Case 2: Not enough permissions
fn case2() {
    let not_funny = vec![String::from("Johnson")];
    let semi_funny = &not_funny[0];
    //let doubly_not_funny = stringify_name_with_title(&not_funny);
    let doubly_not_funny = solution2_1(&not_funny);
    println!("{semi_funny}, {}, {doubly_not_funny}", not_funny[0]);
}

//Unsafe because name is immutable and it has to be because pushing could invalidate other references
fn stringify_name_with_title(name: &Vec<String>) -> String {
    //name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
//That's why just changing &Vec to &mut Vec doesn't solve the problem
fn faux_solution2_1(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
    //Also this would mutate the arguments, which is unexpected to a caller
}
//What about taking ownership of the string by having it passed directly?
fn faux_solution2_2(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
    //Nope, that means name can never be used again (remember references and borrowing?)
    //It's very rare for a function to take ownership of a heap-owned data struture
}

//Ok, so instead of changing the arguments to the functon, what about the body?

//Clone the reference so we don't override or take ownership
fn solution2_1(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}
//We can also reorganize the code inside to add the suffix later, saving the performance cost of a clone
fn solution2_2(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full    
}



//Case 3: Aliasing and mutating a data structure
fn case3() {
    let mut dst = vec![String::from("like"), String::from("a"), String::from("good"), String::from("neighbor")];
    let src = [String::from("State Farm")];
    add_big_strings(&mut dst, &src);
}

//This uses iterators and closures we don't know em just ignore em for now
//Gets the largest string in the dst vec, then pushes all strings in src bigger than the largest into dst
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter()
        .max_by_key(|s| s.len())
        .unwrap();

    for s in src {
        if s.len() > largest.len() {
            //Unsafe - largest is a borrow, so pushing could dealloc the vec and invalidate that borrow
            //dst.push(s.clone());
        }
    }
}

//To fix this, we need to shorten the lifetime of largest so it doesn't intersect with dst

//We could clone largest, but if largest is really large that might be a performance issue
fn semi_solution3_1(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
//We could do the length comparisons first, then add all the new strings after
//This also trades off some performance because we have to create the new to_add vector
fn semi_solution3_2(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = 
        src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
}
//The best solution is just noting the length of the largest, cuz that's all that matters
//Now largest is largest_len, which isn't a reference to anything and therefore doesn't sink dst's W perms
fn solution3(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}



//Case 4: Copying vs. Moving out of a collection
//A common confusion for Rust learners happens when copying data out of a collection
fn case4() {
    println!("{}", safe_copy());
}

fn safe_copy() -> i32 {
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];

    //Works fine here
    //Copying an i32 using *& doesn't copy a pointer: it copies the genuine number
    //This property is called 'implementing the Copy trait'
    *n_ref
}

fn unsafe_copy() /*-> String*/ {
    let v: Vec<String> = vec![String::from("Hello"), String::from("World")];
    let s_ref: &String = &v[0];
    
    //Doesn't work here, because the vec owns the string, but the dereference tries to take ownership
    //Since references are non-owning pointers, ownership can't be taken through them
    //Unsafe because both think they have ownership of "Hello": if both die "Hello" is freed twice
    //*s_ref
}

//In sum, if a value does not own heap data, then it can be copied without a move
//(exception: mutable references)

//So how do you get access to an element?
//Use immutable reference and don't take ownership
fn solution4_1() -> String {
    let v = vec![String::from("Hello world")];
    let v_ref = &v[0];
    println!("{}", v_ref);
    //We can't actually return this, return requires O perms
    // *v_ref

    String::from("Hello world")
}
//Clone the data to take ownership while leaving the vector alone 
fn solution4_2() -> String {
    let v: Vec<String> = vec![String::from("Hello world")];
    v[0].clone()
}
//Remove the data from the vector to transfer ownership
fn solution4_3() -> String {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    v.remove(0)
}



//Mutating different tuple fields
//Note that cases 5 and 6 are actually safe, but sometimes Rust will reject a safe program
fn case5() {
    solution5_1()
}

fn solution5_1() {
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );

    let first = &name.0;
    name.1.push_str(", Esq");
    println!("{first} {}", name.1);
}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}
fn safe_but_rejected_combine_names() {
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );

    let first = get_first(&name);

    //Rust doesn't look at the implementation of get_first, it just sees that some string gets borrowed
    //so it assumes both tuples were changed and slashes both of their W perms
    //name.1.push_str(", Esq");

    //To fix this: 
        //just inline the function (like in the original solution)
        //defer borrow checking to runtime with cells (we'll talk abt it later)
}



//Case 6: Mutating different array elements
//A similar kind of problem arises when we borrow elements of an array
fn case6() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0];
    *x += 1;
    println!("{a:?}");

    //The rust borrow checker doesn't have a different path for every index of a, it uses a[_] (every index)
    //sometimes it can't figure out what an index is (like when an index is found by a complex function)
    safe_but_rejected_array_read_write()
}

fn safe_but_rejected_array_read_write() {
    let mut a = [0, 1, 2, 3];

    //borrowed as mutable: a[_] has no RWO perms
    let x = &mut a[0];

    //requires R perms: if y is ever used, will violate Pointer Safety Principle (is what compiler thinks)
    let y = &a[1];

    //VIOLATED! (it's actually safe though. we see a[0] a[1], compiler sees  a[_] a[_])
    //*x += *y;

    //Even though this is safe, borrow checker limitations prevent
    //You can bypass Rust's borrow checker with some rust APIs, in this case slice::split_first_mut:
    let mut a = [0, 1, 2, 3];
    let (x, rest) = a.split_first_mut().unwrap();
    let y = &rest[0];
    *x += *y;

    //That function works using unsafe blocks:
    //Unsafe blocks allow the use of raw pointers: not safety-checked 
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0] as *mut i32;
    let y = &a[1] as *const i32;
    unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
}
