
extern crate rand;

use rand::Rng;


/// Various small utility functions

#[allow(dead_code)]
/// Return a random element from the vector.
pub fn choose_random<T>(vec: &Vec<T>) -> &T {
    let mut rng = rand::thread_rng();

    let length = vec.len();
    let idx = rng.gen::<usize>() % length as usize;

    &vec[idx]
}

#[allow(dead_code)]
/// Return a random mutable element from the vector.
pub fn choose_random_mut<T>(vec: &mut Vec<T>) -> &mut T {
    let mut rng = rand::thread_rng();

    let length = vec.len();
    let idx = rng.gen::<usize>() % length as usize;

    &mut vec[idx]
}
