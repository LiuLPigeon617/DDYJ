<template>
  <div class="app">
    <div class="app-header">
      <div class="header-title">地震预警</div>
      <div class="header-right">
        <span v-if="store.lastUpdate" class="update-time">{{ store.lastUpdate }}</span>
        <button class="refresh-btn" @click="store.fetchEarthquakes" :disabled="store.loading">
          {{ store.loading ? "⏳" : "🔄" }}
        </button>
      </div>
    </div>

    <div class="app-content">
      <EarthquakeList v-if="activeTab === 'list'" />
      <EarthquakeMap v-else-if="activeTab === 'map'" />
      <SettingsPanel v-else-if="activeTab === 'settings'" />
    </div>

    <div class="tab-bar">
      <div
        v-for="tab in tabs"
        :key="tab.id"
        class="tab-item"
        :class="{ active: activeTab === tab.id }"
        @click="activeTab = tab.id"
      >
        <span class="tab-icon">{{ tab.icon }}</span>
        <span class="tab-label">{{ tab.label }}</span>
      </div>
    </div>

    <transition name="fade">
      <div v-if="store.error" class="error-banner">
        <span>{{ store.error }}</span>
        <button @click="store.error = ''">✕</button>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useEarthquakeStore } from "./stores/earthquake";
import EarthquakeList from "./components/EarthquakeList.vue";
import EarthquakeMap from "./components/EarthquakeMap.vue";
import SettingsPanel from "./components/SettingsPanel.vue";

const store = useEarthquakeStore();
const activeTab = ref<"list" | "map" | "settings">("list");

const tabs = [
  { id: "list" as const, icon: "📋", label: "列表" },
  { id: "map" as const, icon: "🗺️", label: "地图" },
  { id: "settings" as const, icon: "⚙️", label: "设置" },
];

onMounted(async () => {
  await store.loadSettings();
  await store.fetchEarthquakes();
  if (store.settings.notificationsEnabled) {
    await store.startMonitoring();
  }
});

onUnmounted(() => {
  store.stopMonitoring();
});
</script>
