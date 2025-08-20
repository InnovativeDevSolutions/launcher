<template>
  <div class="mod-item" :class="{ 'enabled': mod.enabled }">
    <div class="mod-checkbox">
      <input type="checkbox" :id="`mod-${index}`" :checked="mod.enabled" @change="$emit('toggle')" />
      <label :for="`mod-${index}`" class="checkbox-custom"></label>
    </div>

    <div class="mod-info">
      <div class="mod-header">
        <h4 class="mod-name">{{ mod.name }}</h4>
        <div class="mod-badges">
          <span class="mod-type-badge" :class="mod.mod_type">{{ mod.mod_type.toUpperCase() }}</span>
          <span v-if="mod.extensions && mod.extensions.length > 0" class="extension-badge"
            :title="`${mod.extensions.length} extension(s) detected`">
            EXT
          </span>
        </div>
      </div>

      <div class="mod-details">
        <span v-if="mod.workshop_id" class="mod-detail">
          Workshop ID: {{ mod.workshop_id }}
        </span>

        <span v-if="mod.size" class="mod-detail">
          {{ formatFileSize(mod.size) }}
        </span>

        <!-- <span class="mod-detail mod-path" :title="mod.path">
          {{ mod.path.split('\\').pop() || mod.path.split('/').pop() }}
        </span> -->
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
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
  mod: ModInfo;
  index: number;
}

defineProps<Props>();
defineEmits<{
  'toggle': [];
}>();

function formatFileSize(bytes?: number): string {
  if (!bytes) return "Unknown";
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
}

</script>

<style scoped>
.mod-item {
  background: rgba(0, 0, 0, 0.25);
  backdrop-filter: blur(8px) saturate(150%);
  -webkit-backdrop-filter: blur(8px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px 0;
  padding: 1rem !important;
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.mod-item:hover {
  background: rgba(0, 0, 0, 0.5);
  border-color: rgba(253, 230, 138, 0.4);
  transform: translateY(-2px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25), 0 0 0 1px rgba(253, 230, 138, 0.1);
  backdrop-filter: blur(16px) saturate(175%);
  -webkit-backdrop-filter: blur(16px) saturate(175%);
}

.mod-item.enabled {
  background: rgba(74, 222, 128, 0.04);
  border-color: rgba(74, 222, 128, 0.3);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15), 0 0 0 1px rgba(74, 222, 128, 0.1);
}

.mod-item.enabled:hover {
  background: rgba(74, 222, 128, 0.08);
  border-color: rgba(74, 222, 128, 0.5);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25), 0 0 0 1px rgba(74, 222, 128, 0.2);
}

.mod-checkbox {
  position: relative;
  flex-shrink: 0;
  margin-top: 0.25rem !important;
}

.mod-checkbox input[type="checkbox"] {
  position: absolute;
  opacity: 0;
  width: 20px;
  height: 20px;
  cursor: pointer;
}

.checkbox-custom {
  display: block;
  width: 20px;
  height: 20px;
  background: rgba(255, 255, 255, 0.08);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border: 2px solid rgba(253, 230, 138, 0.3);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.checkbox-custom::after {
  content: "";
  position: absolute;
  left: 6px;
  top: 2px;
  width: 6px;
  height: 10px;
  border: solid #0f0f0f;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.mod-checkbox input[type="checkbox"]:checked+.checkbox-custom {
  background-color: var(--accent-color);
  border-color: var(--accent-color);
}

.mod-checkbox input[type="checkbox"]:checked+.checkbox-custom::after {
  opacity: 1;
}

.mod-checkbox input[type="checkbox"]:hover+.checkbox-custom {
  border-color: rgba(253, 230, 138, 0.6);
}

.mod-info {
  flex: 1;
  min-width: 0;
}

.mod-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.75rem !important;
}

.mod-badges {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}


.mod-name {
  color: var(--text-primary);
  font-size: 1.1rem;
  font-weight: 600;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.mod-type-badge {
  padding: 0.25rem 0.5rem !important;
  border-radius: 6px;
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  flex-shrink: 0;
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
}

.mod-type-badge.workshop {
  background: rgba(59, 130, 246, 0.15);
  color: #60a5fa;
  border: 1px solid rgba(59, 130, 246, 0.3);
}

.mod-type-badge.local {
  background: rgba(34, 197, 94, 0.15);
  color: #4ade80;
  border: 1px solid rgba(34, 197, 94, 0.3);
}

.mod-type-badge.dlc {
  background: rgba(168, 85, 247, 0.15);
  color: #a855f7;
  border: 1px solid rgba(168, 85, 247, 0.3);
}

.mod-item:hover .mod-type-badge {
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.extension-badge {
  padding: 0.25rem 0.5rem !important;
  border-radius: 6px;
  font-size: 0.7rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  flex-shrink: 0;
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  background: rgba(245, 158, 11, 0.15);
  color: #f59e0b;
  border: 1px solid rgba(245, 158, 11, 0.3);
  display: flex;
  align-items: center;
  gap: 0.3rem;
}

.extension-icon {
  width: 12px;
  height: 12px;
}

.mod-item:hover .extension-badge {
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.mod-details {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
}

.mod-detail {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  color: var(--text-secondary);
  font-size: 0.85rem;
}

.detail-icon {
  width: 14px;
  height: 14px;
  opacity: 0.7;
  flex-shrink: 0;
}

.mod-detail.mod-path {
  font-family: 'Courier New', monospace;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
