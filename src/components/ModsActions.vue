<template>
  <div class="mods-actions">
    <div class="left-section">
      <div class="mod-stats">
        <span class="stat-item">
          <span class="stat-label">Total:</span>
          <span class="stat-value">{{ mods.length }}</span>
        </span>
        <span class="stat-item">
          <span class="stat-label">Enabled:</span>
          <span class="stat-value enabled">{{ getEnabledModsCount() }}</span>
        </span>
      </div>
    </div>

    <div class="center-section">
      <div class="search-container">
        <svg class="search-icon" viewBox="0 0 24 24" fill="currentColor">
          <path
            d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z">
          </path>
        </svg>
        <input type="text" class="search-input" placeholder="Search mods..." v-model="searchQuery"
          @input="handleSearch" />
        <button v-if="searchQuery" class="clear-search" @click="clearSearch" title="Clear search">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path
              d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z">
            </path>
          </svg>
        </button>
      </div>
    </div>

    <div class="action-buttons">
      <button class="action-btn preset-btn" @click="showPresetMenu = !showPresetMenu" :disabled="mods.length === 0">
        Presets
      </button>
      <button class="action-btn" @click="$emit('toggle-all', true)" :disabled="mods.length === 0">
        Enable All
      </button>
      <button class="action-btn" @click="$emit('toggle-all', false)" :disabled="mods.length === 0">
        Disable All
      </button>
      <button class="action-btn refresh" @click="$emit('refresh')" :disabled="!arma3Directory || isLoadingMods">
        <span v-if="isLoadingMods" class="loading-spinner"></span>
        <span v-else>Refresh</span>
      </button>
    </div>
  </div>

  <!-- Preset Toolbar (positioned below the preset bar) -->
  <div v-if="showPresetMenu" class="preset-toolbar">
    <div class="preset-toolbar-content">
      <!-- Save Current Configuration Row -->
      <div class="preset-save-row">
        <input type="text" v-model="newPresetName" placeholder="Preset name..." class="preset-input-small"
          @keyup.enter="saveCurrentPreset" />
        <input type="text" v-model="newPresetDescription" placeholder="Description (optional)..."
          class="preset-input-small" />
        <button class="preset-save-btn-small" @click="saveCurrentPreset" :disabled="!newPresetName.trim()">
          Save Preset
        </button>
      </div>

      <!-- Manage Presets Row -->
      <div class="preset-manage-row">
        <div class="preset-dropdown-container">
          <button class="preset-dropdown-btn" @click="showPresetsDropdown = !showPresetsDropdown"
            :disabled="presets.length === 0">
            <span>{{ selectedPresetName || (presets.length === 0 ? 'No presets available' : 'Select a preset...')
            }}</span>
            <svg class="dropdown-arrow" :class="{ 'rotated': showPresetsDropdown }" viewBox="0 0 24 24"
              fill="currentColor">
              <path d="M7 10l5 5 5-5z" />
            </svg>
          </button>
          <div v-if="showPresetsDropdown && presets.length > 0" class="preset-dropdown-menu">
            <div v-for="preset in presets" :key="preset.id" class="preset-dropdown-item" @click="selectPreset(preset)">
              <div class="preset-item-info">
                <div class="preset-item-name">{{ preset.name }}</div>
                <div class="preset-item-meta">
                  <span class="preset-item-description" v-if="preset.description">{{ preset.description }}</span>
                  <span class="preset-item-mod-count">{{ preset.mods.length }} mods</span>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="preset-actions">
          <button class="preset-action-btn apply" @click="applySelectedPreset" :disabled="!selectedPreset">
            Apply
          </button>
          <button class="preset-action-btn delete" @click="deleteSelectedPreset" :disabled="!selectedPreset">
            Delete
          </button>
        </div>
      </div>

      <!-- Close Button -->
      <div class="preset-close-section">
        <button class="preset-close-btn-compact" @click="showPresetMenu = false">
          ×
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

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

interface ModPreset {
  id: string;
  name: string;
  description?: string;
  mods: string[];
  startup_parameters?: string;
  created_at: string;
  updated_at: string;
}

