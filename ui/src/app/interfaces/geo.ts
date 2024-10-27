export interface Geo {
  id: number;
  name: string;
  east: boolean;
  long_d: number;
  long_m: number;
  long_s: number;
  north: boolean;
  lat_d: number;
  lat_m: number;
  lat_s: number;
}

export interface GeoRequest {
  // 城市名
  name: string;
  // 东:+，西:-
  east: boolean;
  // 地理经度
  long_d: number;
  long_m: number;
  long_s: number;

  // 北:+, 南:-
  north: boolean;

  // 地理纬度
  lat_d: number;
  lat_m: number;
  lat_s: number;
}
