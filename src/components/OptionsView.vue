<template>
  <div class="content-view options-view">
    <div class="options-content">
      <div class="options-grid">
        <div class="option-group">
          <h3>Server Settings</h3>
          <div class="input-group">
            <label for="server-address">Server Address:</label>
            <input id="server-address" v-model="serverAddress" type="text" placeholder="127.0.0.1:2302"
              class="server-input" @input="saveSettings" />
          </div>
          <div class="input-group">
            <label for="server-password">Server Password (optional):</label>
            <input id="server-password" v-model="serverPassword" type="password"
              placeholder="Enter server password if required" class="server-input" @input="saveSettings" />
          </div>
          <button @click="queryServer" :disabled="isQuerying" class="query-button">
            {{ isQuerying ? 'Checking...' : 'Test Connection' }}
          </button>

          <!-- Server Test Results -->
          <div v-if="serverInfo" class="server-result">
            <div class="result-header">
              <div class="result-status" :class="{
                'status-online': serverStatus === 'online',
                'status-offline': serverStatus === 'offline',
                'status-checking': serverStatus === 'checking'
              }"></div>
              <span class="result-title">Connection Test Result</span>
            </div>
            <div class="result-content">
              {{ serverInfo }}
            </div>
          </div>
        </div>

        <div class="option-group">
          <h3>Game Settings</h3>

          <!-- Arma 3 Directory Selection -->
          <div class="input-group">
            <label for="arma3-directory">Arma 3 Installation Directory:</label>
            <div class="directory-input-container">
              <input id="arma3-directory" v-model="arma3Directory" type="text"
                placeholder="Select Arma 3 root directory..." class="server-input directory-input" readonly />
              <button @click="selectArma3Directory" class="browse-button">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path
                    d="M10 4H4c-1.11 0-2 .89-2 2v12c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2h-8l-2-2z" />
                </svg>
                Browse
              </button>
            </div>
            <div v-if="arma3Directory" class="directory-status">
              <div class="status-indicator valid"></div>
              <span class="status-text">Directory selected</span>
            </div>
          </div>

          <!-- Steam Library Directory Selection -->
          <div class="input-group">
            <label for="steam-library">Steam Library Directory:</label>
            <div class="directory-input-container">
              <input id="steam-library" v-model="steamLibraryPath" type="text"
                placeholder="Select directory for mods..." class="server-input directory-input" readonly />
              <button @click="selectSteamLibraryPath" class="browse-button">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path
                    d="M10 4H4c-1.11 0-2 .89-2 2v12c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2h-8l-2-2z" />
                </svg>
                Browse
              </button>
            </div>
            <div v-if="steamLibraryPath" class="directory-status">
              <div class="status-indicator valid"></div>
              <span class="status-text">Steam library directory selected</span>
            </div>
            <div class="help-text">
              Optional: Select your Steam library directory to scan for Workshop mods (e.g., C:\Program Files
              (x86)\Steam)
            </div>
          </div>

          <!-- Startup Parameters -->
          <div class="input-group">
            <label for="startup-params">Startup Parameters:</label>
            <input id="startup-params" v-model="startupParameters" type="text"
              placeholder="-noSplash -skipIntro -world=empty" class="server-input" @input="saveSettings" />
            <div class="help-text">
              Optional Arma 3 launch parameters (e.g., -noSplash, -skipIntro, -world=empty)
            </div>
          </div>

          <div class="setting-item">
            <label class="checkbox-label">
              <input type="checkbox" v-model="battleEyeEnabled" @change="saveSettings" /> Enable BattlEye Anti-Cheat
            </label>
            <div class="help-text">
              BattlEye provides anti-cheat protection. Disable only if you need to run without anti-cheat for
              development/testing.
            </div>
          </div>
        </div>

        <div class="option-group">
          <h3>Update Settings</h3>
          
          <div class="setting-item">
            <label class="toggle-label">
              <span class="toggle-text">Automatic Mod Updates</span>
              <div class="toggle-container">
                <input type="checkbox" v-model="autoUpdateEnabled" @change="toggleAutoUpdate" class="toggle-input" />
                <div class="toggle-slider">
                  <div class="toggle-slider-inner"></div>
                </div>
              </div>
            </label>
            <div class="help-text">
              When enabled, Forge framework mods will be automatically checked for updates when you visit the Mods section. Updates are applied automatically after a brief delay. This only affects custom Forge mods hosted on Google Cloud Storage, not Steam Workshop mods.
            </div>
          </div>
          
          <!-- App Updates Section -->
          <div class="setting-item">
            <h4>Launcher Updates</h4>
            <AppUpdater />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject } from 'vue';
import AppUpdater from './AppUpdater.vue';

// Inject shared state
const arma3Directory = inject<any>('arma3Directory');
const steamLibraryPath = inject<any>('steamLibraryPath');
const serverAddress = inject<any>('serverAddress');
const serverPassword = inject<any>('serverPassword');
const serverStatus = inject<any>('serverStatus');
const isQuerying = inject<any>('isQuerying');
const serverInfo = inject<any>('serverInfo');
const startupParameters = inject<any>('startupParameters');
const battleEyeEnabled = inject<any>('battleEyeEnabled');
const autoUpdateEnabled = inject<any>('autoUpdateEnabled');
const selectArma3Directory = inject<any>('selectArma3Directory');
const selectSteamLibraryPath = inject<any>('selectSteamLibraryPath');
const queryServer = inject<any>('queryServer');
const saveSettings = inject<any>('saveSettings');

// Toggle auto-update setting and save to settings.json
function toggleAutoUpdate() {
  saveSettings();
}
</script>

<style scoped>
.content-view {
  padding: 2rem !important;
  min-height: 100vh;
}