interface Props {
  mods: ModInfo[];
  arma3Directory: string;
  isLoadingMods?: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  'toggle-all': [enable: boolean];
  'refresh': [];
  'search': [query: string];
  'apply-preset': [modPaths: string[]];
}>();

// Existing state
const isLoadingMods = props.isLoadingMods ?? false;
const searchQuery = ref('');

// Preset-related state
const showPresetMenu = ref(false);
const showPresetsDropdown = ref(false);
const presets = ref<ModPreset[]>([]);
const selectedPreset = ref<ModPreset | null>(null);
const selectedPresetName = ref('');
const newPresetName = ref('');
const newPresetDescription = ref('');

// Existing functions
function getEnabledModsCount(): number {
  return props.mods.filter(mod => mod.enabled).length;
}

function handleSearch() {
  emit('search', searchQuery.value);
}

function clearSearch() {
  searchQuery.value = '';
  emit('search', '');
}

// Preset functions
async function loadPresets() {
  try {
    presets.value = await invoke<ModPreset[]>('load_mod_presets');
  } catch (error) {
    console.error('Failed to load presets:', error);
  }
}

async function saveCurrentPreset() {
  if (!newPresetName.value.trim()) return;

  try {
    // Get currently enabled mod paths
    const enabledModPaths = props.mods
      .filter(mod => mod.enabled)
      .map(mod => mod.path);

    const newPreset = await invoke<ModPreset>('save_mod_preset', {
      name: newPresetName.value.trim(),
      description: newPresetDescription.value.trim() || null,
      modPaths: enabledModPaths,
      startupParameters: null // We can add this later if needed
    });

    presets.value.push(newPreset);
    newPresetName.value = '';
    newPresetDescription.value = '';

    console.log('Preset saved successfully:', newPreset);
  } catch (error) {
    console.error('Failed to save preset:', error);
  }
}

async function applyPreset(preset: ModPreset) {
  try {
    emit('apply-preset', preset.mods);
    showPresetMenu.value = false;
    console.log('Applied preset:', preset.name);
  } catch (error) {
    console.error('Failed to apply preset:', error);
  }
}

function selectPreset(preset: ModPreset) {
  selectedPreset.value = preset;
  selectedPresetName.value = preset.name;
  showPresetsDropdown.value = false;
}

function applySelectedPreset() {
  if (selectedPreset.value) {
    applyPreset(selectedPreset.value);
  }
}

async function deleteSelectedPreset() {
  if (selectedPreset.value) {
    await deletePreset(selectedPreset.value.id);
    // Clear selection if the deleted preset was selected
    if (selectedPreset.value && !presets.value.find(p => p.id === selectedPreset.value!.id)) {
      selectedPreset.value = null;
      selectedPresetName.value = '';
    }
  }
}

async function deletePreset(presetId: string) {
  try {
    await invoke('delete_mod_preset', { presetId });
    presets.value = presets.value.filter(p => p.id !== presetId);
    console.log('Preset deleted successfully');
  } catch (error) {
    console.error('Failed to delete preset:', error);
  }
}


// Load presets when component mounts
onMounted(() => {
  loadPresets();
});
</script>

<style scoped>
.mods-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px) saturate(150%);
  -webkit-backdrop-filter: blur(8px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 1rem !important;
  margin-bottom: 1rem !important;
  position: sticky;
  top: 2rem;
  z-index: 1000;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.05);

  .left-section {
    flex-shrink: 0;
  }

  .mod-stats {
    display: flex;
    gap: 1rem;

    .stat-item {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      font-size: 0.9rem;

      .stat-label {
        color: var(--text-secondary);
      }

      .stat-value {
        color: var(--accent-color);
        font-weight: 600;

        &.enabled {
          color: #4ade80;
        }

        &.filtered {
          color: #60a5fa;
        }
      }
    }
  }

  .action-buttons {
    display: flex;
    gap: 0.5rem;

    .action-btn {
      padding: 0.5rem 0.75rem !important;
      background: rgba(0, 0, 0, 0.2);
      backdrop-filter: blur(8px);
      -webkit-backdrop-filter: blur(8px);
      border: 1px solid rgba(253, 230, 138, 0.3);
      border-radius: 6px;
      color: var(--accent-color);
      cursor: pointer;
      font-size: 0.85rem;
      transition: all 0.3s ease;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.05);

      &.refresh {
        background: rgba(0, 0, 0, 0.2);
        border-color: rgba(59, 130, 246, 0.3);
        color: #60a5fa;

        &:hover:not(:disabled) {
          background: rgba(59, 130, 246, 0.15);
          border-color: rgba(59, 130, 246, 0.5);
          color: #60a5fa;
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
        }
      }

      &:hover:not(:disabled) {
        background: rgba(253, 230, 138, 0.15);
        border-color: rgba(253, 230, 138, 0.5);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
        backdrop-filter: blur(12px);
        -webkit-backdrop-filter: blur(12px);
      }

      &:disabled {
        opacity: 0.5;
        cursor: not-allowed;
      }
    }
  }
}

