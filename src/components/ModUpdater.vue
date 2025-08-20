<template>
  <div class="mod-updater">
    <div class="updater-header">
      <h3 class="updater-title">Mod Updates</h3>
      <button 
        @click="() => checkForUpdates()" 
        :disabled="isChecking || isUpdating"
        class="check-updates-btn"
      >
        <svg v-if="!isChecking" class="btn-icon" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 4V1l-4 4 4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46A7.93 7.93 0 0022 12c0-5.52-4.48-10-10-10zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74A7.93 7.93 0 002 12c0 5.52 4.48 10 10 10v3l4-4-4-4v3z"/>
        </svg>
        <div v-else class="loading-spinner"></div>
        {{ isChecking ? 'Checking...' : 'Check for Updates' }}
      </button>
    </div>

    <div v-if="updateError" class="update-error">
      <p>{{ updateError }}</p>
    </div>


    <div v-if="updateInfo.length > 0" class="updates-list">
      <div class="updates-header">
        <p>{{ updatesAvailable }} update(s) available:</p>
        <button 
          @click="updateAllMods" 
          :disabled="isUpdating"
          class="update-all-btn"
        >
          <div v-if="isUpdating" class="loading-spinner"></div>
          {{ isUpdating ? 'Updating...' : 'Update All' }}
        </button>
      </div>

      <div class="update-items">
        <div 
          v-for="update in updateInfo" 
          :key="update.mod_name"
          class="update-item"
          :class="{ 'new-mod': update.is_new_mod }"
        >
          <div class="update-info">
            <div class="mod-name">
              {{ update.mod_name }}
              <span v-if="update.is_new_mod" class="new-badge">NEW</span>
            </div>
            <div class="version-info">
              <span v-if="update.current_version" class="current-version">
                Current: v{{ update.current_version }}
              </span>
              <span class="arrow">→</span>
              <span class="remote-version">
                Latest: v{{ update.remote_version }}
              </span>
            </div>
          </div>
          <button 
            @click="updateSingleMod(update)" 
            :disabled="isUpdating"
            class="update-single-btn"
          >
            {{ update.is_new_mod ? 'Install' : 'Update' }}
          </button>
        </div>
      </div>
    </div>

    <div v-if="updateMessages.length > 0" class="update-messages">
      <h4>Update Log:</h4>
      <div class="messages-list">
        <p v-for="(message, index) in updateMessages" :key="index" class="update-message">
          {{ message }}
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, inject } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

interface ModUpdateInfo {
  mod_name: string;
  current_version?: string;
  remote_version: string;
  needs_update: boolean;
  is_new_mod: boolean;
  download_url: string;
  size?: number;
}


interface Props {
  arma3Directory: string;
  autoCheck?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  autoCheck: true
});

const emit = defineEmits<{
  'mods-updated': [];
}>();

const updateInfo = ref<ModUpdateInfo[]>([]);
const isChecking = ref(false);
const isUpdating = ref(false);
const updateError = ref('');
const updateMessages = ref<string[]>([]);
const hasChecked = ref(false);
let notificationUnlisten: UnlistenFn | null = null;

// Inject auto-update setting from central state
const autoUpdateEnabled = inject<any>('autoUpdateEnabled');
// Inject notification handler
const handleModNotification = inject<any>('handleModNotification');

const updatesAvailable = computed(() => {
  return updateInfo.value.filter(info => info.needs_update).length;
});

async function checkForUpdates(auto = false) {
  if (!props.arma3Directory) {
    if (!auto) updateError.value = 'No Arma 3 directory set';
    return;
  }

  isChecking.value = true;
  updateError.value = '';
  if (!auto) updateMessages.value = [];
  
  try {
    // Use the new notification-based command that will emit notifications
    const updates = await invoke<ModUpdateInfo[]>('check_mod_updates_with_notifications', {
      arma3Directory: props.arma3Directory
    });
    
    updateInfo.value = updates.filter(info => info.needs_update);
    hasChecked.value = true;
    
    if (updateInfo.value.length > 0) {
      // Auto-apply updates if enabled
      if (auto && autoUpdateEnabled.value) {
        setTimeout(() => {
          updateAllMods();
        }, 2000); // Wait 2 seconds before auto-updating
      }
    }
    // Note: No need to show "up to date" message here as it's now handled by notifications
  } catch (error) {
    if (!auto) updateError.value = `Failed to check for updates: ${error}`;
    updateInfo.value = [];
  } finally {
    isChecking.value = false;
  }
}

async function updateSingleMod(modInfo: ModUpdateInfo) {
  isUpdating.value = true;
  updateError.value = '';
  
  try {
    const message = await invoke<string>('update_single_mod', {
      arma3Directory: props.arma3Directory,
      modInfo: modInfo
    });
    
    updateMessages.value.push(message);
    
    // Remove the updated mod from the list
    updateInfo.value = updateInfo.value.filter(info => info.mod_name !== modInfo.mod_name);
    
    emit('mods-updated');
  } catch (error) {
    updateError.value = `Failed to update ${modInfo.mod_name}: ${error}`;
  } finally {
    isUpdating.value = false;
  }
}

