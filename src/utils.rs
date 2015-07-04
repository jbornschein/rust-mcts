
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

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use test::Bencher;

    use utils::*;

    #[test]
    fn test_choose_random() {
        let vec = vec![23];

        assert_eq!(*choose_random(&vec), 23);
    }

    #[bench]
    fn bench_choose_random10(b: &mut Bencher) {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        b.iter(|| choose_random(&vec))
    }
}
