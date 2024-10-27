import { CanMatchFn, Router } from '@angular/router';
import { AuthService } from '../../services/auth/auth.service';
import { inject } from '@angular/core';
import { Path } from '../../enum/path';

export const authGuard: CanMatchFn = (route, segments) => {
  const user = inject(AuthService);
  const router = inject(Router);
  if (user.isAuth) return true;
  else return router.parseUrl(`${Path.Root}/${Path.Login}`);
};
