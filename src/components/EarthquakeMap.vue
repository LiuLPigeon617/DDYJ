<template>
  <div class="map-view">
    <div class="map-placeholder">
      <svg viewBox="0 0 375 500" class="map-svg" xmlns="http://www.w3.org/2000/svg">
        <!-- Simplified world map background -->
        <rect width="375" height="500" fill="#0f1117" />
        <!-- Grid lines -->
        <g stroke="#1e2230" stroke-width="0.5" fill="none">
          <line x1="0" y1="100" x2="375" y2="100" />
          <line x1="0" y1="200" x2="375" y2="200" />
          <line x1="0" y1="250" x2="375" y2="250" />
          <line x1="0" y1="300" x2="375" y2="300" />
          <line x1="0" y1="400" x2="375" y2="400" />
          <line x1="93" y1="0" x2="93" y2="500" />
          <line x1="187" y1="0" x2="187" y2="500" />
          <line x1="281" y1="0" x2="281" y2="500" />
        </g>
        <!-- Equator -->
        <line x1="0" y1="250" x2="375" y2="250" stroke="#2d3344" stroke-width="1" stroke-dasharray="4,4" />

        <!-- Earthquake markers -->
        <g v-for="(eq, i) in visibleEarthquakes" :key="eq.id">
          <circle
            :cx="lonToX(eq.longitude)"
            :cy="latToY(eq.latitude)"
            :r="getRadius(eq.magnitude)"
            :class="'map-marker mag-' + store.magClass(eq.magnitude)"
            fill-opacity="0.3"
          >
            <animate attributeName="r" :values="`${getRadius(eq.magnitude)};${getRadius(eq.magnitude) * 2};${getRadius(eq.magnitude)}`" dur="2s" repeatCount="indefinite" />
          </circle>
          <circle
            :cx="lonToX(eq.longitude)"
            :cy="latToY(eq.latitude)"
            :r="getRadius(eq.magnitude) * 0.6"
            :class="'map-marker mag-' + store.magClass(eq.magnitude)"
          />
          <text
            v-if="eq.magnitude >= 5"
            :x="lonToX(eq.longitude) + 8"
            :y="latToY(eq.latitude) - 8"
            class="map-label"
            fill="#e4e7ef"
            font-size="10"
          >M{{ eq.magnitude.toFixed(1) }}</text>
        </g>
      </svg>

      <div class="map-overlay-top">
        <div class="map-legend">
          <div class="legend-item"><span class="legend-dot mag-low"></span>M&lt;3</div>
          <div class="legend-item"><span class="legend-dot mag-mid"></span>3-5</div>
          <div class="legend-item"><span class="legend-dot mag-high"></span>5-7</div>
          <div class="legend-item"><span class="legend-dot mag-extreme"></span>7+</div>
        </div>
      </div>

      <div class="map-overlay-bottom">
        <div class="map-info-card" v-if="selectedEq">
          <div class="info-mag" :class="'mag-' + store.magClass(selectedEq.magnitude)">
            M{{ selectedEq.magnitude.toFixed(1) }}
          </div>
          <div class="info-text">
            <div class="info-location">{{ selectedEq.location }}</div>
            <div class="info-meta">
              深度 {{ selectedEq.depth }}km · {{ store.formatTime(selectedEq.time) }}
            </div>
          </div>
        </div>
        <div class="map-count">显示 {{ visibleEarthquakes.length }} / {{ store.earthquakes.length }} 个地震</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useEarthquakeStore } from "../stores/earthquake";
import type { Earthquake } from "../types/earthquake";

const store = useEarthquakeStore();
const selectedEq = ref<Earthquake | null>(null);

const visibleEarthquakes = computed(() => {
  return store.sortedEarthquakes.slice(0, 50);
});

function lonToX(lon: number): number {
  return ((lon + 180) / 360) * 375;
}

function latToY(lat: number): number {
  return ((90 - lat) / 180) * 500;
}

function getRadius(mag: number): number {
  return Math.max(3, mag * 2.5);
}
</script>
