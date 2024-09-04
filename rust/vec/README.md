#### Definition:

- Array List (in Java) is equivalent to Vector (in Rust)

- Vector is a contiguous growable array type with heap-allocated contents 


#### Todo:

- [x] Create a new vector
  - [x] `Vec::new`
  - [x] macro `vec!`  

- [x] Push values onto the end of a vector (which will grow the vector as needed)

- [x] Pop values out of the vector 

- [x] Indexing, meaning we are able to get data from vector by its index


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
- [25.08.2022] Done push if length < capacity (raw pointer offset a len and write data)
- [26.08.2022] Done push if length == capacity, realloc, it's having the error `[Error] realloc(): invalid next size` in the 3rd time realloc, need to check that next day. also check again the drop trait
- [27.08.2022] Fix `[Error] realloc(): invalid next size` (realloc with wrong size!), learn *size* and *align*, finish `push` function, `Drop` trait, `get` by index, check memory leak (no leaks are possible), finish [Ryan Levick](https://youtu.be/3OL95gZgPWA) tutorial
- [28.08.2022] Done `pop` function, understand dereference assign raw pointer (will drop its current value and that mem must be valid -> the correct way is using ptr::write), check github profiles of rust team members, look through their blogs and be inspired by them. e.g. [Ralf Jung](https://research.ralfj.de/) with his thesis about Rust lang, or [Matklad](https://matklad.github.io/resume/) and his resume, ... there are a ton of people full of enthusiasm out there coding beautiful things
- [29.08.2022] Done marco for `myvec!`, note that the vector created by this macro will have the same capacity as the one created normally, that means I didn't create vector [with\_capacity()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity) even I already known the size of vector when I create it with the macro. I even ask a stupid question about this on stackoverflow ([Capacity of vectors created with macro](https://stackoverflow.com/questions/73531364/capacity-of-vectors-created-with-macro#73531450)); create more tests 
- [30.08.2022] Read [IntoIter](https://doc.rust-lang.org/nomicon/vec/vec-into-iter.html) and decided not implement it, we will finish the implmentation of vector in here. Will record the video to review, re-implement and discuss later. Also, I will take some days off (at most 1 week - by 07.09.2022) to resolve some critical tickets from work, take the holiday, build up the good habits again then come back to HCMC and continue on this. 

Thank to myself that I've already finished this very first data structure. Rust is hard, but I'm proud that I'm learning it. Half a month without missing any day, it is extremely hard for me to do. Thank me!

And thank you, to anyone read this line.


###### Need to cover:
- Record a video for re-implement and explain what I've learned

#### References:
- [Module std::vec](https://doc.rust-lang.org/std/vec/index.html)  
- [Struct std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Implement by Rust team - Source](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html)
- [Implement vec by Rustonomicon - Steps](https://doc.rust-lang.org/nomicon/vec/vec.html)
- [Crust of Rust: Lifetime Annotations](https://www.youtube.com/watch?v=rAl-9HwD858)
- [Let's Get Rusty](https://www.youtube.com/c/LetsGetRusty)
- [Implementing Rust's Vec From Scratch - Ryan Levick](https://youtu.be/3OL95gZgPWA)