.center-section {
  flex: 1;
  display: flex;
  justify-content: center;
  margin: 0 2rem;

  .search-container {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
    max-width: 220px;

    .search-icon {
      position: absolute;
      left: 0.75rem;
      width: 18px;
      height: 18px;
      color: var(--text-secondary);
      pointer-events: none;
      z-index: 1;
    }

    .search-input {
      width: 100%;
      padding: 0.5rem 0.75rem 0.5rem 2.5rem !important;
      background: rgba(0, 0, 0, 0.2);
      backdrop-filter: blur(8px);
      -webkit-backdrop-filter: blur(8px);
      border: 1px solid rgba(255, 255, 255, 0.1);
      border-radius: 8px;
      color: var(--text-primary);
      font-size: 0.85rem;
      transition: all 0.3s ease;
      outline: none;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.05);

      &::placeholder {
        color: var(--text-secondary);
        opacity: 0.7;
      }

      &:focus {
        border-color: rgba(253, 230, 138, 0.4);
        background: rgba(0, 0, 0, 0.3);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3), 0 0 0 1px rgba(253, 230, 138, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.1);
        backdrop-filter: blur(12px);
        -webkit-backdrop-filter: blur(12px);
      }
    }

    .clear-search {
      position: absolute;
      right: 0.5rem;
      width: 20px;
      height: 20px;
      background: none;
      border: none;
      color: var(--text-secondary);
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 3px;
      transition: all 0.2s ease;

      &:hover {
        color: var(--accent-color);
        background-color: rgba(253, 230, 138, 0.1);
      }

      svg {
        width: 14px;
        height: 14px;
      }
    }
  }
}

.preset-btn {
  padding: 0.5rem 0.75rem !important;
  background: rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border: 1px solid rgba(253, 230, 138, 0.3);
  border-radius: 6px;
  color: var(--accent-color);
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.05);

  &:hover:not(:disabled) {
    background: rgba(253, 230, 138, 0.15);
    border-color: rgba(253, 230, 138, 0.5);
    color: var(--accent-color);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.loading-spinner {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid rgba(96, 165, 250, 0.3);
  border-radius: 50%;
  border-top-color: #60a5fa;
  animation: spin 0.8s ease-in-out infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* Preset Toolbar Styles */
.preset-toolbar {
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px) saturate(150%);
  -webkit-backdrop-filter: blur(8px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 2rem !important;
  margin-bottom: 1rem !important;
  position: sticky;
  top: 2rem;
  z-index: 1000;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.05);

  .preset-toolbar-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    z-index: 1000;
    width: 100%;
    position: relative;
  }

  .preset-save-row {
    display: flex;
    gap: 1rem;
    align-items: center;
    width: 100%;
    flex-wrap: wrap;
  }

  .preset-manage-row {
    display: flex;
    gap: 1rem;
    align-items: center;
    width: 100%;
  }
}


.preset-input-small {
  padding: 0.375rem 0.5rem !important;
  background: rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 0.9rem;
  transition: all 0.3s ease;
  outline: none;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.05);
  width: 180px;
  min-height: 44px;
  box-sizing: border-box;

  &:focus {
    border-color: rgba(253, 230, 138, 0.4);
    background: rgba(0, 0, 0, 0.3);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3), 0 0 0 1px rgba(253, 230, 138, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
  }

  &::placeholder {
    color: var(--text-secondary);
    opacity: 0.7;
  }
}

