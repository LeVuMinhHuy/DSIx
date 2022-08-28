use vec::MyVec;

#[derive(Debug, PartialEq)]
struct Dropped(String);

impl Drop for Dropped {
    fn drop(&mut self) {
        //println!("Dropping ! {}", self.0)
    }
}

fn main() {
    let mut my_vec_sample = MyVec::<Dropped>::new();

    my_vec_sample.push(Dropped(String::from("string_0")));
    my_vec_sample.push(Dropped(String::from("string_1")));
    my_vec_sample.push(Dropped(String::from("string_2")));
    my_vec_sample.push(Dropped(String::from("string_3")));
    my_vec_sample.push(Dropped(String::from("string_4")));
    my_vec_sample.push(Dropped(String::from("string_5")));
    my_vec_sample.push(Dropped(String::from("string_6")));
    my_vec_sample.push(Dropped(String::from("string_7")));
    my_vec_sample.push(Dropped(String::from("string_8")));
    my_vec_sample.push(Dropped(String::from("string_9")));

    my_vec_sample.pop();
    my_vec_sample.pop();

    //let pop_data = my_vec_sample.pop();

    //println!("Pop: {:?}", pop_data);

    //println!("Real {:?}", unsafe {
    //    my_vec_sample.as_ptr().add(my_vec_sample.len()).read()
    //});

    //println!("Get after pop: {:?}", my_vec_sample.get(9));
    //println!("Get after pop: {:?}", my_vec_sample.get(10));

    //println!(
    //    "\nlen: {}, cap: {}",
    //    my_vec_sample.len(),
    //    my_vec_sample.capacity()
    //);

    //println!("\nVector includes: ");
    //for n in 0..my_vec_sample.len() {
    //    println!("{:?}", my_vec_sample.get(n));
    //}

    //println!("\n== End of main ==");
}
