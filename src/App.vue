<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide } from "vue";
import { invoke } from "@tauri-apps/api/core";
import PlayView from './components/PlayView.vue';
import ModsView from './components/ModsView.vue';
import FeedView from './components/FeedView.vue';
import OptionsView from './components/OptionsView.vue';
import ProgressBar from './components/ProgressBar.vue';
import packageInfo from '../package.json';

// TypeScript interfaces
interface ModInfo {
  name: string;
  path: string;
  enabled: boolean;
  mod_type: string;
  workshop_id?: string;
  size?: number;
  last_modified?: string;
  extensions: string[]; // List of extension DLL paths found in this mod
}

// Reactive state
const serverInfo = ref("");
const serverAddress = ref("127.0.0.1:2302");
const serverPassword = ref("");
const isQuerying = ref(false);
const serverStatus = ref("unknown"); // "online", "offline", "unknown", "checking"
const activeView = ref("play"); // "play", "mods", "feed", "options"
const arma3Directory = ref(""); // Arma 3 installation directory
const steamLibraryPath = ref(""); // Steam library directory for Workshop mods
const battleEyeEnabled = ref(true); // Enable/disable BattlEye
const autoUpdateEnabled = ref(false); // Enable/disable automatic mod updates
const isLaunching = ref(false);
const mods = ref<ModInfo[]>([]);
const startupParameters = ref("");
const isLoadingMods = ref(false);
let statusCheckInterval: number | null = null;

// Provide reactive state to child components
provide('arma3Directory', arma3Directory);
provide('steamLibraryPath', steamLibraryPath);
provide('serverAddress', serverAddress);
provide('serverPassword', serverPassword);
provide('serverStatus', serverStatus);
provide('isLaunching', isLaunching);
provide('mods', mods);
provide('startupParameters', startupParameters);
provide('battleEyeEnabled', battleEyeEnabled);
provide('autoUpdateEnabled', autoUpdateEnabled);
provide('isLoadingMods', isLoadingMods);

// Navigation function
function setActiveView(view: string) {
  activeView.value = view;
}

// Settings management
async function saveSettings() {
  try {
    await invoke("save_settings", {
      settings: {
        arma3_directory: arma3Directory.value,
        steam_library_path: steamLibraryPath.value,
        custom_mods_directory: "", // Add missing field - will be used later for custom mod directory
        server_address: serverAddress.value,
        server_password: serverPassword.value,
        battle_eye_enabled: battleEyeEnabled.value,
        startup_parameters: startupParameters.value,
        auto_update_enabled: autoUpdateEnabled.value,
      }
    });
    console.log("Settings saved successfully");
  } catch (error) {
    console.error("Error saving settings:", error);
  }
}

async function loadSettings() {
  try {
    const settings = await invoke<any>("load_settings");
    arma3Directory.value = settings.arma3_directory || "";
    steamLibraryPath.value = settings.steam_library_path || "";
    serverAddress.value = settings.server_address || "127.0.0.1:2302";
    serverPassword.value = settings.server_password || "";
    battleEyeEnabled.value = settings.battle_eye_enabled !== false; // Default to enabled
    startupParameters.value = settings.startup_parameters || "";
    autoUpdateEnabled.value = settings.auto_update_enabled === true; // Default to disabled
    console.log("Settings loaded successfully");
  } catch (error) {
    console.error("Error loading settings:", error);
  }
}

// Directory selection functions
async function selectArma3Directory() {
  try {
    const result = await invoke<string | null>("select_directory");
    if (result) {
      arma3Directory.value = result;
      await saveSettings(); // Auto-save when directory is selected
      await loadMods(); // Load mods after directory is selected
    }
  } catch (error) {
    console.error("Error selecting Arma 3 directory:", error);
  }
}

async function selectSteamLibraryPath() {
  try {
    const result = await invoke<string | null>("select_directory");
    if (result) {
      steamLibraryPath.value = result;
      await saveSettings(); // Auto-save when directory is selected
      await loadMods(); // Reload mods to include Workshop mods from new path
    }
  } catch (error) {
    console.error("Error selecting Steam library directory:", error);
  }
}

