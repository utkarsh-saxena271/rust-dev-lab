# Collections

Rust standard library has a number of data structures called collections, they can contain multiple values.
The data these collections points to are stored on the heap, i.e., they store dynamic data!

### Vectors
Vectors are data structures like arrays, but dynamic, their size can be increased/decreased. Just like the vectors in C++.

```rust
fn main() {
    // vectors
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    vec.push(40);
    println!("{:?}",vec)
}
```

### Hashmaps
  