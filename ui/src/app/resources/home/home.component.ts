import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { AuthService } from '../../services/auth/auth.service';
import { Title } from '@angular/platform-browser';
import { Path as RootPath } from '../../enum/path';
import { Path } from '../enum/path';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss',
})
export class HomeComponent implements OnInit {
  title = '例库';
  path = Path;
  active = this.path.Native;

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    public user: AuthService,
    private titleService: Title
  ) {}

  ngOnInit(): void {
    this.titleService.setTitle(this.title);

    // 因为home组件对应的path是'',因此this.route.snapshot.url是空数组
    // 即使访问/user，this.route.snapshot.url也是空数组，
    // 因为ActivatedRoute是当前激活的路由，此组件对应的路由是""，非当前激活路由
    // 访问/user，user组件中的ActivatedRoute可以获取到 "user"这个path
    const currentPath = this.router.url.split('/').pop();
    if (currentPath === Path.User) {
      this.active = Path.User;
    } else if (currentPath === Path.Native) {
      // 这段赋值可以不用写，增加这个分支判断是为了去掉多余的路由跳转
      this.active = Path.Native;
    } else {
      this.router.navigate([Path.Native], { relativeTo: this.route });
    }
  }

  login() {
    this.router.navigateByUrl(`${RootPath.Root}/${RootPath.Login}`);
  }

  logout() {
    this.user.deleteToken();
  }
}
