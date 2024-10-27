import { AlertKind } from '../enum/alert';
export interface Alert {
  kind: AlertKind;
  message: string;
}
