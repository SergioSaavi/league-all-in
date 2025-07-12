// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use tauri::State;
use windows_record::{
    Recorder, 
    RecorderConfig, 
    RecorderConfigBuilder, 
    AudioSource,
    Result as RecorderResult
};

// Define recording state
struct RecordingState(Mutex<Option<Recorder>>);

#[derive(Serialize, Deserialize)]
struct RecordingSettings {
    output_path: String,
    fps: u32,
    record_audio: bool,
    process_name: Option<String>,
}

#[tauri::command]
fn start_recording(settings: RecordingSettings, state: State<RecordingState>) -> Result<String, String> {
    let mut recorder_state = state.0.lock().map_err(|_| "Failed to lock recording state".to_string())?;
    
    if recorder_state.is_some() {
        return Err("Recording already in progress".to_string());
    }
    
    // Build recorder configuration
    let mut config_builder = RecorderConfigBuilder::new();
    config_builder = config_builder
        .fps(settings.fps, 1)
        .output_path(&settings.output_path);
    
    if settings.record_audio {
        config_builder = config_builder
            .capture_audio(true)
            .audio_source(AudioSource::Desktop);
    }
    
    let config = config_builder.build();
    
    // Create and start recorder
    match Recorder::new(config) {
        Ok(mut recorder) => {
            // If process name is provided, set it
            if let Some(proc_name) = &settings.process_name {
                recorder = recorder.with_process_name(proc_name);
            }
            
            match recorder.start_recording() {
                Ok(_) => {
                    *recorder_state = Some(recorder);
                    Ok("Recording started successfully".to_string())
                },
                Err(e) => Err(format!("Failed to start recording: {}", e)),
            }
        },
        Err(e) => Err(format!("Failed to create recorder: {}", e)),
    }
}

#[tauri::command]
fn stop_recording(state: State<RecordingState>) -> Result<String, String> {
    let mut recorder_state = state.0.lock().map_err(|_| "Failed to lock recording state".to_string())?;
    
    if let Some(recorder) = recorder_state.take() {
        match recorder.stop_recording() {
            Ok(_) => Ok("Recording stopped successfully".to_string()),
            Err(e) => Err(format!("Failed to stop recording: {}", e)),
        }
    } else {
        Err("No recording in progress".to_string())
    }
}

#[tauri::command]
fn save_replay(output_path: &str, state: State<RecordingState>) -> Result<String, String> {
    let recorder_state = state.0.lock().map_err(|_| "Failed to lock recording state".to_string())?;
    
    if let Some(recorder) = recorder_state.as_ref() {
        match recorder.save_replay(output_path) {
            Ok(_) => Ok(format!("Replay saved to {}", output_path)),
            Err(e) => Err(format!("Failed to save replay: {}", e)),
        }
    } else {
        Err("No active recorder found".to_string())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(RecordingState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            greet,
            start_recording,
            stop_recording,
            save_replay,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
