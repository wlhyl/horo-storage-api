import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { Router } from '@angular/router';
import { Location } from '@angular/common';
import { AuthService } from '../services/auth/auth.service';
import { Path } from '../enum/path';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss',
})
export class LoginComponent implements OnInit {
  title = '登录';

  name = '';
  password = '';
  nameError: string | null = null;
  passwordError: string | null = null;
  authError: string | null = null;
  private canGoBack: boolean;

  constructor(
    private titleService: Title,
    private readonly router: Router,
    private readonly location: Location,
    private auth: AuthService
  ) {
    // 返回上一个url的参考连接；
    // https://stackoverflow.com/questions/35446955/how-to-go-back-last-page
    this.canGoBack = !!this.router.getCurrentNavigation()?.previousNavigation;
  }
  ngOnInit(): void {
    this.titleService.setTitle(this.title);
  }

  login() {
    this.nameError = null;
    this.passwordError = null;
    if (this.name.length === 0) this.nameError = '请输入用户名';
    // if (this.account.length >= 10) this.accountError = '不能大于10个字符';
    if (this.password.length === 0) this.passwordError = '请输入密码';
    // if (this.password.length >= 10) this.passwordError = '不能大于10个字符';
    this.auth.auth(this.name, this.password).subscribe({
      next: () => {
        if (this.canGoBack) this.location.back();
        else this.router.navigateByUrl(`${Path.Root}/${Path.Resource}`);
      },
      error: (error) => (this.authError = '登录失败'),
    });
  }
}
