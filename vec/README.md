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
1. There are still more actions we can do with Vector, but I won't implement them in here for the sake of simplicity

2. Will discuss about related problems more in section [#til](#what-i-learned) below through the implementation progress


#### What I learned:

###### Progress
- [18.08.2022] Read the implementation of Rustonomicon and try to understand lifetime from Crust of Rust series on youtube
- [19.08.2022] Read Rustonomicon about *Drop Check*, watch [Let's Get Rusty](https://www.youtube.com/c/LetsGetRusty) about *Lifetime*, *Smart pointer*, *Defer*, *Drop Trait*, watch a bit Crust to Rust.
- [20.08.2022] *Cargo workspace*, *Raw pointer*, *Rc*, *RefCell*, init struct vector, *NonNull*, *#[test]*, watch [Implementing Rust's Vec From Scratch - Ryan Levick](https://youtu.be/3OL95gZgPWA), fix structure to cargo workspace, implement `MyVec::new` with dangling pointer NonNull, write tests
- [21.08.2022] Write *push* function, using *NonNull<T>*, *alloc::alloc*, learn about *alloc::Layout*, test about using *Option<Box<T>>* instead of *NonNull<T>* and it worked - why ? maybe we need to implement more to understand
- [22.08.2022] Update docs
- [23.08.2022] Still watch [Ryan Levick](https://youtu.be/3OL95gZgPWA) about `push` function
- [24.08.2022] Rest day... too much tasks from company... i know, i know, i don't have to do all of that. will finish `push` function tomorrow. i'm interested in some leetcode too but i will keep implementing this repo every morning i promise


###### Need to cover:

- Next day:  
  - [ ] Finish `push` fn 

- Statuses:
  - Lifetime:  ![](https://progress-bar.dev/80)
  - Smart pointer:  ![](https://progress-bar.dev/60)
  - Drop check: ![](https://progress-bar.dev/01)
  - Raw pointer: ![](https://progress-bar.dev/60)


#### References:
- [Module std::vec](https://doc.rust-lang.org/std/vec/index.html)  
- [Struct std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Implement by Rust team - Source](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html)
- [Implement vec by Rustonomicon - Steps](https://doc.rust-lang.org/nomicon/vec/vec.html)
- [Crust of Rust: Lifetime Annotations](https://www.youtube.com/watch?v=rAl-9HwD858)
- [Let's Get Rusty](https://www.youtube.com/c/LetsGetRusty)
- [Implementing Rust's Vec From Scratch - Ryan Levick](https://youtu.be/3OL95gZgPWA)
