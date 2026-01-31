use anyhow::Result;
use totp_rs::{Algorithm, TOTP};

pub struct TotpService;

impl TotpService {
    /// Generate a new TOTP secret (Base32 encoded)
    pub fn generate_secret() -> String {
        use rand::RngCore;
        let mut bytes = [0u8; 20];
        rand::thread_rng().fill_bytes(&mut bytes);
        base32::encode(base32::Alphabet::RFC4648 { padding: false }, &bytes)
    }

    /// Generate a TOTP URL for QR code generation
    pub fn generate_qr_code_url(secret: &str, email: &str, issuer: &str) -> Result<String> {
        let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret)
            .ok_or_else(|| anyhow::anyhow!("Invalid base32 secret"))?;

        let totp = TOTP::new(
            Algorithm::SHA256,
            6,
            1,
            30,
            secret_bytes,
            Some(issuer.to_string()),
            email.to_string(),
        )
        .map_err(|e| anyhow::anyhow!("Failed to create TOTP: {}", e))?;

        Ok(totp.get_url())
    }

    /// Verify a TOTP code
    pub fn verify_code(secret: &str, code: &str) -> bool {
        let secret_bytes =
            match base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret) {
                Some(b) => b,
                None => return false,
            };

        let totp = match TOTP::new(
            Algorithm::SHA256,
            6,
            1,
            30,
            secret_bytes,
            None,
            "".to_string(),
        ) {
            Ok(t) => t,
            Err(_) => return false,
        };

        totp.check_current(code).unwrap_or(false)
    }
}
