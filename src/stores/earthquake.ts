import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { Earthquake, MonitorSettings, EarthquakeStats } from "../types/earthquake";

export const useEarthquakeStore = defineStore("earthquake", () => {
  const earthquakes = ref<Earthquake[]>([]);
  const settings = ref<MonitorSettings>({
    minMagnitude: 2.5,
    pollIntervalSec: 60,
    notificationsEnabled: true,
    soundEnabled: true,
    region: "all",
  });
  const isMonitoring = ref(false);
  const lastUpdate = ref<string>("");
  const loading = ref(false);
  const error = ref<string>("");
  const newIds = ref<Set<string>>(new Set());

  let unlistenFn: UnlistenFn | null = null;

  const stats = computed<EarthquakeStats>(() => {
    if (earthquakes.value.length === 0) {
      return { total: 0, maxMagnitude: 0, avgMagnitude: 0, recent24h: 0, recent1h: 0 };
    }
    const now = Date.now();
    const hourAgo = now - 60 * 60 * 1000;
    const dayAgo = now - 24 * 60 * 60 * 1000;
    const recent1h = earthquakes.value.filter(
      (e) => new Date(e.time).getTime() > hourAgo
    ).length;
    const recent24h = earthquakes.value.filter(
      (e) => new Date(e.time).getTime() > dayAgo
    ).length;
    const maxMag = Math.max(...earthquakes.value.map((e) => e.magnitude));
    const avgMag =
      earthquakes.value.reduce((sum, e) => sum + e.magnitude, 0) /
      earthquakes.value.length;
    return {
      total: earthquakes.value.length,
      maxMagnitude: maxMag,
      avgMagnitude: Math.round(avgMag * 100) / 100,
      recent24h,
      recent1h,
    };
  });

  const sortedEarthquakes = computed(() => {
    return [...earthquakes.value].sort(
      (a, b) => new Date(b.time).getTime() - new Date(a.time).getTime()
    );
  });

  function magClass(mag: number): string {
    if (mag >= 7) return "extreme";
    if (mag >= 5) return "high";
    if (mag >= 3) return "mid";
    return "low";
  }

  function formatTime(time: string): string {
    const d = new Date(time);
    const now = new Date();
    const diff = now.getTime() - d.getTime();
    const mins = Math.floor(diff / 60000);
    const hours = Math.floor(diff / 3600000);
    const days = Math.floor(diff / 86400000);
    if (mins < 1) return "刚刚";
    if (mins < 60) return `${mins}分钟前`;
    if (hours < 24) return `${hours}小时前`;
    if (days < 7) return `${days}天前`;
    return d.toLocaleString("zh-CN", { month: "2-digit", day: "2-digit", hour: "2-digit", minute: "2-digit" });
  }

  async function fetchEarthquakes() {
    loading.value = true;
    error.value = "";
    try {
      const data = await invoke<Earthquake[]>("get_recent_earthquakes");
      earthquakes.value = data;
      lastUpdate.value = new Date().toLocaleTimeString("zh-CN");
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "获取数据失败";
    } finally {
      loading.value = false;
    }
  }

  async function startMonitoring() {
    try {
      await invoke("start_monitoring");
      isMonitoring.value = true;
      if (!unlistenFn) {
        unlistenFn = await listen<Earthquake>("new-earthquake", (event) => {
          const eq = event.payload;
          if (!earthquakes.value.find((e) => e.id === eq.id)) {
            earthquakes.value.unshift(eq);
            newIds.value.add(eq.id);
            setTimeout(() => newIds.value.delete(eq.id), 3000);
          }
        });
      }
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "启动监控失败";
    }
  }

  async function stopMonitoring() {
    try {
      await invoke("stop_monitoring");
      isMonitoring.value = false;
      if (unlistenFn) {
        unlistenFn();
        unlistenFn = null;
      }
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "停止监控失败";
    }
  }

  async function loadSettings() {
    try {
      settings.value = await invoke<MonitorSettings>("get_settings");
    } catch {
      // use defaults
    }
  }

  async function saveSettings(newSettings: MonitorSettings) {
    try {
      await invoke("update_settings", { settings: newSettings });
      settings.value = newSettings;
    } catch (e: any) {
      error.value = typeof e === "string" ? e : e.message || "保存设置失败";
    }
  }

  return {
    earthquakes,
    settings,
    isMonitoring,
    lastUpdate,
    loading,
    error,
    newIds,
    stats,
    sortedEarthquakes,
    magClass,
    formatTime,
    fetchEarthquakes,
    startMonitoring,
    stopMonitoring,
    loadSettings,
    saveSettings,
  };
});
