import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { environment } from '../../../environments/environment';
import { UpdateUserRequest } from '../../interfaces/user';
import { AuthService } from '../auth/auth.service';
import { Native, NativeRequest } from '../../interfaces/native';
import { PageResponser } from '../../interfaces/page';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  private readonly url = `${environment.base_url}`;
  private readonly http_options = { 'Content-Type': 'application/json' };

  constructor(private http: HttpClient, private user: AuthService) {}

  // 更新新user
  updateUser(user: UpdateUserRequest): Observable<void> {
    return this.http.put<void>(`${this.url}/user`, user, {
      headers: { ...this.http_options, token: this.user.token },
    });
  }

  // 获取native
  getNatives(
    page: number,
    size: number
  ): Observable<PageResponser<Array<Native>>> {
    return this.http.get<PageResponser<Array<Native>>>(
      `${this.url}/natives?page=${page}&size=${size}`,
      {
        headers: { ...this.http_options, token: this.user.token },
      }
    );
  }

  // 新增native
  addNative(native: NativeRequest): Observable<Native> {
    return this.http.post<Native>(`${this.url}/natives`, native, {
      headers: { ...this.http_options, token: this.user.token },
    });
  }

  // 更新native
  updateNative(id: number, native: NativeRequest): Observable<void> {
    return this.http.put<void>(`${this.url}/natives/${id}`, native, {
      headers: { ...this.http_options, token: this.user.token },
    });
  }

  // 删除native
  deleteNative(id: number): Observable<void> {
    return this.http.delete<void>(`${this.url}/natives/${id}`, {
      headers: { ...this.http_options, token: this.user.token },
    });
  }
}
