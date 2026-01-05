fn main() {
    println!("Hello, world!");

    // some exercises on tuple..
    // Tuple is a  compound data type in Rust  It's used to store heterogenouse sequence of elements. 
    // That's value of differnt data types can be stored.
    let tuple1:(&str,i32,f64) = ("Hello",  23, 20.4);
    println!("Tuple is {:?}", tuple1);
    println!("First value for the tuple is {}", tuple1.0);
}
