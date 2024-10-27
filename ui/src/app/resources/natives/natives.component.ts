import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { Alert } from '../../interfaces/alert';
import { PageResponser } from '../../interfaces/page';
import { Native, NativeRequest } from '../../interfaces/native';
import { ApiService } from '../../services/api/api.service';
import { AlertKind } from '../../enum/alert';

@Component({
  selector: 'app-natives',
  templateUrl: './natives.component.html',
  styleUrl: './natives.component.scss',
})
export class NativesComponent implements OnInit {
  page = 0;
  size = 10;
  message: Array<Alert> = [];

  saving = false;
  deleting = 0;

  natives: PageResponser<Array<Native>> = {
    data: [],
    total: 0,
  };

  // 新增/更新native
  native: Native = {
    id: 0,
    name: null,
    sex: true,
    // year: 0,
    // month: 0,
    // day: 0,
    // hour: 0,
    // minute: 0,
    // second: 0,
    // tz: 0,
    // st: false,
    ...this.nowDate(),
    geo: {
      id: 0,
      name: '',
      long_d: 0,
      lat_d: 0,
      east: true,
      long_m: 0,
      long_s: 0,
      north: true,
      lat_m: 0,
      lat_s: 0,
    },
    describe: null,
    create_date: '',
    last_update_date: null,
  };

  zones = [...Array(25)].map((_, i) => i - 12).reverse();

  constructor(private api: ApiService, private titleService: Title) {}
  ngOnInit(): void {
    this.titleService.setTitle('例');
    this.getNatives();
  }

  private async getNatives() {
    this.message = [];
    this.api.getNatives(this.page, this.size).subscribe({
      next: (response) => (this.natives = response),
      error: (error) => {
        let msg = error.error.error;
        let message = '获取native失败！';
        if (msg) message = msg;
        this.message.push({
          kind: AlertKind.DANGER,
          message,
        });
      },
    });
  }

  edit(id: number) {
    let c = this.natives.data.find((c) => c.id === id);
    if (c) {
      // 不能使用 this.native = c ，这种方式是引用复制，
      // 修改this.native也就是在修改this.natives中的值
      // 使用{...c}进行值复制
      // this.native = c;
      this.native = { ...c };
    }
  }
  delete(id: number) {
    this.message = [];
    this.deleting = id;
    this.api
      .deleteNative(id)
      .subscribe({
        next: () => {
          this.getNatives();
        },
        error: (error) => {
          let msg = error.error.error;
          let message = '删除native失败！';
          if (msg) message = msg;
          this.message.push({
            kind: AlertKind.DANGER,
            message,
          });
        },
      })
      .add(() => (this.deleting = 0));
  }

  pageChange(page: number) {
    this.page = page;
    this.getNatives();
  }

  save() {
    this.message = [];
    const nativeRequest: NativeRequest = {
      name: null,
      sex: this.native.sex,
      year: this.native.year,
      month: this.native.month,
      day: this.native.day,
      hour: this.native.hour,
      minute: this.native.minute,
      second: this.native.second,
      tz: this.native.tz,
      st: this.native.st,
      geo: {
        ...this.native.geo,
      },
      describe: null,
    };

    // 校验geo
    //     if (this.native.geo!.long > 180 || this.native.geo!.long < -180) {
    if (nativeRequest.geo.name === '') {
      let message = '请输入城市名';
      this.message.push({
        kind: AlertKind.DANGER,
        message,
      });
      return;
    }
    const long =
      nativeRequest.geo.long_d +
      nativeRequest.geo.long_m +
      nativeRequest.geo.long_s;
    if (long > 180) {
      let message = '-180<=long<=180';
      this.message.push({
        kind: AlertKind.DANGER,
        message,
      });
      return;
    }
    //     if (this.native.geo!.lat > 90 || this.native.geo!.lat < -90) {

    const lat =
      nativeRequest.geo.lat_d +
      nativeRequest.geo.lat_m +
      nativeRequest.geo.lat_s;
    if (lat > 90) {
      let message = '-90<=lat<=90';
      this.message.push({
        kind: AlertKind.DANGER,
        message,
      });
      return;
    }

    // 修正native.name，native.describe
    // this.native.name==='' if(this.native.name)不执行
    if (this.native.name) nativeRequest.name = this.native.name;
    if (this.native.describe) nativeRequest.describe = this.native.describe;

    this.saving = true;
    if (this.native.id === 0) {
      this.api
        .addNative(nativeRequest)
        .subscribe({
          next: (response) => {
            this.getNatives();
          },
          error: (error) => {
            let msg = error.error.error;
            let message = '新增native失败！';
            if (msg) message = msg;
            this.message.push({
              kind: AlertKind.DANGER,
              message,
            });
          },
        })
        .add(() => {
          this.cancel();
          this.saving = false;
        });
    } else {
      this.api
        .updateNative(this.native.id, nativeRequest)
        .subscribe({
          next: () => {
            this.getNatives();
          },
          error: (error) => {
            let msg = error.error.error;
            let message = '更新native失败！';
            if (msg) message = msg;
            this.message.push({
              kind: AlertKind.DANGER,
              message,
            });
          },
        })
        .add(() => {
          this.cancel();
          this.saving = false;
        });
    }
  }
  cancel() {
    this.native = {
      id: 0,
      name: null,
      sex: true,
      ...this.nowDate(),
      geo: {
        id: 0,
        name: '',
        long_d: 0,
        lat_d: 0,
        east: true,
        long_m: 0,
        long_s: 0,
        north: true,
        lat_m: 0,
        lat_s: 0,
      },
      describe: null,
      create_date: '',
      last_update_date: null,
    };
  }

  private nowDate() {
    let t = new Date();
    let year = t.getFullYear();
    let month = t.getMonth() + 1;
    let day = t.getDate();
    let hour = t.getHours();
    let minute = t.getMinutes();
    let second = t.getSeconds();

    let st = false;
    // 判断夏令时
    let d1 = new Date(year, 1, 1);
    let tz = d1.getTimezoneOffset() / -60;
    // let d2 = new Date(this.horo.year,7,1);
    if (t.getTimezoneOffset() != d1.getTimezoneOffset()) {
      st = true;
      tz -= 1;
    }
    return {
      year,
      month,
      day,
      hour,
      minute,
      second,
      tz,
      st,
    };
  }
}
