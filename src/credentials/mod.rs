// Encrypted credential management
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub service: String,
    pub email: String,
    pub password_hash: String, // Never store plaintext
    pub oauth_token: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct CredentialManager {
    credentials: Vec<Credential>,
    _master_password: String,
}

impl CredentialManager {
    pub fn new(master_password: &str) -> Self {
        Self {
            credentials: Vec::new(),
            _master_password: master_password.to_string(),
        }
    }

    pub fn add_credential(
        &mut self,
        service: &str,
        email: &str,
        password: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Hash password with Argon2
        let salt = SaltString::generate(rand::thread_rng());
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                Box::new(std::io::Error::other(format!("Hash error: {}", e)))
                    as Box<dyn std::error::Error>
            })?;
        let password_hash = password_hash.to_string();

        let id = Uuid::new_v4().to_string();
        let credential = Credential {
            id: id.clone(),
            service: service.to_string(),
            email: email.to_string(),
            password_hash,
            oauth_token: None,
            created_at: chrono::Utc::now(),
        };

        self.credentials.push(credential);
        Ok(id)
    }

    pub fn verify_password(
        &self,
        credential_id: &str,
        password: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let cred = self
            .credentials
            .iter()
            .find(|c| c.id == credential_id)
            .ok_or("Credential not found")?;

        let parsed_hash = PasswordHash::new(&cred.password_hash).map_err(|e| {
            Box::new(std::io::Error::other(format!("Parse error: {}", e)))
                as Box<dyn std::error::Error>
        })?;
        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn list_credentials(&self) -> Vec<Credential> {
        self.credentials.clone()
    }

    pub fn get_credential(&self, id: &str) -> Option<&Credential> {
        self.credentials.iter().find(|c| c.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_storage() {
        let mut manager = CredentialManager::new("master_pass");
        let id = manager
            .add_credential("gmail", "user@gmail.com", "secure_password")
            .unwrap();

        assert!(manager.verify_password(&id, "secure_password").unwrap());
        assert!(!manager.verify_password(&id, "wrong_password").unwrap());
    }
}