// Mod management functions
async function loadMods() {
  if (!arma3Directory.value) {
    mods.value = [];
    isLoadingMods.value = false;
    return;
  }

  try {
    isLoadingMods.value = true;
    console.log("Starting mod scan...");

    const loadedMods = await invoke<ModInfo[]>("scan_mods", {
      arma3Directory: arma3Directory.value,
      steamLibraryPath: steamLibraryPath.value || null
    });
    mods.value = loadedMods;
    console.log("Mods loaded successfully:", mods.value);

  } catch (error) {
    console.error("Error loading mods:", error);
    mods.value = [];
  } finally {
    isLoadingMods.value = false;
  }
}


function toggleMod(index: number) {
  mods.value[index].enabled = !mods.value[index].enabled;
}

function toggleAllMods(enable: boolean) {
  mods.value.forEach((mod: ModInfo) => {
    mod.enabled = enable;
  });
}

// Server query function
async function queryServer() {
  if (!serverAddress.value.trim()) {
    serverInfo.value = "Please enter a server address";
    serverStatus.value = "unknown";
    return;
  }

  try {
    isQuerying.value = true;
    serverStatus.value = "checking";
    serverInfo.value = "Querying server...";

    // Extract base address and increment port by one for query
    const addressParts = serverAddress.value.trim().split(":");
    const queryPort = parseInt(addressParts[1] || "2302") + 1;
    const queryAddress = `${addressParts[0]}:${queryPort}`;

    const result = await invoke<string>("query_server", { address: queryAddress });
    serverInfo.value = result;

    // Determine server status based on response
    if (result.includes("Server is Offline") || result.includes("Failed to connect") || result.includes("Error:")) {
      serverStatus.value = "offline";
    } else {
      serverStatus.value = "online";
    }
  } catch (error) {
    serverInfo.value = `Error: ${error}`;
    serverStatus.value = "offline";
  } finally {
    isQuerying.value = false;
  }
}

// Lifecycle hooks
onMounted(async () => {
  await loadSettings(); // Load settings first
  queryServer(); // Initial server query

  // Load mods in the background if directory is set
  if (arma3Directory.value) {
    loadMods(); // This is now non-blocking
  }

  // Set up periodic status checking
  statusCheckInterval = setInterval(() => {
    queryServer();
  }, 30000); // Check every 30 seconds
});

onUnmounted(() => {
  if (statusCheckInterval) {
    clearInterval(statusCheckInterval);
    statusCheckInterval = null;
  }
});

// Additional provide for functions that components need
provide('selectArma3Directory', selectArma3Directory);
provide('selectSteamLibraryPath', selectSteamLibraryPath);
provide('saveSettings', saveSettings);
provide('queryServer', queryServer);
provide('toggleMod', toggleMod);
provide('isQuerying', isQuerying);
provide('serverInfo', serverInfo);

// Notification state
const currentNotification = ref<any>(null);
const showNotification = ref(false);
let notificationTimeout: number | null = null;

function handleModNotification(notification: any) {
  currentNotification.value = notification;
  showNotification.value = true;
  
  // Clear existing timeout
  if (notificationTimeout) {
    clearTimeout(notificationTimeout);
  }
  
  // Auto-hide notification after 8 seconds for up-to-date, 15 seconds for updates
  const autoHideTime = notification.notification_type === 'up-to-date' ? 8000 : 15000;
  notificationTimeout = setTimeout(() => {
    showNotification.value = false;
    currentNotification.value = null;
  }, autoHideTime);
}

function dismissNotification() {
  showNotification.value = false;
  currentNotification.value = null;
  if (notificationTimeout) {
    clearTimeout(notificationTimeout);
    notificationTimeout = null;
  }
}

provide('handleModNotification', handleModNotification);
</script>

