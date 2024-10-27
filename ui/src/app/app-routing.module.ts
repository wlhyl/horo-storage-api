import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { Path } from './enum/path';
import { authGuard } from './guard/auth/auth.guard';

const routes: Routes = [
  {
    path: Path.Root,
    children: [
      {
        path: Path.Login,
        loadChildren: () =>
          import('./login/login.module').then((m) => m.LoginModule),
      },

      {
        path: Path.Resource,
        loadChildren: () =>
          import('./resources/resources.module').then((m) => m.ResourcesModule),

        canMatch: [authGuard],
      },

      { path: '**', redirectTo: Path.Resource },
    ],
  },
  { path: '**', redirectTo: Path.Root },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
