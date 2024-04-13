use rand::distributions::{Distribution, Standard};
use rand::thread_rng;
use rand::Rng;

// Upload 1.
#[test]
fn constant() {
    let handle = cimetrics_rs::handle();
    handle.add("constant", 1);
}

// Upload a random integer between 0 and 2^8.
#[test]
fn random_one() {
    let handle = cimetrics_rs::handle();
    handle.add("random_one", rand::thread_rng().gen::<u8>() as u64);
}

// Upload a random integer between 0 and 2^16.
#[test]
fn random_two() {
    let handle = cimetrics_rs::handle();
    handle.add("random_two", rand::thread_rng().gen::<u16>() as u64);
}

// 1/4 of the time doesn't upload metrics.
#[test]
fn inconsistent() {
    let handle = cimetrics_rs::handle();
    if rand::thread_rng().gen::<u8>() > (u8::MAX / 4) {
        handle.add("inconsistent_constant", 2);
        handle.add("inconsistent_random", rand::thread_rng().gen::<u8>() as u64);
    }
}

#[test]
fn many() {
    let handle = cimetrics_rs::handle();
    let mut rng = rand::thread_rng();
    handle.add("random_u8_one", rng.gen::<u8>() as u64);
    handle.add("random_u8_two", rng.gen::<u8>() as u64);
    handle.add("random_u8_three", rng.gen::<u8>() as u64);
    handle.add("random_u16_one", rng.gen::<u16>() as u64);
    handle.add("random_u16_two", rng.gen::<u16>() as u64);
    handle.add("random_u16_three", rng.gen::<u16>() as u64);
    handle.add("random_u32_one", rng.gen::<u32>() as u64);
    handle.add("random_u32_two", rng.gen::<u32>() as u64);
    handle.add("random_u32_three", rng.gen::<u32>() as u64);

    handle.add("random_many_one", rng.gen::<u16>() as u64);
    handle.add("random_many_two", rng.gen::<u16>() as u64);
    handle.add("random_many_three", rng.gen::<u16>() as u64);
    handle.add("random_many_four", rng.gen::<u16>() as u64);
    handle.add("random_many_five", rng.gen::<u16>() as u64);
    handle.add("random_many_six", rng.gen::<u16>() as u64);
    handle.add("random_many_seven", rng.gen::<u16>() as u64);
}

#[test]
fn standard_distribution() {
    let handle = cimetrics_rs::handle();
    let mut rng = thread_rng();
    let mut iter = Standard
        .sample_iter(&mut rng)
        .map(|x: f32| (1_000_000f32 * x) as u64);
    handle.add("standard_one", iter.next().unwrap());
    handle.add("standard_two", iter.next().unwrap());
    handle.add("standard_three", iter.next().unwrap());
}
