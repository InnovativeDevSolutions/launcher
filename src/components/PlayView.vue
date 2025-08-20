<template>
  <div class="play-content">
    <div class="button-group">
      <button class="launch-button" @click="launchArma3" :disabled="isLaunching || !arma3Directory">
        {{ isLaunching ? 'Launching...' : (hasEnabledMods ? 'Launch with Mods' : 'Launch Game') }}
      </button>
      <button class="join-button" @click="joinServer"
        :disabled="isLaunching || !arma3Directory || serverStatus === 'offline'"
        :class="{ 'disabled': serverStatus === 'offline', 'ping': serverStatus === 'online' }">
        {{ isLaunching ? 'Launching...' : (hasEnabledMods ? 'Join with Mods' : 'Join Server') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { inject, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// TypeScript interfaces
interface ModInfo {
  name: string;
  path: string;
  enabled: boolean;
  mod_type: string;
  workshop_id?: string;
  size?: number;
  last_modified?: string;
  extensions: string[]; // Array of paths to extension DLLs
}

// Inject shared state
const arma3Directory = inject<any>('arma3Directory');
const serverAddress = inject<any>('serverAddress');
const serverPassword = inject<any>('serverPassword');
const serverStatus = inject<any>('serverStatus');
const isLaunching = inject<any>('isLaunching');
const mods = inject<any>('mods');
const startupParameters = inject<any>('startupParameters');
const battleEyeEnabled = inject<any>('battleEyeEnabled');

// Computed property to check if any mods are enabled
const hasEnabledMods = computed(() => {
  return mods?.value?.some((mod: ModInfo) => mod.enabled) || false;
});

async function launchArma3() {
  if (!arma3Directory?.value) {
    alert("Please select Arma 3 directory first in Options.");
    return;
  }

  try {
    isLaunching!.value = true;

    if (hasEnabledMods.value) {
      // Launch with mods using enhanced launcher (supports extensions)
      const enabledMods = mods!.value.filter((mod: ModInfo) => mod.enabled);
      const result = await invoke<string>("launch_arma3_enhanced", {
        arma3Directory: arma3Directory.value,
        serverAddress: null,
        serverPassword: null,
        enabledMods: enabledMods,
        startupParameters: startupParameters!.value || "",
        battleEyeEnabled: battleEyeEnabled!.value
      });
      console.log(result);
    } else {
      // Launch without mods
      const result = await invoke<string>("launch_arma3", {
        arma3Directory: arma3Directory.value,
        serverAddress: null,
        serverPassword: null,
        battleEyeEnabled: battleEyeEnabled!.value
      });
      console.log(result);
    }
  } catch (error) {
    console.error("Error launching Arma 3:", error);
    alert(`Failed to launch Arma 3: ${error}`);
  } finally {
    isLaunching!.value = false;
  }
}

async function joinServer() {
  if (!arma3Directory?.value) {
    alert("Please select Arma 3 directory first in Options.");
    return;
  }

  if (serverStatus?.value === "offline") {
    alert("Server is offline. Cannot join.");
    return;
  }

  try {
    isLaunching!.value = true;

    if (hasEnabledMods.value) {
      // Join server with mods using enhanced launcher (supports extensions)
      const enabledMods = mods!.value.filter((mod: ModInfo) => mod.enabled);
      const result = await invoke<string>("launch_arma3_enhanced", {
        arma3Directory: arma3Directory.value,
        serverAddress: serverAddress?.value,
        serverPassword: serverPassword?.value || null,
        enabledMods: enabledMods,
        startupParameters: startupParameters!.value || "",
        battleEyeEnabled: battleEyeEnabled!.value
      });
      console.log(result);
    } else {
      // Join server without mods
      const result = await invoke<string>("launch_arma3", {
        arma3Directory: arma3Directory.value,
        serverAddress: serverAddress?.value,
        serverPassword: serverPassword?.value || null,
        battleEyeEnabled: battleEyeEnabled!.value
      });
      console.log(result);
    }
  } catch (error) {
    console.error("Error joining server:", error);
    alert(`Failed to join server: ${error}`);
  } finally {
    isLaunching!.value = false;
  }
}
</script>

<style scoped>
.play-content {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2rem;
  z-index: 300;
  padding: 2rem !important;
}

.button-group {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.launch-button {
  width: 250px;
  padding: 0.75rem 0 !important;
  background-color: var(--accent-color);
  color: #0f0f0f;
  border: none;
  border-radius: 0;
  font-size: 1.2em;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
}

.launch-button:hover:not(:disabled) {
  box-shadow: 0 0 20px rgba(253, 230, 138, 0.8);
  color: #fff;
  transform: translateY(-2px);
}

.launch-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.join-button {
  width: 250px;
  padding: 0.75rem 0 !important;
  background-color: #111114;
  color: rgba(255, 255, 255, 0.8);
  border: none;
  border-radius: 0;
  font-size: 1.2em;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
}

.join-button:hover:not(:disabled) {
  background-color: var(--accent-color);
  color: #0f0f0f !important;
  transform: translateY(-2px);
}

.join-button.disabled,
.join-button:disabled {
  background-color: #111114;
  color: #666;
  cursor: not-allowed;
}

.join-button.disabled:hover,
.join-button:disabled:hover {
  background-color: #111114;
  transform: none;
}

.join-button.ping {
  animation: button-ping-shadow 4s ease-in-out infinite;
}

@keyframes button-ping-shadow {

  0%,
  100% {
    box-shadow: 0 0 0px rgba(253, 230, 138, 0.2);
    color: rgba(255, 255, 255, 0.8);
  }

  50% {
    box-shadow: 0 0 15px rgba(253, 230, 138, 0.6);
    color: rgba(253, 230, 138, 0.9);
  }
}
</style>
