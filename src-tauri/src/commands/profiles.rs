use crate::profiles::profile::Profile;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn save_profile(state: State<'_, AppState>, profile: Profile) -> Result<(), String> {
    state
        .profile_repository
        .save_profile(profile)
        .await
        .map_err(|err| {
            let a = err.to_string();
            println!("{}", a);
            a
        })
}

#[tauri::command]
pub async fn delete_profile(state: State<'_, AppState>, name: String) -> Result<(), String> {
    state
        .profile_repository
        .delete_profile(&name)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn find_profile_by_name(
    state: State<'_, AppState>,
    name: String,
) -> Result<Option<Profile>, String> {
    state
        .profile_repository
        .find_profile_by_name(&name)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn profile_names(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    state
        .profile_repository
        .profile_names()
        .await
        .map_err(|err| err.to_string())
}
