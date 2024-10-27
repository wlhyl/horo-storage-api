import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { Alert } from '../../interfaces/alert';
import { AuthService } from '../../services/auth/auth.service';
import { ApiService } from '../../services/api/api.service';
import { AlertKind } from '../../enum/alert';
import { UpdateUserRequest } from '../../interfaces/user';

@Component({
  selector: 'app-user',
  templateUrl: './user.component.html',
  styleUrl: './user.component.scss',
})
export class UserComponent implements OnInit {
  user = {
    id: 0, // this.authenticatedUser.user.id,
    name: '', //this.authenticatedUser.user.name,
  };

  // public user = {
  //   id: this.authenticatedUser.user.id,
  //   name: this.authenticatedUser.user.name,
  // };

  oldPassword = '';
  password = '';
  repeatPassword = '';

  updating = false;

  message: Array<Alert> = [];

  constructor(
    private authenticatedUser: AuthService,
    private api: ApiService,
    private titleService: Title
  ) {
    this.user.id = authenticatedUser.user.id;
    this.user.name = authenticatedUser.user.name;
  }

  ngOnInit(): void {
    this.titleService.setTitle('用户配置');
  }

  updateUser() {
    this.message = [];
    if (this.oldPassword === '') {
      this.message.push({
        kind: AlertKind.DANGER,
        message: '请输入旧密码',
      });
      return;
    }
    if (this.password === '') {
      this.message.push({
        kind: AlertKind.DANGER,
        message: '请输入新密码！',
      });
      return;
    }

    if (this.password !== this.repeatPassword) {
      this.message.push({
        kind: AlertKind.DANGER,
        message: '新密码两次输入不一致！',
      });
      return;
    }

    if (this.password === this.oldPassword) {
      this.message.push({
        kind: AlertKind.DANGER,
        message: '新密码与旧密码相同！',
      });
      return;
    }

    let user: UpdateUserRequest = {
      password: this.password,
      old_password: this.oldPassword,
    };

    this.updating = true;
    this.api
      .updateUser(user)
      .subscribe({
        next: () => {
          const message = '密码修改成功，请重新登录！';
          this.message.push({
            kind: AlertKind.SUCCESS,
            message,
          });
          this.authenticatedUser.deleteToken();
        },
        error: (error) => {
          const msg = error.error.error;
          const message = msg ? msg : '修改用户失败';
          this.message.push({
            kind: AlertKind.DANGER,
            message,
          });
        },
      })
      .add(() => {
        this.updating = false;
        this.oldPassword = '';
        this.password = '';
        this.repeatPassword = '';
      });
  }
}
