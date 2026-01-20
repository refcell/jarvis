use crate::security::KeychainManager;

#[tauri::command]
pub fn store_api_key(provider: String, api_key: String) -> Result<(), String> {
    KeychainManager::store_api_key(&provider, &api_key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_api_key(provider: String) -> Result<Option<String>, String> {
    KeychainManager::get_api_key(&provider).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_api_key(provider: String) -> Result<(), String> {
    KeychainManager::delete_api_key(&provider).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn has_api_key(provider: String) -> Result<bool, String> {
    KeychainManager::has_api_key(&provider).map_err(|e| e.to_string())
}