.options-content {
  max-width: 1200px;
  margin: 0 auto;
}

.options-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

.option-group {
  background: rgba(0, 0, 0, 0.25);
  backdrop-filter: blur(8px) saturate(150%);
  -webkit-backdrop-filter: blur(8px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px 0;
  padding: 1.5rem !important;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.option-group h3 {
  color: var(--accent-color);
  margin-bottom: 1rem !important;
  font-size: 1.2rem;
}

.input-group {
  margin-bottom: 1rem !important;
}

.input-group label {
  display: block;
  color: var(--text-secondary);
  margin-bottom: 0.5rem !important;
  font-size: 0.9rem;
}

.server-input {
  width: 100%;
  padding: 0.75rem !important;
  background: rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 0.9rem;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

.server-input:focus {
  outline: none;
  border-color: rgba(253, 230, 138, 0.4);
  background: rgba(0, 0, 0, 0.3);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3), 0 0 0 1px rgba(253, 230, 138, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.server-input::placeholder {
  color: rgba(246, 246, 246, 0.4);
}

.query-button {
  width: 100%;
  padding: 0.75rem !important;
  background: rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border: 1px solid rgba(253, 230, 138, 0.3);
  border-radius: 6px;
  color: var(--accent-color);
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.query-button:hover:not(:disabled) {
  background: rgba(253, 230, 138, 0.15);
  border-color: rgba(253, 230, 138, 0.5);
  color: var(--accent-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.query-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.server-result {
  margin-top: 1rem !important;
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(12px) saturate(150%);
  -webkit-backdrop-filter: blur(12px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  padding: 1rem !important;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.result-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.75rem !important;
}

.result-status {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.result-status.status-online {
  background-color: var(--accent-color);
  box-shadow: 0 0 8px rgba(253, 230, 138, 0.4);
}

.result-status.status-offline {
  background-color: #ef4444;
  box-shadow: 0 0 8px rgba(239, 68, 68, 0.4);
}

.result-status.status-checking {
  background-color: var(--accent-color);
  box-shadow: 0 0 8px rgba(253, 230, 138, 0.4);
  animation: pulse 1.5s infinite;
}

.result-title {
  color: var(--text-secondary);
  font-size: 0.9rem;
  font-weight: 500;
}

.result-content {
  color: var(--text-primary);
  font-size: 0.85rem;
  line-height: 1.4;
  background-color: rgba(0, 0, 0, 0.2);
  padding: 0.75rem !important;
  border-radius: 4px;
  border-left: 3px solid var(--accent-color);
  font-family: 'Courier New', monospace;
  white-space: pre-wrap;
  word-break: break-word;
}

.directory-input-container {
  display: flex;
  align-items: stretch;
}

.directory-input {
  flex: 1;
  border-top-right-radius: 0 !important;
  border-bottom-right-radius: 0 !important;
}

.browse-button {
  padding: 0.75rem 1rem !important;
  background: rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border: 1px solid rgba(253, 230, 138, 0.3);
  border-left: none;
  border-radius: 0 6px 6px 0;
  color: var(--accent-color);
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.9rem;
  white-space: nowrap;
  min-width: fit-content;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.browse-button:hover {
  background: rgba(253, 230, 138, 0.15);
  border-color: rgba(253, 230, 138, 0.5);
  color: var(--accent-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.browse-button svg {
  width: 16px;
  height: 16px;
}

.directory-status {
  margin-top: 0.5rem !important;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  transition: all 0.3s ease;
}

.status-indicator.valid {
  background-color: var(--accent-color);
  box-shadow: 0 0 6px rgba(253, 230, 138, 0.4);
}

.status-text {
  color: var(--text-secondary);
  font-size: 0.85rem;
}

.setting-item {
  margin-bottom: 0.75rem !important;
}

.checkbox-label {
  display: flex !important;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-secondary);
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  accent-color: var(--accent-color);
}

.help-text {
  margin-top: 0.5rem !important;
  color: var(--text-secondary);
  font-size: 0.8rem;
  opacity: 0.8;
  line-height: 1.3;
}

/* Toggle Switch Styling */
.toggle-label {
  display: flex !important;
  align-items: center;
  justify-content: space-between;
  color: var(--text-secondary);
  cursor: pointer;
  user-select: none;
  font-size: 0.9rem;
}

.toggle-text {
  font-weight: 500;
}

.toggle-container {
  position: relative;
  display: inline-block;
}

.toggle-input {
  display: none;
}

.toggle-slider {
  position: relative;
  width: 48px;
  height: 24px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 24px;
  transition: all 0.3s ease;
  cursor: pointer;
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.05);
}

.toggle-slider-inner {
  position: absolute;
  content: '';
  height: 18px;
  width: 18px;
  border-radius: 50%;
  background: var(--text-secondary);
  top: 2px;
  left: 3px;
  transition: all 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.toggle-input:checked + .toggle-slider {
  background: var(--accent-color);
  border-color: rgba(253, 230, 138, 0.4);
  box-shadow: 0 2px 12px rgba(253, 230, 138, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
}

.toggle-input:checked + .toggle-slider .toggle-slider-inner {
  transform: translateX(24px);
  background: #0f0f0f;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
}

.toggle-slider:hover {
  background: rgba(255, 255, 255, 0.15);
  border-color: rgba(255, 255, 255, 0.2);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
}

.toggle-input:checked + .toggle-slider:hover {
  background: rgba(253, 230, 138, 0.9);
  border-color: rgba(253, 230, 138, 0.6);
  box-shadow: 0 4px 16px rgba(253, 230, 138, 0.4), inset 0 1px 0 rgba(255, 255, 255, 0.15);
}

@keyframes pulse {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.5;
  }
}
</style>
