use vec::MyVec;

fn main() {
    let mut my_vec_sample = MyVec::<u32>::new();

    my_vec_sample.push(1);
    my_vec_sample.push(2);
    my_vec_sample.push(3);
    my_vec_sample.push(4);
    my_vec_sample.push(5);
    my_vec_sample.push(6);
    my_vec_sample.push(7);
    my_vec_sample.push(8);
    my_vec_sample.push(9);

    println!(
        "len: {}, cap: {}",
        my_vec_sample.len(),
        my_vec_sample.capacity()
    )
}
