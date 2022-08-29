use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::NonNull;

/// why we need **ptr** in struct vector is a *raw vector* - NonNull ?

#[allow(dead_code)]
pub struct MyVec<T> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
}

#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ), * ) => {
        {
            let mut temp_vec = MyVec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };

    ( $x:expr ; $y:expr ) => {
        {
            let mut temp_vec = MyVec::new();

            if $y <= 0 {
                panic!("Size of vector must be greater than zero");
            };

            for _i in 0..$y {
                temp_vec.push($x);
            }

            temp_vec
        }
    };
}

// TODO: need to check again this drop trait to understand
impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            unsafe {
                std::ptr::drop_in_place(std::slice::from_raw_parts_mut(
                    self.ptr.as_ptr(),
                    self.len,
                )); // <- TODO: read the Rustonomicon implementation for understanding

                let size = std::mem::size_of::<T>() * self.capacity;
                let align = std::mem::align_of::<T>();

                let layout = Layout::from_size_align(size, align).expect("Cannot get layout of T");
                dealloc(self.ptr.as_ptr() as *mut u8, layout)
            }
        }
    }
}

#[allow(dead_code)]
impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            capacity: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.capacity == 0 {
            // init a block of data with size: 4 * size_of::<T>
            let new_space_in_heap = Layout::array::<T>(4).expect("Cannot create layout");

            // allocate a layout of data onto heap
            let ptr: *mut T = unsafe { alloc(new_space_in_heap) as *mut T };

            // init new NonNull raw pointer with the pointer to new space data in heap
            // store to myvec.ptr
            let ptr = NonNull::new(ptr).expect("NonNull::new error");

            // now we have enough space for store item, let's store it
            // we convert myvec ptr back to raw pointer and write item to the heap
            // note: why we don't do this: `*self.ptr.as_ptr() = item` <- understood

            unsafe {
                ptr.as_ptr().write(item);
            }

            self.ptr = ptr;

            // update capacity of myvec to 4
            self.capacity = 4;

            // update len of myvec to 1
            self.len = 1;
        } else if self.len < self.capacity {
            unsafe { self.ptr.as_ptr().add(self.len).write(item) }

            // increase len by 1
            self.len += 1;
        } else if self.len == self.capacity {
            // get block of memory from size (all vector) and alignment of T
            let size = std::mem::size_of::<T>() * self.capacity;
            let align = std::mem::align_of::<T>();

            let layout = Layout::from_size_align(size, align).expect("Cannot get layout of T");

            // new size = current capacity * 2
            let new_capacity = self
                .capacity
                .checked_mul(2)
                .expect("Muliple overflow capacity");

            let ptr = unsafe {
                let realloc_ptr: *mut T = realloc(
                    self.ptr.as_ptr() as *mut u8,
                    layout,
                    new_capacity * std::mem::size_of::<T>(),
                ) as *mut T;

                let ptr = NonNull::new(realloc_ptr).expect("NonNull::new error");

                // Store new item to last location in reallocated heap
                ptr.as_ptr().add(self.len).write(item);

                ptr
            };

            self.ptr = ptr;

            // increase len by 1
            self.len += 1;

            // assign new capacity
            self.capacity = new_capacity;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { self.ptr.as_ptr().add(self.len).read() })
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { &*self.ptr.as_ptr().add(index) })
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
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
        let mut my_vec_sample = MyVec::<String>::new();

        my_vec_sample.push(String::from("string_0"));
        my_vec_sample.push(String::from("string_1"));
        my_vec_sample.push(String::from("string_2"));
        my_vec_sample.push(String::from("string_3"));
        my_vec_sample.push(String::from("string_4"));
        my_vec_sample.push(String::from("string_5"));
        my_vec_sample.push(String::from("string_6"));
        my_vec_sample.push(String::from("string_7"));
        my_vec_sample.push(String::from("string_8"));
        my_vec_sample.push(String::from("string_9"));

        let pop_data = my_vec_sample.pop();

        assert_eq!(my_vec_sample.capacity(), 16);
        assert_eq!(my_vec_sample.len(), 9);

        assert_eq!(pop_data, Some(String::from("string_9")));

        for n in 0..my_vec_sample.len() {
            assert_eq!(
                my_vec_sample.get(n),
                Some(&(String::from("string_") + &n.to_string()))
            )
        }

        assert_eq!(my_vec_sample.get(1000), None);
    }

    #[test]
    fn my_vec_macro() {
        let mut my_vec_sample = myvec![0, 1, 2, 3, 4];

        assert_eq!(my_vec_sample.capacity(), 8);
        assert_eq!(my_vec_sample.len(), 5);

        let pop_data = my_vec_sample.pop();

        assert_eq!(pop_data, Some(4));

        for n in 0..my_vec_sample.len() {
            assert_eq!(my_vec_sample.get(n), Some(&(n)))
        }

        assert_eq!(my_vec_sample.get(1000), None);
    }

    #[test]
    fn my_vec_macro_with_semicolon() {
        let my_vec_sample: MyVec<usize> = myvec![0; 10];
        assert_eq!(my_vec_sample.capacity(), 16);
        assert_eq!(my_vec_sample.len(), 10);

        for n in 0..my_vec_sample.len() {
            assert_eq!(my_vec_sample.get(n), Some(&(0)))
        }
    }

    #[test]
    fn vec_and_vec_macro() {
        let mut vec_macro = vec![0, 1, 2, 3, 4];
        let mut vec = Vec::new();

        for i in 0..6 {
            vec.push(i)
        }

        vec_macro.push(5);

        println!("Capacity of vec with macro: {}", vec_macro.capacity());
        println!("Capacity of vec push: {}", vec.capacity());
    }
}
