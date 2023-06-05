#![allow(dead_code)]

use std::io;
use rand::Rng;
use crate::vectors;

//Move-only API programming on the heap is inconvenient
pub fn explain() {
    let mut greeting = String::new();
    let mut farewell = String::new();
    
    println!("Enter a greeting: ");
    io::stdin().read_line(&mut greeting).expect("Input should have been read"); 
    greeting = String::from(greeting.trim());

    println!("Enter a farewell: ");
    io::stdin().read_line(&mut farewell).expect("Input should have been read");
    farewell = String::from(farewell.trim());

    //(greeting, farewell) = 
        greet_farewell(greeting, farewell);
    
    //println!("Good morning, {greeting}, Good night, {farewell}");
    //This is inconvenient error. I can only say hi and bye once in my entire life?
    explain_references();
}

fn greet_farewell(str1: String, str2: String) //-> (String, String) 
{
    let mut combined = str1.clone();
    combined.push_str(&str2);
    println!("{combined}!");

    //You could return ownership of the strings to prevent this, but that's lame
    //(str1, str2)
}


//
fn explain_references() {
    let mut input = String::new();

    println!("Enter an exciting word: ");
    io::stdin().read_line(&mut input).expect("Expected to get input");
    let input = String::from(input.trim());

    //What's that ampersand (&) doing there?
    emphasize(&input);
    println!("Who cares. It's just {input}.");

    //& marks a reference, which is a type of pointer
    explain_dereferencing();
}


//Another ampersand (&)
fn emphasize(word: &String) {
    //instead of taking ownership, this function creates a reference to the input variable
    //That means once this function ends, nothing is dealloc'd
    println!("Can you believe it guys? It's {word}!!!");

    //References are non-owning pointers, so using a reference is also called 'borrowing'
}


fn explain_dereferencing() {
    //println! can work with Strings and &Strings due to dereferencing (*): follows reference to data

    //Remember, a box is just memory on the heap, so a Box as a variable points to that memory

    let mut a: Box<i32> 
        = Box::new(1);      //a points to the value 1 on the heap 
    let b: i32 = *a;        //b grabbed the value straight from heap, b = 1
    *a += 1;                

    let c: &Box<i32> = &a;  //c points to a on the stack
    let d: i32 = **c;       //so go to a, then to the value

    let e: &i32 = &*a;      //e also points to the memory on the heap (it's basically a)
    let f: i32 = *e;        //so we grab the value

    
    println!("a: {a}, b: {b}, c: {c}, d: {d}, e: {e}, f: {f}");

    //Think of * as an arrow, so you travel to the thing a pointer points to
    //Think of & as a U turn, so you take a few steps back and point to the thing

    //Rust implicitly references and dereferences sometimes (for example, when using the dot operator)
    let x = Box::new(-42);
    let x_abs1 = i32::abs(*x);  //explicit dereference
    let x_abs2 = x.abs();  //implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let str = String::from("Hello");
    let str_len1 = str::len(&str);  //explicit reference
    let str_len2 = str.len();  //implicit reference
    assert_eq!(str_len1, str_len2);

    explain_aliasing();
}


fn explain_aliasing() {
    //Pointers are scary because they allow simultaneous aliasing and mutation

    //aliasing is accessing the same data through different variables
    let a_box = Box::new('A');
    let box_value1 = *a_box;
    let box_reference = &a_box;
    let box_value2 = **box_reference;

    if box_value1 == box_value2 {
        println!("They're the same.");    
    }

    if box_value1 == *a_box {
        println!("These are also the same.");    
    }

    //if aliased variables are changed or dealloc'd carelessly, can cause unsafe behavior
    //The next examples will use vectors, so learn abt them
    vectors::explain();

    //Vectors have a length and a capacity
    //When you push to a vector and it goes above capacity, 
    //it creates new alloc in the heap, moves data there, then deallocs the original memory

    #[allow(unused_mut)]
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &vec[2];
    //vec.push(4);  //Not ok: num is still in use
    println!("Third element is {}", *num);  //num no longer in use
    vec.push(4);  //This is ok: num isn't used anymore

    //POINTER SAFETY PRINCIPLE: Data should never be aliased and mutated at the same time.

    //With boxes, this is easy - you can't alias with a box
    //With references and borrowing, the BORROW CHECKER is used to ensure memory safety (check slides)

    explain_mutable_references();   
}