.preset-save-btn-small {
  padding: 0.375rem 0.75rem !important;
  background: rgba(34, 197, 94, 0.2);
  border: 1px solid rgba(34, 197, 94, 0.3);
  border-radius: 6px;
  color: #4ade80;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.2s ease;
  font-weight: 500;
  min-height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  white-space: nowrap;

  &:hover:not(:disabled) {
    background: rgba(34, 197, 94, 0.3);
    border-color: rgba(34, 197, 94, 0.5);
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.preset-actions {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex-shrink: 0;
}

.preset-action-btn {
  padding: 0.375rem 0.75rem !important;
  border-radius: 6px;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
  font-weight: 500;
  min-height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-sizing: border-box;
  white-space: nowrap;

  &.apply {
    background: rgba(59, 130, 246, 0.2);
    border-color: rgba(59, 130, 246, 0.3);
    color: #60a5fa;

    &:hover:not(:disabled) {
      background: rgba(59, 130, 246, 0.3);
      border-color: rgba(59, 130, 246, 0.5);
    }
  }

  &.delete {
    background: rgba(239, 68, 68, 0.2);
    border-color: rgba(239, 68, 68, 0.3);
    color: #f87171;

    &:hover:not(:disabled) {
      background: rgba(239, 68, 68, 0.3);
      border-color: rgba(239, 68, 68, 0.5);
    }
  }

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
}

.preset-close-section {
  position: absolute;
  top: 0;
  right: 0;
  display: flex;
  align-items: flex-start;

  .preset-close-btn-compact {
    width: 32px;
    height: 32px;
    background: rgba(107, 114, 128, 0.2);
    border: 1px solid rgba(107, 114, 128, 0.3);
    border-radius: 6px;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.2rem;
    transition: all 0.2s ease;

    &:hover {
      background: rgba(239, 68, 68, 0.2);
      border-color: rgba(239, 68, 68, 0.3);
      color: #f87171;
    }
  }
}

/* Preset Dropdown Styles */
.preset-dropdown-container {
  position: relative;
  width: 100%;
  max-width: 400px;

  .preset-dropdown-btn {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.375rem 0.5rem !important;
    background: rgba(0, 0, 0, 0.2);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    color: var(--text-primary);
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s ease;
    outline: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2), inset 0 1px 0 rgba(255, 255, 255, 0.05);
    box-sizing: border-box;
    min-height: 44px;

    &:hover {
      border-color: rgba(253, 230, 138, 0.4);
      background: rgba(0, 0, 0, 0.3);
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3), inset 0 1px 0 rgba(255, 255, 255, 0.1);
      backdrop-filter: blur(12px);
      -webkit-backdrop-filter: blur(12px);
    }
  }

  .dropdown-arrow {
    width: 20px;
    height: 20px;
    color: var(--text-secondary);
    transition: transform 0.2s ease;

    &.rotated {
      transform: rotate(180deg);
    }
  }

  .preset-dropdown-menu {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: rgba(0, 0, 0, 0.8);
    backdrop-filter: blur(12px) saturate(150%);
    -webkit-backdrop-filter: blur(12px) saturate(150%);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    margin-top: 0.5rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.1);
    z-index: 1001 !important;
    max-height: 300px;
    overflow-y: auto;

    .preset-dropdown-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 1rem !important;
      border-bottom: 1px solid rgba(255, 255, 255, 0.1);
      transition: all 0.2s ease;
      box-sizing: border-box;

      &:last-child {
        border-bottom: none;
      }

      &:hover {
        background: rgba(255, 255, 255, 0.1);
      }

      .preset-item-info {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 0.3rem;
        margin-right: 1rem;

        .preset-item-name {
          color: var(--text-primary);
          font-size: 0.95rem;
          font-weight: 600;
        }

        .preset-item-meta {
          display: flex;
          align-items: center;
          gap: 0.75rem;
          flex-wrap: wrap;

          .preset-item-description {
            color: var(--text-secondary);
            font-size: 0.8rem;
            font-style: italic;
          }

          .preset-item-mod-count {
            color: var(--accent-color);
            font-size: 0.8rem;
            font-weight: 500;
          }
        }
      }
    }
  }
}
</style>
