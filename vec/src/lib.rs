use std::ptr::NonNull;

/// why we need **ptr** in struct vector is a *raw vector* - NonNull ?

#[allow(dead_code)]
struct MyVec<T> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
}

#[allow(dead_code)]
impl<T> MyVec<T> {
    fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            capacity: 0,
            len: 0,
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
    fn std_vec() {
        let mut vec_sample = Vec::<u32>::new();
        vec_sample.push(1);
        vec_sample.push(2);
        vec_sample.push(3);
        vec_sample.push(4);
        vec_sample.push(5);

        //println!("{}", vec_sample.capacity());

        assert_eq!(vec_sample, [1, 2, 3, 4, 5]);
        assert_eq!(vec_sample.capacity(), 8);
        assert_eq!(vec_sample.len(), 5)
    }

    #[test]
    fn my_vec() {
        let my_vec_sample = MyVec::<u32>::new();

        assert_eq!(my_vec_sample.capacity(), 0);
        assert_eq!(my_vec_sample.len(), 0)
    }
}
