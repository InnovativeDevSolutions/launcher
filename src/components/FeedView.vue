<template>
  <div class="content-view feed-view">
    <div class="feed-content">
      <div class="feed-items">
        <div class="feed-item">
          <h3>Server Update</h3>
          <p>New content available for download</p>
          <span class="feed-time">2 hours ago</span>
        </div>
        <div class="feed-item">
          <h3>Community Event</h3>
          <p>Join the weekend tournament starting Friday</p>
          <span class="feed-time">1 day ago</span>
        </div>
        <div class="feed-item">
          <h3>Patch Notes</h3>
          <p>Version 1.2.0 includes bug fixes and improvements</p>
          <span class="feed-time">3 days ago</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// This component is purely presentational for now
</script>

<style scoped>
.content-view {
  height: 100%;
  padding: 2rem !important;
  display: flex;
  align-items: flex-start;
  justify-content: center;
}

.feed-content {
  max-width: 800px;
  width: 100%;
}

.feed-items {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-height: calc(100vh - 175px);
  overflow-y: auto;
  scrollbar-width: none;
  /* Firefox */
  -ms-overflow-style: none;
  /* IE and Edge */
  padding-right: 0.5rem;
  padding-top: 0.125rem !important;
}

.feed-items::-webkit-scrollbar {
  display: none;
  /* Chrome, Safari and Opera */
}

.feed-item {
  background: rgba(0, 0, 0, 0.25);
  backdrop-filter: blur(8px) saturate(150%);
  -webkit-backdrop-filter: blur(8px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px 0;
  padding: 1.5rem !important;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.feed-item:hover {
  background: rgba(0, 0, 0, 0.5);
  border-color: rgba(253, 230, 138, 0.4);
  transform: translateY(-2px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25), 0 0 0 1px rgba(253, 230, 138, 0.1);
  backdrop-filter: blur(16px) saturate(175%);
  -webkit-backdrop-filter: blur(16px) saturate(175%);
}

.feed-item h3 {
  color: var(--accent-color);
  margin-bottom: 0.5rem !important;
  font-size: 1.2rem;
}

.feed-item p {
  color: var(--text-secondary);
  margin-bottom: 1rem !important;
  line-height: 1.5;
}

.feed-time {
  color: rgba(246, 246, 246, 0.5);
  font-size: 0.9rem;
}
</style>
