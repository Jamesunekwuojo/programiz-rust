fn main() {
    println!("Hello, world!");

    // some exercises on tuple..
    // Tuple is a  compound data type in Rust  It's used to store heterogenouse sequence of elements. 
    // That's value of differnt data types can be stored.
    // In Rust, we can create a tuple in two different ways:
    // Tuple with data type
    // Tuple without data type


    let tuple1:(&str,i32,f64) = ("Hello",  23, 20.4);
    println!("Tuple is {:?}", tuple1);
    println!("First value for the tuple is {}", tuple1.0);

    // Exploring ownership concept..
    The major aim of the ownership is to manage heap.
    Stack and Heap are both data structures in Rust.
    The Stack Data sctrucures operate LIFO ---> that's Last I First Out. Think of it as 
    arranging plates on stacking plates on each other.
    Adding data on stack is know as  --> pushing
    Removing data from the stack is known as  -- Pulling
    
}
