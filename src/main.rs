use std::io;
use rand::Rng;

fn main() {
    let mut xs: [i32; 10];
    let mut sorted_xs;

    xs  = generate_number_dist();

    sorted_xs = xs.clone();

    sorted_xs.sort();


    if(xs==sorted_xs)
    {
        println!("arr1 and arr2 have similar elements");
    }
    else
    {
        println!("arr1 and arr2 does not have similar elements");
    }

    println!("Array: {:?}", xs);

    while xs!=sorted_xs {
        xs = bogo_sort(xs);
    }

    
    println!("Array: {:?}", xs);
    println!("Array: {:?}", sorted_xs);

}




fn generate_number_dist() -> [i32; 10] {

    let mut rng = rand::thread_rng();  // Obtain a random number generator

    let mut xs: [i32; 10] = [0; 10];  // Initialize the array with default values (0)

    for num in &mut xs {
        *num = rng.gen_range(1..=100);  // Generate a random number between 1 and 100 (inclusive)
    }

    return xs  // Return the populated array

}

fn bogo_sort(mut sortable: [i32; 10]) -> [i32; 10]{
    let mut rng = rand::thread_rng();
    let rand_1 = rng.gen_range(0..=9);
    let rand_2 = rng.gen_range(0..=9);
    let temp_1;
    let temp_2;

    temp_1 = sortable[rand_1];
    temp_2 = sortable[rand_2];

    sortable[rand_1] = temp_2;
    sortable[rand_2] = temp_1;

    return sortable;
}
fn bubble_sort() {

}

fn insertion_sort() {

}


fn in_order() {

}