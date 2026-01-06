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

    /*To illustrate this concept of ownership a data type apart from the normal string data
    type discussed earlier, will be looked at and that i the 'String' type .

    So, what’s the difference here? Why can String be mutated but literals cannot? The difference is in how these two types deal with memory.

    n the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

    The memory must be requested from the memory allocator at runtime.
    We need a way of returning this memory to the allocator when we’re done with our String.

That first part is done by us: When we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

However, the second part is different. In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.

Rust takes a different path: The memory is automatically returned once the variable that owns it goes out of scope. Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal:

  {
    let mut s = String::from("Hello"); // it's valid from this point forward
    // do stuff with s
  }
  // it's out of scope

There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.

This pattern has a profound impact on the way Rust code is written. It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the data we’ve allocated on the heap. Let’s explore some of those situations now.*/






}
