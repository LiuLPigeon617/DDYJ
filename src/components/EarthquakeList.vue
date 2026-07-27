<template>
  <div class="list-view">
    <StatsOverview />

    <div class="monitor-toggle">
      <div class="toggle-info">
        <span class="toggle-title">实时监控</span>
        <span class="toggle-desc">
          {{ store.isMonitoring ? "正在监控地震活动..." : "监控已停止" }}
        </span>
      </div>
      <button
        class="toggle-btn"
        :class="{ on: store.isMonitoring }"
        @click="store.isMonitoring ? store.stopMonitoring() : store.startMonitoring()"
      >
        <span class="toggle-knob"></span>
      </button>
    </div>

    <div class="list-header">
      <span>最近地震 ({{ store.sortedEarthquakes.length }})</span>
    </div>

    <div class="eq-list">
      <div
        v-for="eq in store.sortedEarthquakes"
        :key="eq.id"
        class="eq-item"
        :class="{ new: store.newIds.has(eq.id) }"
      >
        <div class="eq-magnitude" :class="'mag-' + store.magClass(eq.magnitude)">
          <span class="mag-num">{{ eq.magnitude.toFixed(1) }}</span>
          <span class="mag-unit">M</span>
        </div>
        <div class="eq-info">
          <div class="eq-location">{{ eq.location }}</div>
          <div class="eq-meta">
            <span>{{ store.formatTime(eq.time) }}</span>
            <span>深度 {{ eq.depth }}km</span>
          </div>
        </div>
        <div class="eq-source">{{ eq.source }}</div>
      </div>

      <div v-if="store.sortedEarthquakes.length === 0 && !store.loading" class="empty-state">
        <span class="empty-icon">🌍</span>
        <span>暂无地震数据</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEarthquakeStore } from "../stores/earthquake";
import StatsOverview from "./StatsOverview.vue";

const store = useEarthquakeStore();
</script>