<template>
  <div class="app-layout">
    <!-- Vertical Navbar -->
    <nav class="navbar">
      <div class="nav-buttons">
        <button class="nav-button" :class="{ active: activeView === 'play' }" @click="setActiveView('play')"
          title="Play / Launch">
          <svg class="nav-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M8 5v14l11-7z" />
          </svg>
        </button>

        <button class="nav-button" :class="{ active: activeView === 'mods' }" @click="setActiveView('mods')"
          title="Mods">
          <svg class="nav-icon" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M20.5 11H19V7c0-1.1-.9-2-2-2h-4V3.5C13 2.12 11.88 1 10.5 1S8 2.12 8 3.5V5H4c-1.1 0-2 .9-2 2v4h1.5c1.38 0 2.5 1.12 2.5 2.5S4.88 16 3.5 16H2v4c0 1.1.9 2 2 2h4v-1.5c0-1.38 1.12-2.5 2.5-2.5s2.5 1.12 2.5 2.5V22h4c1.1 0 2-.9 2-2v-4h1.5c1.38 0 2.5-1.12 2.5-2.5S21.88 11 20.5 11z" />
          </svg>
        </button>

        <button class="nav-button" :class="{ active: activeView === 'feed' }" @click="setActiveView('feed')"
          title="Feed">
          <svg class="nav-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z" />
          </svg>
        </button>

        <button class="nav-button" :class="{ active: activeView === 'options' }" @click="setActiveView('options')"
          title="Options">
          <svg class="nav-icon" viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.8,11.69,4.8,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z" />
          </svg>
        </button>
      </div>
    </nav>

    <!-- Dark Panel -->
    <div class="dark-panel">
      <!-- View Title -->
      <div class="view-title">
        <h2 v-if="activeView === 'mods'" class="title">Mod Manager</h2>
        <h2 v-else-if="activeView === 'feed'" class="title">Game Feed</h2>
        <h2 v-else-if="activeView === 'options'" class="title">Options</h2>
      </div>

      <!-- Avatar/Soldier overlay -->
      <div class="avatar-overlay" :class="{ 'shifted-left': activeView !== 'play' }">
        <img src="/layer1.png" alt="Avatar" class="avatar-image" />
      </div>

      <!-- Dynamic Component Views -->
      <PlayView v-if="activeView === 'play'" />

      <!-- Server status gumball (only visible in play view) -->
      <div v-if="activeView === 'play'" class="server-status">
        <div class="status-gumball" :class="{
          'online': serverStatus === 'online',
          'offline': serverStatus === 'offline',
          'checking': serverStatus === 'checking',
          'ping': serverStatus === 'online',
          'ping-inward': serverStatus === 'offline'
        }"></div>
        <span class="status-text">
          {{
            serverStatus === 'online' ? serverInfo :
              serverStatus === 'offline' ? 'Server is Offline' :
                serverStatus === 'checking' ? 'Checking...' :
                  '⚪ Status Unknown'
          }}
        </span>
      </div>
    </div>

    <!-- Background Panel (Main Content) -->
    <main class="background-panel">
      <!-- Dynamic Component Views for Main Panel -->
      <FeedView v-if="activeView === 'feed'" />
      <ModsView v-if="activeView === 'mods'" :mods="mods" :arma3Directory="arma3Directory"
        :isLoadingMods="isLoadingMods" @goto-options="setActiveView('options')" @refresh="loadMods"
        @toggle-mod="toggleMod" @toggle-all="toggleAllMods" />
      <OptionsView v-if="activeView === 'options'" />

      <!-- Global Progress Bar -->
      <ProgressBar />

      <!-- New Notification System -->
      <div v-if="showNotification && currentNotification" class="mod-notification" 
           :class="`notification-${currentNotification.notification_type}`" 
           @click="dismissNotification">
        <div class="notification-content">
          <svg v-if="currentNotification.notification_type === 'up-to-date'" class="notification-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
          </svg>
          <svg v-else-if="currentNotification.notification_type === 'updates-available'" class="notification-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 4V1l-4 4 4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46A7.93 7.93 0 0022 12c0-5.52-4.48-10-10-10zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74A7.93 7.93 0 002 12c0 5.52 4.48 10 10 10v3l4-4-4-4v3z"/>
          </svg>
          <svg v-else class="notification-icon" viewBox="0 0 24 24" fill="currentColor">
            <path d="M13 13h-2V7h2m0 10h-2v-2h2M12 2A10 10 0 0 0 2 12a10 10 0 0 0 10 10 10 10 0 0 0 10-10A10 10 0 0 0 12 2z"/>
          </svg>
          <div class="notification-text">
            <strong>{{ currentNotification.title }}</strong>
            <p>{{ currentNotification.message }}</p>
          </div>
        </div>
        <button class="notification-close" @click.stop="dismissNotification">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
          </svg>
        </button>
      </div>


      <!-- Version Display -->
      <div class="version-display">
        <span class="version-text">v{{ packageInfo.version }}</span>
      </div>
    </main>
  </div>
</template>

<style>
/* ===== GLOBAL STYLES ===== */
* {
  margin: 0 !important;
  padding: 0 !important;
  box-sizing: border-box;
}

