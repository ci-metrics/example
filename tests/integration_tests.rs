use rand::distributions::{Alphanumeric, Distribution, Standard, Uniform};
use rand::thread_rng;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

static METRICS: Metrics = Metrics::new();

struct Metrics(OnceLock<MetricsInner>);
struct MetricsInner(Mutex<(u64, HashMap<&'static str, u64>)>);
struct MetricsHandle<'a>(&'a MetricsInner);

impl Metrics {
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }
    fn handle(&self) -> MetricsHandle<'_> {
        let metrics = self
            .0
            .get_or_init(|| MetricsInner(Mutex::new((0, HashMap::new()))));
        let mut guard = metrics.0.lock().unwrap();
        let (count, _map) = &mut *guard;
        *count += 1;

        MetricsHandle(metrics)
    }
}
impl MetricsHandle<'_> {
    fn add(&self, name: &'static str, value: u64) {
        let mut guard = self.0 .0.lock().unwrap();
        let (_count, map) = &mut *guard;
        map.insert(name, value);
    }
}
impl Drop for MetricsHandle<'_> {
    fn drop(&mut self) {
        let mut guard = self.0 .0.lock().unwrap();
        let (count, map) = &mut *guard;
        *count -= 1;
        if *count == 0 {
            let file = std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open("metrics")
                .unwrap();
            serde_json::to_writer_pretty(file, map).unwrap();
        }
    }
}

// Upload 1.
#[test]
fn constant() {
    let handle = METRICS.handle();
    handle.add("constant", 1);
}

// Upload a random integer between 0 and 2^8.
#[test]
fn random_one() {
    let handle = METRICS.handle();
    handle.add("random_one", rand::thread_rng().gen::<u8>() as u64);
}

// Upload a random integer between 0 and 2^16.
#[test]
fn random_two() {
    let handle = METRICS.handle();
    handle.add("random_two", rand::thread_rng().gen::<u16>() as u64);
}

// 1/4 of the time doesn't upload metrics.
#[test]
fn inconsistent() {
    let handle = METRICS.handle();
    if rand::thread_rng().gen::<u8>() > (u8::MAX / 4) {
        handle.add("inconsistent_constant", 2);
        handle.add("inconsistent_random", rand::thread_rng().gen::<u8>() as u64);
    }
}

#[test]
fn many() {
    let handle = METRICS.handle();
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
    let handle = METRICS.handle();
    let mut rng = thread_rng();
    let mut iter = Standard
        .sample_iter(&mut rng)
        .map(|x: f32| (1_000_000f32 * x) as u64);
    handle.add("standard_one", iter.next().unwrap());
    handle.add("standard_two", iter.next().unwrap());
    handle.add("standard_three", iter.next().unwrap());
}
