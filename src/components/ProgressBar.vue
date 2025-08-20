<template>
  <div v-if="Object.keys(modProgress).length > 0" class="global-progress-bar">
    <div v-for="(progress, modName) in modProgress" :key="modName" class="progress-item">
      <div class="progress-header">
        <span class="mod-name">{{ modName }}</span>
        <span class="progress-percentage">{{ Math.round(progress.percentage) }}%</span>
      </div>
      <div class="progress-bar">
        <div class="progress-fill" :style="{ width: progress.percentage + '%' }"></div>
      </div>
      <div class="progress-message">{{ progress.message }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

interface ModUpdateProgress {
  mod_name: string;
  step: string;
  current: number;
  total: number;
  percentage: number;
  message: string;
}

const modProgress = ref<Record<string, ModUpdateProgress>>({});
let progressUnlisten: UnlistenFn | null = null;

function setupProgressListener() {
  listen<ModUpdateProgress>('mod-update-progress', (event) => {
    const progress = event.payload;
    
    if (progress.step === 'complete') {
      // Remove from progress when complete
      delete modProgress.value[progress.mod_name];
    } else {
      modProgress.value[progress.mod_name] = progress;
    }
  }).then(unlisten => {
    progressUnlisten = unlisten;
  });
}

onMounted(() => {
  setupProgressListener();
});

onUnmounted(() => {
  if (progressUnlisten) {
    progressUnlisten();
  }
});
</script>

<style scoped>
.global-progress-bar {
  position: fixed;
  top: 20px;
  left: 20px;
  z-index: 1000;
  max-width: 350px;
  min-width: 300px;
}

.progress-item {
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border: 1px solid rgba(253, 230, 138, 0.3);
  border-radius: 12px;
  padding: 1rem !important;
  margin-bottom: 0.5rem !important;
  box-shadow: 0 6px 24px rgba(0, 0, 0, 0.6);
  animation: slideInFromLeft 0.5s ease;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem !important;
}

.mod-name {
  color: var(--text-primary);
  font-weight: 600;
  font-size: 0.9rem;
}

.progress-percentage {
  color: var(--accent-color);
  font-weight: 600;
  font-size: 0.875rem;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 0.5rem !important;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-color), #f59e0b);
  border-radius: 4px;
  transition: width 0.3s ease;
  position: relative;
}

.progress-fill::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.2),
    transparent
  );
  animation: shimmer 1.5s infinite;
}

.progress-message {
  color: var(--text-secondary);
  font-size: 0.8rem;
  margin: 0 !important;
  line-height: 1.2;
}

@keyframes slideInFromLeft {
  from {
    transform: translateX(-100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}
</style>
