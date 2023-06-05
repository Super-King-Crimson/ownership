pub fn explain() {
    println!("Vectors!");

    //Vectors have variable length, because their data is stored in the heap
    let mut weekdays: Vec<char> = vec!['日', '月', '火', '水', '木'];

    //the vec! macro creates a vector with type Vec<T>
    //All elements must have same type
    //Vec::push (or vec.push) adds element to vector
    weekdays.push('金');  //by the way this is an implict reference: push expects &mut Vec
    Vec::push(&mut weekdays, '土');


    //So you make arrays with [] and vectors with vec![]
    let arr: [char; 5] = ['月', '火', '水', '木', '金'];
    println!("The array has length {}", arr.len());

    let vec: Vec<char> = vec!['M', 'T', 'W', 'H', 'F'];
    println!("The vector has length {}", vec.len());
}