import { Geo, GeoRequest } from './geo';

export interface Native {
  id: number;
  name: string | null;
  sex: boolean;
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  tz: number;
  st: boolean;

  geo: Geo;

  describe: string | null;

  create_date: string;
  last_update_date: string | null;
}

export interface NativeRequest {
  // 姓名
  name: string | null;
  // 性别
  sex: boolean;
  // 年，最小值1900
  year: number;
  // 月
  month: number;
  // 日
  day: number;
  // 时
  hour: number;
  // 分
  minute: number;
  // 秒
  second: number;
  // 出生地时区，东区为正数，西区为负数
  tz: number;
  // 出生时的夏令时，有夏令时：true，无夏令时： false
  st: boolean;

  geo: GeoRequest;

  // 说明文字
  describe: string | null;
}
