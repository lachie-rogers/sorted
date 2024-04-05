use std::io;
use std::mem::swap;
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
        xs = bubble_sort(xs);
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
fn bubble_sort(mut sortable: [i32; 10]) -> [i32; 10] {
    let length = 10;

    // Outer loop iterating from 1 to length (exclusive)
    for i in 1..length {
        let mut swapped = false;

        // Inner loop for bubble sort
        for j in 0..length - i {
            if sortable[j] > sortable[j + 1] {
                sortable.swap(j, j + 1);
                swapped = true;
            }
        }

        // Check if no elements were swapped in this pass
        if !swapped {
            break;
        }
    }

    return sortable;
}

fn insertion_sort() {

}
