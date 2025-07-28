use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use std::sync::OnceLock;

// Cached Argon2 instance for better performance
static ARGON2: OnceLock<Argon2> = OnceLock::new();

fn get_argon2() -> &'static Argon2<'static> {
    ARGON2.get_or_init(|| Argon2::default())
}

pub async fn generate_hash_password(password: String) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = get_argon2();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            eprintln!("❌ Hash error: {:?}", e);
            e.to_string()
        })?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password_hash(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("❌ Hash parse error: {:?}", e);
            return false;
        }
    };
    get_argon2()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn verify_password_hash_bytes(password: &str, password_hash_bytes: &[u8]) -> bool {
    let password_hash_str = match std::str::from_utf8(password_hash_bytes) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Invalid UTF-8 in stored password hash: {:?}", e);
            return false;
        }
    };

    let parsed_hash = match PasswordHash::new(password_hash_str) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("❌ Hash parse error: {:?}", e);
            return false;
        }
    };

    get_argon2()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
