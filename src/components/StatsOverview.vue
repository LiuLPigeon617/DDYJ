<template>
  <div class="stats-row">
    <div class="stat-card">
      <span class="stat-value" :class="'mag-' + store.magClass(store.stats.maxMagnitude)">
        {{ store.stats.maxMagnitude.toFixed(1) }}
      </span>
      <span class="stat-label">最大震级</span>
    </div>
    <div class="stat-card">
      <span class="stat-value">{{ store.stats.recent1h }}</span>
      <span class="stat-label">1小时内</span>
    </div>
    <div class="stat-card">
      <span class="stat-value">{{ store.stats.recent24h }}</span>
      <span class="stat-label">24小时内</span>
    </div>
    <div class="stat-card">
      <span class="stat-value">{{ store.stats.total }}</span>
      <span class="stat-label">总计</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEarthquakeStore } from "../stores/earthquake";
const store = useEarthquakeStore();
</script>