html,
body {
  height: 100%;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  font-smooth: always;
  text-rendering: optimizeLegibility;
  font-feature-settings: "kern" 1;
  font-kerning: normal;
  font-variant-ligatures: common-ligatures;
}

:root {
  --primary-bg: #18181b;
  --secondary-bg: #f6f6f6;
  --accent-color: #fde68a;
  --text-primary: #f6f6f6;
  --text-secondary: rgba(246, 246, 246, 0.8);
  --border-color: rgba(253, 230, 138, 0.2);
  --shadow-dark: rgba(0, 0, 0, 0.3);

  color: #0f0f0f;
  background-color: var(--secondary-bg);
}

/* ===== LAYOUT STYLES ===== */
.app-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
}

/* ===== NAVBAR STYLES ===== */
.navbar {
  width: 80px;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px) saturate(150%);
  -webkit-backdrop-filter: blur(8px) saturate(150%);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.05);
  border-right: 1px solid rgba(253, 230, 138, 0.1);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 1rem 0 !important;
  z-index: 200;
  color: var(--text-secondary);

  .nav-buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 2rem !important;
  }

  .nav-button {
    width: 50px;
    height: 50px;
    background-color: transparent;
    border: 2px solid rgba(253, 230, 138, 0.3);
    border-radius: 8px;
    color: rgba(246, 246, 246, 0.7);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;

    &:hover {
      background-color: rgba(253, 230, 138, 0.1);
      border-color: rgba(253, 230, 138, 0.6);
      color: var(--accent-color);
      transform: translateX(2px);
    }

    &.active {
      background-color: var(--accent-color);
      border-color: var(--accent-color);
      color: #0f0f0f;
      box-shadow: 0 0 20px rgba(253, 230, 138, 0.4);
    }
  }

  .nav-icon {
    width: 24px;
    height: 24px;
  }
}

/* ===== DARK PANEL STYLES ===== */
.dark-panel {
  width: 325px;
  height: 100vh;
  background-color: rgba(24, 24, 27, 0.95);
  backdrop-filter: blur(10px);
  position: relative;
  display: flex;
  flex-direction: column;
  z-index: 200;

  .view-title {
    position: absolute;
    top: 40px;
    left: 0;
    right: 0;
    display: flex;
    justify-content: center;
    z-index: 300;

    .title {
      color: var(--accent-color);
      font-size: 1.5rem;
      font-weight: 600;
      text-align: center;
      text-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
      background: rgba(24, 24, 27, 0.9);
      backdrop-filter: blur(10px);
      padding: 0.75rem 1.5rem !important;
      border-radius: 8px;
      border: 1px solid var(--border-color);
      box-shadow: 0 4px 20px var(--shadow-dark);
      transition: all 0.3s ease;

      &:hover {
        border-color: rgba(253, 230, 138, 0.4);
        box-shadow: 0 6px 25px rgba(0, 0, 0, 0.4);
      }
    }
  }

  .avatar-overlay {
    position: absolute;
    top: 0;
    left: 550px;
    width: 100%;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 400;
    pointer-events: none;
    transform: translateY(75px) scale(0.9);
    transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);

    &.shifted-left {
      transform: translateY(125px) scale(0.8);
      left: 225px;
    }

    .avatar-image {
      height: 100vh;
      width: auto;
      object-fit: contain;
      filter: drop-shadow(0 0 20px var(--shadow-dark));
    }
  }

  .server-status {
    position: absolute;
    bottom: 20px;
    left: 20px;
    right: 20px;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--text-secondary);
    font-size: 0.9rem;
    z-index: 500;

    .status-gumball {
      width: 12px;
      height: 12px;
      border-radius: 50%;
      transition: all 0.3s ease;

      &.online {
        background-color: var(--accent-color);
        box-shadow: 0 0 10px rgba(253, 230, 138, 0.4);
      }

      &.offline {
        background-color: transparent;
        border: 1px solid rgba(253, 230, 138, 0.3);
      }

      &.checking {
        background-color: var(--accent-color);
        box-shadow: 0 0 10px rgba(253, 230, 138, 0.4);
        animation: pulse 1.5s infinite;
      }

      &.unknown {
        background-color: transparent;
        border: 1px solid rgba(253, 230, 138, 0.3);
      }

      &.ping {
        animation: ping-shadow 2s cubic-bezier(0, 0, 0.2, 1) infinite;
      }

      &.ping-inward {
        animation: ping-inward-shadow 2s cubic-bezier(0, 0, 0.2, 1) infinite;
      }
    }
  }
}

