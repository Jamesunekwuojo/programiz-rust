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
    /*The major aim of the ownership is to manage Heap.
    Stack and Heap are both data structures in Rust, Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. .

    The Stack Data sctrucures operate LIFO ---> that's Last I First Out. Think of it as 
    arranging plates on stacking plates on each other.
    Adding data on stack is know as  --> pushing
    Removing data from the stack is known as  -- Pulling

    The Heap is less organise .. here is how it works: when u store data on the heap, you request for a 
    space on the heap. The memory alocator will:  
    * look for a space that is enough on the heap
    * Mark it as being used and the returns a pointer (that is the address of the location)
    This process is called allocating on the heap */

    // Ownership Rules:
    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.






}