async function updateAllMods() {
  isUpdating.value = true;
  updateError.value = '';
  
  try {
    const messages = await invoke<string[]>('update_all_mods', {
      arma3Directory: props.arma3Directory
    });
    
    updateMessages.value.push(...messages);
    
    // Clear the update list
    updateInfo.value = [];
    
    emit('mods-updated');
  } catch (error) {
    updateError.value = `Failed to update mods: ${error}`;
  } finally {
    isUpdating.value = false;
  }
}



function setupNotificationListener() {
  listen<any>('mod-notification', (event) => {
    const notification = event.payload;
    // Forward the notification to the global handler
    if (handleModNotification) {
      handleModNotification(notification);
    }
  }).then(unlisten => {
    notificationUnlisten = unlisten;
  });
}

// Lifecycle hooks
onMounted(() => {
  setupNotificationListener();
  
  // Auto-check for updates on mount only if both autoCheck prop and autoUpdateEnabled are true
  if (props.autoCheck && props.arma3Directory && autoUpdateEnabled.value) {
    setTimeout(() => {
      checkForUpdates(true);
    }, 3000); // Wait 3 seconds after app start
  }
});

onUnmounted(() => {
  if (notificationUnlisten) {
    notificationUnlisten();
  }
});

// Expose checkForUpdates for parent component
defineExpose({ checkForUpdates });
</script>

<script lang="ts">
import { computed } from 'vue';
</script>

<style scoped>
.mod-updater {
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(8px);
  border: 1px solid rgba(253, 230, 138, 0.2);
  border-radius: 12px;
  padding: 1.5rem !important;
  margin-bottom: 1.5rem !important;
}

.updater-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem !important;
}

.updater-title {
  color: var(--accent-color);
  font-size: 1.2rem;
  font-weight: 600;
  margin: 0 !important;
}

.check-updates-btn,
.update-all-btn,
.update-single-btn {
  background: rgba(253, 230, 138, 0.1);
  border: 1px solid rgba(253, 230, 138, 0.3);
  border-radius: 6px;
  color: var(--accent-color);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem !important;
  transition: all 0.2s ease;
}

.check-updates-btn:hover:not(:disabled),
.update-all-btn:hover:not(:disabled),
.update-single-btn:hover:not(:disabled) {
  background: rgba(253, 230, 138, 0.2);
  border-color: rgba(253, 230, 138, 0.5);
}

.check-updates-btn:disabled,
.update-all-btn:disabled,
.update-single-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}


.btn-icon {
  width: 16px;
  height: 16px;
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(253, 230, 138, 0.3);
  border-radius: 50%;
  border-top-color: var(--accent-color);
  animation: spin 1s linear infinite;
}

.update-error {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-radius: 6px;
  color: #ef4444;
  padding: 0.75rem !important;
  margin-bottom: 1rem !important;
}

.no-updates {
  color: #10b981;
  text-align: center;
  padding: 1rem !important;
  font-weight: 500;
}

.updates-list {
  margin-top: 1rem !important;
}

.updates-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem !important;
}

.updates-header p {
  color: var(--text-secondary);
  font-weight: 500;
  margin: 0 !important;
}

.update-items {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.update-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  padding: 1rem !important;
  transition: all 0.2s ease;
}

.update-item.new-mod {
  border-color: rgba(34, 197, 94, 0.3);
  background: rgba(34, 197, 94, 0.05);
}

.update-item:hover {
  background: rgba(0, 0, 0, 0.3);
  border-color: rgba(253, 230, 138, 0.2);
}

.update-info {
  flex: 1;
}

.mod-name {
  color: var(--text-primary);
  font-weight: 600;
  margin-bottom: 0.25rem !important;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.new-badge {
  background: #22c55e;
  color: white;
  font-size: 0.75rem;
  font-weight: 700;
  padding: 0.125rem 0.375rem !important;
  border-radius: 4px;
  text-transform: uppercase;
}

.version-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
}

.current-version {
  color: var(--text-secondary);
}

.arrow {
  color: var(--accent-color);
  font-weight: 600;
}

.remote-version {
  color: var(--accent-color);
  font-weight: 600;
}

.update-single-btn {
  flex-shrink: 0;
  margin-left: 1rem !important;
}

.update-messages {
  margin-top: 1.5rem !important;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  padding-top: 1rem !important;
}

.update-messages h4 {
  color: var(--accent-color);
  font-size: 1rem;
  margin-bottom: 0.5rem !important;
}

.messages-list {
  max-height: 200px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.update-message {
  color: var(--text-secondary);
  font-size: 0.875rem;
  padding: 0.25rem 0 !important;
  margin: 0 !important;
}


@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
