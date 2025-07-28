use rand::{Rng, distributions::Alphanumeric, thread_rng};

pub async fn new_random_string(size: usize) -> Result<String, ()> {
    let s: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();

    Ok(s)
}