/* ===== BACKGROUND PANEL STYLES ===== */
.background-panel {
  flex: 1;
  height: 100vh;
  background: left center / cover no-repeat url(/layer0.png), var(--primary-bg);
  position: relative;
  overflow-y: auto;
  scrollbar-width: none;
  /* For Firefox */
  -ms-overflow-style: none;
  /* For IE/Edge */
}

.background-panel::-webkit-scrollbar {
  display: none;
  /* For Chrome, Safari, and Opera */
}

/* ===== VERSION DISPLAY STYLES ===== */
.version-display {
  position: fixed;
  bottom: 20px;
  right: 20px;
  z-index: 1000;
}

.version-text {
  color: var(--text-secondary);
  font-size: 0.8rem;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  padding: 0.5rem 0.75rem !important;
  font-family: 'Courier New', monospace;
  letter-spacing: 0.5px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  transition: all 0.3s ease;
}

.version-text:hover {
  background: rgba(0, 0, 0, 0.6);
  border-color: rgba(253, 230, 138, 0.3);
  color: var(--accent-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

/* ===== ANIMATIONS ===== */
@keyframes ping-shadow {
  0% {
    box-shadow: 0 0 rgba(253, 230, 138, 0.702);
  }

  100% {
    box-shadow: 0 0 0 10px rgba(253, 230, 138, 0);
  }
}

@keyframes ping-inward-shadow {
  0% {
    box-shadow: 0 0 0 10px rgba(253, 230, 138, 0);
  }

  100% {
    box-shadow: 0 0 rgba(253, 230, 138, 0.302);
  }
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

/* ===== MOD NOTIFICATION STYLES ===== */
.mod-notification {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 1001;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border: 1px solid rgba(253, 230, 138, 0.3);
  border-radius: 12px;
  padding: 1rem !important;
  max-width: 320px;
  box-shadow: 0 6px 24px rgba(0, 0, 0, 0.6);
  cursor: pointer;
  transition: all 0.3s ease;
  animation: slideInFromRight 0.5s ease;
}

.mod-notification.notification-up-to-date {
  border-color: rgba(34, 197, 94, 0.4);
}

.mod-notification.notification-up-to-date .notification-icon {
  color: #22c55e;
}

.mod-notification.notification-up-to-date .notification-text strong {
  color: #22c55e;
}

.mod-notification.notification-updates-available {
  border-color: rgba(253, 230, 138, 0.4);
}

.mod-notification.notification-updates-available .notification-icon {
  color: var(--accent-color);
}

.mod-notification.notification-updates-available .notification-text strong {
  color: var(--accent-color);
}

.mod-notification.notification-error {
  border-color: rgba(239, 68, 68, 0.4);
}

.mod-notification.notification-error .notification-icon {
  color: #ef4444;
}

.mod-notification.notification-error .notification-text strong {
  color: #ef4444;
}

.mod-notification:hover {
  border-color: rgba(253, 230, 138, 0.6);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.7);
  transform: translateY(-2px);
}


.notification-content {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
}

.notification-icon {
  width: 24px;
  height: 24px;
  color: var(--accent-color);
  flex-shrink: 0;
  margin-top: 0.125rem !important;
}

.notification-text {
  flex: 1;
}

.notification-text strong {
  display: block;
  color: var(--accent-color);
  font-weight: 600;
  margin-bottom: 0.25rem !important;
  font-size: 0.9rem;
}

.notification-text p {
  color: var(--text-secondary);
  font-size: 0.8rem;
  line-height: 1.4;
  margin: 0 !important;
}

.notification-close {
  position: absolute;
  top: 0.75rem;
  right: 0.75rem;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0.25rem !important;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.notification-close:hover {
  background: rgba(255, 255, 255, 0.1);
  color: var(--accent-color);
}

.notification-close svg {
  width: 16px;
  height: 16px;
}

@keyframes slideInFromRight {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

/* ===== DARK MODE ===== */
@media (prefers-color-scheme: dark) {
  :root {
    color: var(--text-primary);
    background-color: var(--primary-bg);
  }
}
</style>
