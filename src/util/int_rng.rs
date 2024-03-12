use rand::Rng;

pub fn i32_random_number_generator(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
