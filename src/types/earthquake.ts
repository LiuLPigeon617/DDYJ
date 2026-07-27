export interface Earthquake {
  id: string;
  magnitude: number;
  depth: number;
  location: string;
  latitude: number;
  longitude: number;
  time: string;
  source: string;
}

export interface MonitorSettings {
  minMagnitude: number;
  pollIntervalSec: number;
  notificationsEnabled: boolean;
  soundEnabled: boolean;
  region: string;
}

export interface EarthquakeStats {
  total: number;
  maxMagnitude: number;
  avgMagnitude: number;
  recent24h: number;
  recent1h: number;
}
