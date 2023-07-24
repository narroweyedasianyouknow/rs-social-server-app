use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
      let cost = 10;
      let hashed = hash(password, cost);
      return hashed;
}
pub fn verify_password(password: &str, hashed_password: &str) -> bool {
      // Проверяем, соответствует ли переданный пароль хешированному паролю
      match verify(password, hashed_password) {
            Ok(result) => result,
            Err(_) => false,
      }
}
