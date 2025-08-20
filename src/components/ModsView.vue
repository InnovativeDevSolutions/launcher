<template>
  <div class="mods-content-main">
    <div v-if="!arma3Directory" class="no-directory">
      <div class="no-directory-content">
        <svg class="folder-icon" viewBox="0 0 24 24" fill="currentColor">
          <path d="M10 4H4c-1.11 0-2 .89-2 2v12c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2h-8l-2-2z" />
        </svg>
        <h3>No Arma 3 Directory Selected</h3>
        <p>Please select your Arma 3 installation directory in Options to manage mods.</p>
        <button @click="$emit('goto-options')" class="goto-options-button">
          Go to Options
        </button>
      </div>
    </div>

    <div v-else-if="isLoadingMods" class="loading-mods">
      <div class="loading-mods-content">
        <div class="loading-spinner-large"></div>
        <h3>Scanning Mods...</h3>
        <p>Searching for mods in your Arma 3 directory and Steam Workshop folders. This may take a moment.</p>
      </div>
    </div>

    <div v-else-if="mods.length === 0" class="no-mods">
      <div class="no-mods-content">
        <svg class="mods-icon" viewBox="0 0 24 24" fill="currentColor">
          <path
            d="M20.5 11H19V7c0-1.1-.9-2-2-2h-4V3.5C13 2.12 11.88 1 10.5 1S8 2.12 8 3.5V5H4c-1.1 0-2 .9-2 2v4h1.5c1.38 0 2.5 1.12 2.5 2.5S4.88 16 3.5 16H2v4c0 1.1.9 2 2 2h4v-1.5c0-1.38 1.12-2.5 2.5-2.5s2.5 1.12 2.5 2.5V22h4c1.1 0 2-.9 2-2v-4h1.5c1.38 0 2.5-1.12 2.5-2.5S21.88 11 20.5 11z" />
        </svg>
        <h3>No Mods Found</h3>
        <p>No mods were found in your Arma 3 directory. Install mods from Steam Workshop or manually to get started.</p>
        <button @click="$emit('refresh')" class="refresh-button">
          Refresh Mods
        </button>
      </div>
    </div>

    <div v-else class="mods-list-container">
      <ModUpdater :arma3Directory="arma3Directory" @mods-updated="handleModsUpdated" />
      
      <ModsActions :mods="mods" :arma3Directory="arma3Directory" :isLoadingMods="isLoadingMods"
        @toggle-all="(enable: boolean) => $emit('toggle-all', enable)" @refresh="$emit('refresh')"
        @search="handleSearch" @apply-preset="handleApplyPreset" />

      <div class="mods-list">
        <div v-if="searchQuery && filteredMods.length === 0" class="no-results">
          <div class="no-results-content">
            <svg class="search-icon-large" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z">
              </path>
            </svg>
            <h3>No mods found</h3>
            <p>No mods match "{{ searchQuery }}". Try adjusting your search terms.</p>
          </div>
        </div>
        <ModItem v-for="(mod, index) in filteredMods" :key="mod.path" :mod="mod" :index="index"
          @toggle="handleToggleMod(index)" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import ModItem from './ModItem.vue';
import ModsActions from './ModsActions.vue';
import ModUpdater from './ModUpdater.vue';

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

interface Props {
  mods: ModInfo[];
  arma3Directory: string;
  isLoadingMods?: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  'goto-options': [];
  'refresh': [];
  'toggle-mod': [index: number];
  'toggle-all': [enable: boolean];
}>();

const searchQuery = ref('');

const filteredMods = computed(() => {
  if (!searchQuery.value) {
    return props.mods;
  }
  return props.mods.filter(mod =>
    mod.name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

function handleSearch(query: string) {
  searchQuery.value = query;
}

function handleToggleMod(index: number) {
  // Find the original index in the full mods array
  const filteredMod = filteredMods.value[index];
  const originalIndex = props.mods.findIndex(mod => mod.path === filteredMod.path);
  emit('toggle-mod', originalIndex);
}

function handleApplyPreset(modPaths: string[]) {
  // Disable all mods first
  props.mods.forEach(mod => {
    mod.enabled = false;
  });

  // Enable mods that are in the preset
  props.mods.forEach(mod => {
    if (modPaths.includes(mod.path)) {
      mod.enabled = true;
    }
  });

  console.log('Applied preset with', modPaths.length, 'mods');
}

function handleModsUpdated() {
  // Emit refresh to trigger mod scan after updates
  emit('refresh');
}

</script>

<style scoped>
.mods-content-main {
  max-width: 1000px;
  width: 100%;
  margin: 0 auto;
  padding: 2rem !important;
  min-height: 100vh;
}

.no-directory,
.no-mods,
.loading-mods {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 400px;
}

.no-directory-content,
.no-mods-content,
.loading-mods-content {
  text-align: center;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px) saturate(150%);
  -webkit-backdrop-filter: blur(8px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 3rem 2rem !important;
  max-width: 500px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.05);
  transition: all 0.3s ease;
}

.folder-icon,
.mods-icon {
  width: 64px;
  height: 64px;
  color: var(--accent-color);
  margin: 0 auto 1.5rem auto !important;
  opacity: 0.8;
}

.loading-spinner-large {
  width: 64px;
  height: 64px;
  border: 4px solid rgba(253, 230, 138, 0.2);
  border-radius: 50%;
  border-top-color: var(--accent-color);
  animation: spin 1s ease-in-out infinite;
  margin: 0 auto 1.5rem auto !important;
}

.no-directory-content h3,
.no-mods-content h3,
.loading-mods-content h3 {
  color: var(--accent-color);
  margin-bottom: 1rem !important;
  font-size: 1.5rem;
}

.no-directory-content p,
.no-mods-content p,
.loading-mods-content p {
  color: var(--text-secondary);
  margin-bottom: 2rem !important;
  line-height: 1.6;
}

.goto-options-button,
.refresh-button {
  padding: 0.75rem 1.5rem !important;
  background-color: var(--accent-color);
  color: #0f0f0f;
  border: none;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
}

.goto-options-button:hover,
.refresh-button:hover {
  box-shadow: 0 0 20px rgba(253, 230, 138, 0.6);
  transform: translateY(-2px);
}

.mods-list-container {
  position: relative;
}

.mods-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  padding-top: 1rem !important;
  max-height: calc(100vh - 175px);
  overflow-y: auto;
  scrollbar-width: none;
  /* Firefox */
  -ms-overflow-style: none;
  /* IE and Edge */
}

.mods-list::-webkit-scrollbar {
  display: none;
  /* Chrome, Safari and Opera */
}

.no-results {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 200px;
  padding: 2rem;
}

.no-results-content {
  text-align: center;
  background: rgba(0, 0, 0, 0.25);
  backdrop-filter: blur(8px) saturate(150%);
  -webkit-backdrop-filter: blur(8px) saturate(150%);
  border: 1px solid rgba(253, 230, 138, 0.2);
  border-radius: 12px;
  padding: 2rem !important;
  max-width: 400px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.05);
  transition: all 0.3s ease;
}

.search-icon-large {
  width: 48px;
  height: 48px;
  color: var(--accent-color);
  margin: 0 auto 1rem auto !important;
  opacity: 0.7;
}

.no-results-content h3 {
  color: var(--accent-color);
  margin-bottom: 0.75rem !important;
  font-size: 1.25rem;
}

.no-results-content p {
  color: var(--text-secondary);
  line-height: 1.5;
  font-size: 0.9rem;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
