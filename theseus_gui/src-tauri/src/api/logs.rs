use crate::api::Result;
use theseus::{
    logs::{self, Logs},
    prelude::ProfilePathId,
};

/*
A log is a struct containing the datetime string, stdout, and stderr, as follows:

pub struct Logs {
    pub datetime_string:  String,
    pub stdout: String,
    pub stderr: String,
}
*/

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("logs")
        .invoke_handler(tauri::generate_handler![
            logs_get_logs,
            logs_get_logs_by_datetime,
            logs_get_output_by_datetime,
            logs_delete_logs,
            logs_delete_logs_by_datetime,
        ])
        .build()
}

/// Get all Logs for a profile, sorted by datetime
#[tauri::command]
pub async fn logs_get_logs(
    profile_path: ProfilePathId,
    clear_contents: Option<bool>,
) -> Result<Vec<Logs>> {
    let val = logs::get_logs(profile_path, clear_contents).await?;

    Ok(val)
}

/// Get a Log struct for a profile by profile id and datetime string
#[tauri::command]
pub async fn logs_get_logs_by_datetime(
    profile_path: ProfilePathId,
    datetime_string: String,
) -> Result<Logs> {
    Ok(logs::get_logs_by_datetime(profile_path, datetime_string).await?)
}

/// Get the stdout for a profile by profile id and datetime string
#[tauri::command]
pub async fn logs_get_output_by_datetime(
    profile_path: ProfilePathId,
    datetime_string: String,
) -> Result<String> {
    let profile_path = if let Some(p) =
        crate::profile::get(&profile_path, None).await?
    {
        p.profile_id()
    } else {
        return Err(theseus::Error::from(
            theseus::ErrorKind::UnmanagedProfileError(profile_path.to_string()),
        )
        .into());
    };

    Ok(logs::get_output_by_datetime(&profile_path, &datetime_string).await?)
}

/// Delete all logs for a profile by profile id
#[tauri::command]
pub async fn logs_delete_logs(profile_path: ProfilePathId) -> Result<()> {
    Ok(logs::delete_logs(profile_path).await?)
}

/// Delete a log for a profile by profile id and datetime string
#[tauri::command]
pub async fn logs_delete_logs_by_datetime(
    profile_path: ProfilePathId,
    datetime_string: String,
) -> Result<()> {
    Ok(logs::delete_logs_by_datetime(profile_path, &datetime_string).await?)
}
