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
  padding: 1rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  margin-top: 1rem;
}

.error-message {
  color: red;
}
</style>