//also known as unique references
fn explain_mutable_references() {
    //before we used immutable (or shared) references
    //sometimes we want to change data without moving it
    let mut ids: Vec<u64> = vec![91488919, 48199402, 0, 1881002, 777];
    let num: &mut u64 = &mut ids[2];  //use &mut T to create a mutable reference of type T

    //Now that we have a mutable reference, all the vector's Read Write Own perms are gone
    //println!("Found invalid id: {}", ids[2]);

    //In exchange, now the deref'd path can be written to
    *num = rand::thread_rng().gen_range(1..=u64::MAX);
    println!("Changed third id to be valid: {}", ids[2]);

    //Basically, mutable references prevent aliasing by removing RWO perms from original reference
    //mutable references can be turned immutable

    let god_ref = &mut ids[4];

    //Hey, it's mutable!
    *god_ref += 1;
    *god_ref -= 1;

    //Not anymore!
    let immutable_god_ref = &*god_ref;

    //How DARE you try to desecrate our Lord! (cannot be assigned because it is BORROWED)
    //*god_ref = 666;

    //So we can alias! (vector doesn't get its RWO perms back until god_ref is gone though)
    println!("Here, I made a number to represent God: {}", *god_ref);
    //println!("Did I mention God is {}?", ids[4]);
    println!("Thank our Lord and Savior {immutable_god_ref} for Rust!");
    
    //Finally, perms!
    ids[0] = 1;

    explain_lifetimes();
}


//Permissions are returned to the original path to a value in the heap when all other paths (references) die.
fn explain_lifetimes() {
    let mut original: Box<[i32; 3]> = Box::new([1; 3]);

    //Hey, implicit dereference!
    let ref_1 = &mut original[/*10*/1];  //ref_1 born (WAIT ISN'T THIS OUT OF BOUNDS?)

    *ref_1 = 5;
    let ref_2 = &*ref_1;  //ref_2 born, ref_1 dies

    assert_eq!(*ref_2, 5);      //ref_2 dies

    //original gains back perms
    (*original)[1] = 1;

    //With control flow, a lifetime doesn't have to be defined for a full block of code
    let mut pos_or_neg: Box<[i8; 10]> = Box::new([1, -2, 3, 4, -5, -6, 7, 8, 9, -10]);
    
    //Another implicit dereference!
    let rng_i = rand::thread_rng().gen_range(0..pos_or_neg.len());

    //And another implcit dereference! Should be (*pos_or_neg)[rng_i]
    let rng_value = pos_or_neg[rng_i];


    //pos_or_neg only ever loses RWO perms in the if statement of this block
    if rng_value.is_positive() {
        let conditional_ref = &mut pos_or_neg[rng_i];
        *conditional_ref = rng_value * -1;

        println!("Changed position {rng_i} to be negative (now {conditional_ref})");
    } else {
        println!("Position {rng_i} ({rng_value}) is already negative");
    }    

    explain_data_outliving_references();
}


//Keep in mind - being set to nil is still a mutation.
fn explain_data_outliving_references() {
    //When Rust knows how long a reference lives, it just uses the Own permission
    let oh_no = String::from("I'm in danger.");
    let danger = &oh_no;

    //The borrow removes oh_no's O permission, but drop requires that, so the program doesn't compile
    //drop(oh_no);
    println!("{}", *danger);

    //If Rust doesn't know a reference's lifetime (like when it the input or output from a function), uses Flow
    let why = &mut vec![9, 10];
    
    //let w = flow(why, &50);

    why.push(11);

    //This is unsafe, but ONLY if flow doesn't return &50
    //Since it's uncertain, this won't compile
    //println!("{w}");
}


//This doesn't compile because if Rust only looks at the function signature, 
//it doesn't know if the outputted &i32 is a reference to im or confused

// fn flow(im: &Vec<i32>, confused: &i32) -> &i32 {
//     //neither of the args have the F permission, so they aren't returned
//     let cond = true;

//     if cond {
//         &(**im)[0];
//     } else {
//         confused;
//     }
// }

//To specify what can or can't be returned, Rust uses lifetimes
//For now, just know references passed as input/output to a function use Flow to verify safety