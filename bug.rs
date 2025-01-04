fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let slice = &vec[..];

    // This is incorrect and will cause a runtime error if vec is modified.
    // The slice borrows the underlying vector, but the vector may be reallocated if it grows beyond its capacity.
    for i in 0..vec.len() {
        println!("Value at index {}: {}", i, slice[i]);
    }
    vec.push(3); // This line might cause a runtime error
}