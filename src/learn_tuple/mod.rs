use std::io;
pub(crate) fn tuple_check(){
    println!("Cargo tuple Basics");

    let a =[1,2,3,4,5];

    let mut index= String::new();

    io::stdin().read_line(&mut index).expect("failed to read number");

    let index: usize = index.trim().parse().expect("Entered number is not a index");

    let element = a[index];

    println!("The Value of the element at index {index} is:{element}");

}