fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Safe approach 1: Iterate over a copy of the vector
    for i in vec.clone().iter(){
        println!("Value at index {}: {}", i, *i);
    }
    vec.push(3);

    // Safe approach 2: Create a new vector from the slice
    let new_vec:Vec<i32> = vec.iter().cloned().collect();
    for i in new_vec.iter() {
        println!("Value from new vector: {}", i);
    }
    
    // Safe approach 3: Use iterators
    for (index, value) in vec.iter().enumerate() {
        println!("Value at index {}: {}", index, value);
    }
    vec.push(4); 
} 