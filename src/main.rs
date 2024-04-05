use std::io;
use rand::Rng;

fn main() {
    let xs: [i32; 10];
    xs = generate_number_dist();

    println!("Array: {:?}", xs);
}




fn generate_number_dist() -> [i32; 10] {

    let mut rng = rand::thread_rng();  // Obtain a random number generator

    let mut xs: [i32; 10] = [0; 10];  // Initialize the array with default values (0)

    for num in &mut xs {
        *num = rng.gen_range(1..=100);  // Generate a random number between 1 and 100 (inclusive)
    }

    return xs  // Return the populated array

}

fn bogo_sort() {


}
fn bubble_sort() {

}

fn insertion_sort() {

}


fn in_order() {


    assert_eq!(vec![1.1, 1.123, 1.15, 2.0, 5.5], vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}