import { Injectable } from '@angular/core';
import { environment } from '../../../environments/environment';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { AuthUser } from '../../interfaces/user';
import { jwtDecode } from 'jwt-decode';
import { map } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class AuthService {
  private readonly url = `${environment.base_url}`;
  private readonly http_options = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
  };

  private _user: AuthUser = {
    id: 0,
    name: '',
    exp: 0,
  };

  public get user() {
    return this._user;
  }

  private _token = '';

  constructor(private http: HttpClient) {}

  public get token(): string {
    // token 未过期
    if (this._token !== '' && this._user.exp > Date.now() / 1000.0)
      return this._token;

    // token 过期
    if (this._token !== '' && this._user.exp <= Date.now() / 1000) {
      this._token = '';
      localStorage.removeItem('token');
      return '';
    }

    // 以下是 this._token===""
    const token = localStorage.getItem('token');

    if (token === null) return '';

    // _token === "", localstorage !== null 发生在第一次加载页面或更新了token
    try {
      const user = jwtDecode<AuthUser>(token);

      // 获取当前时间戳
      // const currentTimestamp: number = Date.now();

      if (user.exp <= Date.now() / 1000) {
        // token 过期
        localStorage.removeItem('token');
        return '';
      }

      this._user = user;
      this._token = token;
      return token;
    } catch (error: any) {
      // 正常情况下是不会抛出错误
      console.log(`解析jwt错误：${error}`);
      localStorage.removeItem('token');
      return '';
    }
  }

  // private set token(token: string) {
  //   //  if("") 等同于if(false)
  //   if (token === '') {
  //     localStorage.removeItem('token');
  //     this._token = '';
  //     return;
  //   }

  //   try {
  //     const user = jwtDecode<AuthUser>(token);

  //     // 获取当前时间戳
  //     const currentTimestamp: number = Date.now();

  //     if (user.exp < currentTimestamp) {
  //       // token 过期
  //       return;
  //     }

  //     this.user = user;
  //     this._token = token;
  //     localStorage.setItem('token', token);
  //     return;
  //   } catch (error) {
  //     // 正常情况下是不会抛出错误

  //     return;
  //   }
  // }

  public get isAuth(): boolean {
    // 根据token过期时间判断token是否有效 在get token中已经判断
    return this.token !== '';
  }

  /**
   *
   * @param name 用户名
   * @param password 密码
   * 认证失败抛异常
   */
  public auth(name: string, password: string) {
    return this.http
      .post<{ token: string }>(
        `${this.url}/login`,
        { name, password },
        this.http_options
      )
      .pipe(
        map((v) => {
          this._token = '';
          localStorage.setItem('token', v.token);
          // return  v;
        })
      );
  }

  public deleteToken() {
    this._token = '';
    localStorage.removeItem('token');
  }
}
