use anyhow::Result;
use keyring::Entry;

const SERVICE_NAME: &str = "com.refcell.jarvis";

/// Keychain manager for secure API key storage
pub struct KeychainManager;

impl KeychainManager {
    /// Store an API key in the macOS Keychain
    pub fn store_api_key(provider: &str, api_key: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, &format!("{}_api_key", provider))?;
        entry.set_password(api_key)?;
        Ok(())
    }

    /// Retrieve an API key from the macOS Keychain
    pub fn get_api_key(provider: &str) -> Result<Option<String>> {
        let entry = Entry::new(SERVICE_NAME, &format!("{}_api_key", provider))?;
        match entry.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("Failed to get API key: {}", e)),
        }
    }

    /// Delete an API key from the macOS Keychain
    pub fn delete_api_key(provider: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, &format!("{}_api_key", provider))?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
            Err(e) => Err(anyhow::anyhow!("Failed to delete API key: {}", e)),
        }
    }

    /// Check if an API key exists for a provider
    pub fn has_api_key(provider: &str) -> Result<bool> {
        Ok(Self::get_api_key(provider)?.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires keychain access
    fn test_keychain_operations() {
        let provider = "test_provider";
        let api_key = "test_api_key_12345";

        // Store
        KeychainManager::store_api_key(provider, api_key).unwrap();

        // Retrieve
        let retrieved = KeychainManager::get_api_key(provider).unwrap();
        assert_eq!(retrieved, Some(api_key.to_string()));

        // Check exists
        assert!(KeychainManager::has_api_key(provider).unwrap());

        // Delete
        KeychainManager::delete_api_key(provider).unwrap();

        // Verify deleted
        assert!(!KeychainManager::has_api_key(provider).unwrap());
    }
}
