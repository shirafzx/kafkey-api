use anyhow::Result;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use rand::Rng;

/// Service for generating and validating API keys
pub struct ApiKeyService;

impl ApiKeyService {
    /// Generate a new API key with format: pk_live_<random> or pk_test_<random>
    /// Returns (plain_key, key_hash, key_prefix)
    pub fn generate_key(environment: &str) -> Result<(String, String, String)> {
        // Determine prefix based on environment
        let prefix = match environment {
            "production" | "live" => "pk_live",
            "test" | "development" => "pk_test",
            _ => "pk_dev",
        };

        // Generate 32 random bytes and encode as hex (64 characters)
        let mut rng = rand::thread_rng();
        let mut random_bytes = [0u8; 32];
        rng.fill(&mut random_bytes);
        let random_hex = hex::encode(random_bytes);

        // Construct the full key
        let plain_key = format!("{}_{}", prefix, random_hex);

        // Hash the key using Argon2
        let key_hash = Self::hash_key(&plain_key)?;

        // Extract prefix for storage (first 10 chars for display)
        let key_prefix = format!(
            "{}...{}",
            &plain_key[..10],
            &plain_key[plain_key.len() - 4..]
        );

        Ok((plain_key, key_hash, key_prefix))
    }

    /// Hash an API key using Argon2
    pub fn hash_key(key: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(key.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash API key: {}", e))?
            .to_string();

        Ok(hash)
    }

    /// Verify an API key against its hash
    pub fn verify_key(key: &str, hash: &str) -> Result<bool> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("Invalid hash format: {}", e))?;

        let argon2 = Argon2::default();

        match argon2.verify_password(key.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Validate API key format
    pub fn validate_format(key: &str) -> bool {
        // Check if key starts with valid prefix
        let valid_prefixes = ["pk_live_", "pk_test_", "pk_dev_"];
        let has_valid_prefix = valid_prefixes.iter().any(|prefix| key.starts_with(prefix));

        if !has_valid_prefix {
            return false;
        }

        // Check length (prefix + underscore + 64 hex chars = 72-74 chars total)
        if key.len() < 70 || key.len() > 75 {
            return false;
        }

        // Check that the part after prefix is valid hex
        let parts: Vec<&str> = key.splitn(3, '_').collect();
        if parts.len() != 3 {
            return false;
        }

        let hex_part = parts[2];
        hex_part.chars().all(|c| c.is_ascii_hexdigit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key_production() {
        let (plain_key, hash, prefix) = ApiKeyService::generate_key("production").unwrap();

        assert!(plain_key.starts_with("pk_live_"));
        assert!(prefix.starts_with("pk_live_"));
        assert!(prefix.contains("..."));
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_generate_key_test() {
        let (plain_key, _, _) = ApiKeyService::generate_key("test").unwrap();
        assert!(plain_key.starts_with("pk_test_"));
    }

    #[test]
    fn test_hash_and_verify() {
        let key = "pk_live_1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let hash = ApiKeyService::hash_key(key).unwrap();

        assert!(ApiKeyService::verify_key(key, &hash).unwrap());
        assert!(!ApiKeyService::verify_key("wrong_key", &hash).unwrap());
    }

    #[test]
    fn test_validate_format() {
        assert!(ApiKeyService::validate_format(
            "pk_live_1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        ));
        assert!(ApiKeyService::validate_format(
            "pk_test_1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        ));

        assert!(!ApiKeyService::validate_format("invalid_key"));
        assert!(!ApiKeyService::validate_format("pk_live_short"));
        assert!(!ApiKeyService::validate_format(
            "pk_dev_1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        ));
    }
}
