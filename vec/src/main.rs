use vec::MyVec;

#[derive(Debug, PartialEq)]
struct Dropped(usize);

impl Drop for Dropped {
    fn drop(&mut self) {
        println!("Dropping ! {}", self.0)
    }
}

fn main() {
    let mut my_vec_sample = MyVec::<Dropped>::new();

    my_vec_sample.push(Dropped(0));
    my_vec_sample.push(Dropped(1));
    my_vec_sample.push(Dropped(2));
    my_vec_sample.push(Dropped(3));
    my_vec_sample.push(Dropped(4));
    my_vec_sample.push(Dropped(5));
    my_vec_sample.push(Dropped(6));
    my_vec_sample.push(Dropped(7));
    my_vec_sample.push(Dropped(8));

    println!(
        "len: {}, cap: {}",
        my_vec_sample.len(),
        my_vec_sample.capacity()
    );

    for n in 0..my_vec_sample.len() {
        println!("{:?}", my_vec_sample.get(n));
    }

    println!("{:?}", my_vec_sample.get(1000));

    println!("== End of main ==");
}
