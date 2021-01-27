use argon2::{self, Config};

pub fn hash(password: &String, salt: &String) -> String {
    let config = Config::default();
    return argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
}

pub fn verify(password: &String, hash: &String) -> bool {
    return argon2::verify_encoded(&hash, password.as_bytes()).unwrap();
}
