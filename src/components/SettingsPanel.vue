<template>
  <div class="settings-view">
    <div class="settings-group">
      <div class="group-title">预警设置</div>

      <div class="setting-row">
        <div class="setting-label-group">
          <span class="setting-label">通知推送</span>
          <span class="setting-desc">检测到新地震时推送通知</span>
        </div>
        <button
          class="toggle-btn"
          :class="{ on: local.notificationsEnabled }"
          @click="local.notificationsEnabled = !local.notificationsEnabled"
        >
          <span class="toggle-knob"></span>
        </button>
      </div>

      <div class="setting-row">
        <div class="setting-label-group">
          <span class="setting-label">声音提醒</span>
          <span class="setting-desc">高震级地震时播放警报声</span>
        </div>
        <button
          class="toggle-btn"
          :class="{ on: local.soundEnabled }"
          @click="local.soundEnabled = !local.soundEnabled"
        >
          <span class="toggle-knob"></span>
        </button>
      </div>
    </div>

    <div class="settings-group">
      <div class="group-title">震级阈值</div>

      <div class="setting-row column">
        <div class="setting-label-group">
          <span class="setting-label">最小震级</span>
          <span class="setting-desc">仅显示不低于此震级的地震</span>
        </div>
        <div class="slider-row">
          <input
            type="range"
            class="slider"
            min="0"
            max="8"
            step="0.5"
            v-model.number="local.minMagnitude"
          />
          <span class="slider-value">M{{ local.minMagnitude.toFixed(1) }}</span>
        </div>
      </div>

      <div class="setting-row column">
        <div class="setting-label-group">
          <span class="setting-label">检查间隔</span>
          <span class="setting-desc">每隔多久检查一次新地震数据</span>
        </div>
        <div class="slider-row">
          <input
            type="range"
            class="slider"
            min="30"
            max="300"
            step="30"
            v-model.number="local.pollIntervalSec"
          />
          <span class="slider-value">{{ local.pollIntervalSec }}秒</span>
        </div>
      </div>
    </div>

    <div class="settings-group">
      <div class="group-title">数据源</div>
      <div class="data-source-info">
        <div class="source-item">
          <span class="source-name">USGS</span>
          <span class="source-desc">美国地质调查局 — 全球实时数据</span>
          <span class="source-status online">在线</span>
        </div>
        <div class="source-item">
          <span class="source-name">CENC</span>
          <span class="source-desc">中国地震台网中心 — 中国区域数据</span>
          <span class="source-status online">在线</span>
        </div>
      </div>
    </div>

    <div class="settings-actions">
      <button class="btn-primary" @click="save">保存设置</button>
    </div>

    <div class="app-info">
      <div class="app-name">DDYJ 地震预警</div>
      <div class="app-version">v0.1.0 · Tauri 2 + Rust + Vue 3</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, onMounted } from "vue";
import { useEarthquakeStore } from "../stores/earthquake";
import type { MonitorSettings } from "../types/earthquake";

const store = useEarthquakeStore();

const local = reactive<MonitorSettings>({
  minMagnitude: 2.5,
  pollIntervalSec: 60,
  notificationsEnabled: true,
  soundEnabled: true,
  region: "all",
});

onMounted(() => {
  Object.assign(local, store.settings);
});

async function save() {
  await store.saveSettings({ ...local });
  await store.fetchEarthquakes();
}
</script>
