mod ownership;
mod references_and_borrowing;
mod vectors;
mod ownership_case_study;
mod slice;

use std::io;

fn main() {
    //ownership::introduce();
    //references_and_borrowing::explain();
    // ownership_case_study::explain();
    //slice::explain();;
    //ownership::recap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expected to get input");
    println!("{input}");
}
