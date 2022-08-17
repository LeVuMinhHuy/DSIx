#### Definition:

- Array List (in Java) is equivalent to Vector (in Rust)

- Vector is a contiguous growable array type with heap-allocated contents 


#### Todo:

- [ ] Create a new vector
  - [ ] `Vec::new`
  - [ ] macro `vec!`  

- [ ] Push values onto the end of a vector (which will grow the vector as needed)

- [ ] Pop values out of the vector 

- [ ] Indexing, meaning we are able to get data from vector by its index


#### Examples:

- Create: 
```rust
let v: Vec<i32> = Vec::new();
```

```rust
let v: Vec<i32> = vec![];

let v = vec![1, 2, 3, 4, 5];

let v = vec![0; 10]; // ten zeroes
```

- Push:
```rust
let mut v = vec![1, 2];

v.push(3);

// Result: [1, 2, 3]
```

- Pop:
```rust
let mut v = vec![1, 2];
let two = v.pop();

// Result: two = 2, v = [1]
```

- Indexing
```rust
let mut v = vec![1, 2, 3];
let three = v[2];
v[1] = v[1] + 5;

// Result: three = 3, v[1] = 6
```

#### Discuss:
1. There are still more actions we can do with Vector, but I won't implement them in here for the simplicity

2. Will discuss about heap more in here while I'm implementing


#### What I learned:


#### References:
- [Module std::vec](https://doc.rust-lang.org/std/vec/index.html)  
- [Struct std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
