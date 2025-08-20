import { reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface UpdateMetadata {
  version: string;
  currentVersion: string;
  notes?: string;
  date?: string;
}

export interface UpdateEvent {
  chunkLength?: number;
  contentLength?: number;
}

export interface UpdateState {
  isChecking: boolean;
  isDownloading: boolean;
  isInstalling: boolean;
  updateAvailable: boolean;
  updateMetadata: UpdateMetadata | null;
  downloadProgress: number;
  contentLength?: number;
  error: string | null;
}

export function useUpdater() {
  const state = reactive<UpdateState>({
    isChecking: false,
    isDownloading: false,
    isInstalling: false,
    updateAvailable: false,
    updateMetadata: null,
    downloadProgress: 0,
    contentLength: undefined,
    error: null,
  });

  // Listen for download progress events
  let progressUnlisten: (() => void) | null = null;
  let finishedUnlisten: (() => void) | null = null;
  let installUnlisten: (() => void) | null = null;

  const setupEventListeners = async () => {
    if (progressUnlisten) return; // Already set up

    try {
      progressUnlisten = await listen<UpdateEvent>('updater://download-progress', (event) => {
        if (event.payload.chunkLength) {
          state.downloadProgress += event.payload.chunkLength;
        }
      });

      finishedUnlisten = await listen('updater://download-finished', () => {
        state.isDownloading = false;
        state.downloadProgress = state.contentLength || 100;
      });

      installUnlisten = await listen('updater://install-finished', () => {
        state.isInstalling = false;
      });
    } catch (error) {
      console.warn('Failed to setup update event listeners:', error);
    }
  };

  const checkForUpdate = async (): Promise<UpdateMetadata | null> => {
    state.isChecking = true;
    state.error = null;

    try {
      await setupEventListeners();
      
      const updateMetadata = await invoke<UpdateMetadata | null>('check_for_update');
      
      if (updateMetadata) {
        state.updateAvailable = true;
        state.updateMetadata = updateMetadata;
      } else {
        state.updateAvailable = false;
        state.updateMetadata = null;
      }

      return updateMetadata;
    } catch (error) {
      state.error = `Failed to check for updates: ${error}`;
      console.error('Update check failed:', error);
      return null;
    } finally {
      state.isChecking = false;
    }
  };



  const downloadAndInstallUpdate = async (): Promise<boolean> => {
    if (!state.updateAvailable) {
      state.error = 'No update available to install';
      return false;
    }

    state.isDownloading = true;
    state.downloadProgress = 0;
    state.error = null;

    try {
      await invoke('download_and_install_update');
      return true;
    } catch (error) {
      state.error = `Failed to download and install update: ${error}`;
      console.error('Update download and installation failed:', error);
      return false;
    } finally {
      state.isDownloading = false;
      state.isInstalling = false;
    }
  };

  const cleanup = () => {
    if (progressUnlisten) {
      progressUnlisten();
      progressUnlisten = null;
    }
    if (finishedUnlisten) {
      finishedUnlisten();
      finishedUnlisten = null;
    }
    if (installUnlisten) {
      installUnlisten();
      installUnlisten = null;
    }
  };

  return {
    state,
    checkForUpdate,
    downloadAndInstallUpdate,
    cleanup,
  };
}
