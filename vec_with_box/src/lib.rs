#[allow(dead_code)]
struct MyVec<T> {
    // ptr: NonNull<T>,
    ptr: Option<Box<T>>,
    capacity: usize,
    len: usize,
}

#[allow(dead_code)]
impl<T> MyVec<T> {
    fn new() -> Self {
        Self {
            ptr: Option::None,
            capacity: 0,
            len: 0,
        }
    }

    fn push(&mut self, data: T) {
        if self.capacity == 0 {
            self.ptr = Some(Box::new(data));

            if self.ptr.is_none() {
                println!("Cannot alloc")
            }

            self.capacity = 4;
            self.len += 1;
        }
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_vec() {
        let mut my_vec = MyVec::<u32>::new();

        assert_eq!(my_vec.capacity(), 0);
        assert_eq!(my_vec.len(), 0);

        my_vec.push(1);

        assert_eq!(my_vec.capacity(), 4);
        assert_eq!(my_vec.len(), 1);
    }
}
