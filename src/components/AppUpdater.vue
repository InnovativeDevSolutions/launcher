<template>
  <div class="updater-container">
    <p v-if="updater.state.isChecking">Checking for updates...</p>
    <div v-if="updater.state.updateAvailable">
      <p>A new version is available: {{ updater.state.updateMetadata?.version }}</p>
      <button @click="updater.downloadAndInstallUpdate()" :disabled="updater.state.isDownloading || updater.state.isInstalling">
        <span v-if="updater.state.isDownloading">Downloading... ({{ downloadPercentage }}%)</span>
        <span v-else-if="updater.state.isInstalling">Installing...</span>
        <span v-else>Download & Install</span>
      </button>
      <p v-if="updater.state.error" class="error-message">{{ updater.state.error }}</p>
    </div>
    <p v-else-if="!updater.state.isChecking && !updater.state.error">Your app is up-to-date.</p>
    <p v-if="updater.state.error" class="error-message">{{ updater.state.error }}</p>
    <button @click="updater.checkForUpdate()" :disabled="updater.state.isChecking">
      Check for Updates
    </button>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, computed } from 'vue';
import { useUpdater } from '../composables/useUpdater';

const updater = useUpdater();

const downloadPercentage = computed(() => {
  if (!updater.state.contentLength || !updater.state.downloadProgress) return 0;
  return Math.round((updater.state.downloadProgress / updater.state.contentLength) * 100);
});

onMounted(() => {
  updater.checkForUpdate();
});

onUnmounted(() => {
  updater.cleanup();
});
</script>

<style scoped>
.updater-container {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(12px) saturate(150%);
  -webkit-backdrop-filter: blur(12px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  padding: 1rem !important;
  margin-top: 1rem !important;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.updater-container p {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin-bottom: 0.75rem !important;
  line-height: 1.4;
}

.updater-container button {
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
  font-size: 0.9rem;
  margin-bottom: 0.5rem !important;
}

.updater-container button:hover:not(:disabled) {
  background: rgba(253, 230, 138, 0.15);
  border-color: rgba(253, 230, 138, 0.5);
  color: var(--accent-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.updater-container button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-message {
  color: #ef4444 !important;
  background-color: rgba(239, 68, 68, 0.1);
  border-left: 3px solid #ef4444;
  padding: 0.5rem !important;
  border-radius: 4px;
  font-size: 0.85rem;
  margin-top: 0.5rem !important;
}

/* Success message styling for up-to-date status */
.updater-container p:not(.error-message) {
  position: relative;
}

/* Add subtle glow effect for version info */
.updater-container div p:first-child {
  color: var(--accent-color);
  font-weight: 500;
}

/* Style the parent h4 that contains the title */
:global(.setting-item h4) {
  color: var(--text-secondary) !important;
  font-size: 1rem !important;
  margin-bottom: 0.75rem !important;
  font-weight: 500 !important;
}</style>
