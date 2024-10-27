import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { Alert } from '../../interfaces/alert';

@Component({
  selector: 'app-alert',
  templateUrl: './alert.component.html',
  styleUrls: ['./alert.component.scss'],
})
export class AlertComponent implements OnInit {
  @Input()
  alerts: Array<Alert> = [];

  @Output()
  alertsChange = new EventEmitter<Array<Alert>>();

  constructor() {}

  ngOnInit(): void {}

  close(alert: Alert) {
    this.alerts.splice(this.alerts.indexOf(alert), 1);
    this.alertsChange.emit(this.alerts);
  }
}
