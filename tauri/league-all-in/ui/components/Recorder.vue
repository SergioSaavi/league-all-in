<template>
  <div class="recorder-container">
    <h2>League All-In Recorder</h2>
    
    <div class="settings-panel">
      <div class="form-group">
        <label for="output-path">Output File Path:</label>
        <input
          id="output-path"
          v-model="settings.outputPath"
          type="text"
          placeholder="C:\Path\to\output.mp4"
        />
      </div>
      
      <div class="form-group">
        <label for="process-name">League of Legends Process (optional):</label>
        <input
          id="process-name"
          v-model="settings.processName"
          type="text"
          placeholder="League of Legends.exe"
        />
      </div>
      
      <div class="form-group">
        <label for="fps">Recording FPS:</label>
        <input
          id="fps"
          v-model.number="settings.fps"
          type="number"
          min="1"
          max="120"
        />
      </div>
      
      <div class="form-group checkbox">
        <input
          id="record-audio"
          v-model="settings.recordAudio"
          type="checkbox"
        />
        <label for="record-audio">Record Audio</label>
      </div>
    </div>
    
    <div class="controls">
      <button
        :class="['record-button', { recording: isRecording }]"
        @click="toggleRecording"
        :disabled="isProcessing"
      >
        {{ isRecording ? 'Stop Recording' : 'Start Recording' }}
      </button>
      
      <button 
        v-if="isRecording"
        class="save-replay-button"
        @click="saveReplay"
        :disabled="isProcessing"
      >
        Save Replay
      </button>
      
      <div v-if="statusMessage" :class="['status-message', statusType]">
        {{ statusMessage }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Recording settings
const settings = reactive({
  outputPath: 'C:\\Users\\Videos\\league-recording.mp4',
  fps: 60,
  recordAudio: true,
  processName: 'League of Legends (TM) Client'
})

// State
const isRecording = ref(false)
const isProcessing = ref(false)
const statusMessage = ref('')
const statusType = ref('info')

// Methods
async function toggleRecording() {
  try {
    isProcessing.value = true
    
    if (!isRecording.value) {
      // Start recording
      statusMessage.value = 'Starting recording...'
      statusType.value = 'info'
      
      const result = await invoke('start_recording', {
        settings: {
          output_path: settings.outputPath,
          fps: settings.fps,
          record_audio: settings.recordAudio,
          process_name: settings.processName || null
        }
      })
      
      statusMessage.value = result
      statusType.value = 'success'
      isRecording.value = true
    } else {
      // Stop recording
      statusMessage.value = 'Stopping recording...'
      statusType.value = 'info'
      
      const result = await invoke('stop_recording')
      
      statusMessage.value = result
      statusType.value = 'success'
      isRecording.value = false
    }
  } catch (error) {
    statusMessage.value = `Error: ${error}`
    statusType.value = 'error'
  } finally {
    isProcessing.value = false
  }
}

async function saveReplay() {
  try {
    isProcessing.value = true
    statusMessage.value = 'Saving replay...'
    statusType.value = 'info'
    
    const replayPath = `C:\\Users\\Videos\\league-replay-${new Date().getTime()}.mp4`
    const result = await invoke('save_replay', {
      outputPath: replayPath
    })
    
    statusMessage.value = result
    statusType.value = 'success'
  } catch (error) {
    statusMessage.value = `Error saving replay: ${error}`
    statusType.value = 'error'
  } finally {
    isProcessing.value = false
  }
}
</script>

<style scoped>
.recorder-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  background-color: #f5f5f5;
  border-radius: 8px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
}

h2 {
  text-align: center;
  margin-bottom: 2rem;
  color: #333;
}

.settings-panel {
  margin-bottom: 2rem;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
}

.checkbox {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.checkbox label {
  margin-bottom: 0;
}

input[type="text"],
input[type="number"] {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 1rem;
}

.controls {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.record-button {
  padding: 1rem 2rem;
  font-size: 1.2rem;
  font-weight: bold;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
  width: 100%;
  max-width: 300px;
}

.record-button:hover {
  background-color: #45a049;
}

.record-button.recording {
  background-color: #f44336;
}

.record-button.recording:hover {
  background-color: #d32f2f;
}

.save-replay-button {
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  background-color: #2196F3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
  width: 100%;
  max-width: 300px;
}

.save-replay-button:hover {
  background-color: #0b7dda;
}

.record-button:disabled,
.save-replay-button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.status-message {
  padding: 0.75rem;
  border-radius: 4px;
  text-align: center;
  width: 100%;
}

.status-message.info {
  background-color: #e3f2fd;
  color: #0d47a1;
}

.status-message.success {
  background-color: #e8f5e9;
  color: #1b5e20;
}

.status-message.error {
  background-color: #ffebee;
  color: #b71c1c;
}
</style>
