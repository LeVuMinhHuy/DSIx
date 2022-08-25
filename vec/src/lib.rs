use std::alloc::{alloc, Layout};
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

    unsafe fn push(&mut self, item: T) {
        if self.capacity == 0 {
            // init a block of data with size: 4 * size_of::<T>
            let new_space_in_heap = Layout::array::<T>(4).expect("Cannot create layout");

            // allocate a layout of data onto heap
            let ptr: *mut T = alloc(new_space_in_heap) as *mut T;

            // init new NonNull raw pointer with the pointer to new space data in heap
            // store to myvec.ptr
            self.ptr = NonNull::new(ptr).expect("NonNull::new error");

            // now we have enough space for store item, let's store it
            // we convert myvec ptr back to raw pointer and write item to the heap
            // note: why we don't do this: `*self.ptr.as_ptr() = item` ?
            self.ptr.as_ptr().write(item);

            // update capacity of myvec to 4
            self.capacity = 4;

            // update len of myvec to 1
            self.len = 1
        } else if self.len < self.capacity {
            self.ptr.as_ptr().add(self.len).write(item)
        } else {
            // TODO: realloc
            todo!()
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
