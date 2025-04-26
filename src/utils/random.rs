use rand::Rng;

pub fn generate_unique_id() -> String {
    let mut rng = rand::rng();
    let id = rng.random::<u64>();
    format!("{:016x}", id)
}
