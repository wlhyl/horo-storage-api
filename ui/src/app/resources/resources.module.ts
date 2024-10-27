import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { ResourcesRoutingModule } from './resources-routing.module';
import { NativesComponent } from './natives/natives.component';
import { UserComponent } from './user/user.component';
import { HomeComponent } from './home/home.component';

// 双向绑定
import { FormsModule } from '@angular/forms';

import { StorageCommonModule } from '../common/storage-common.module';

import {
  NgbDropdownModule,
  NgbNav,
  // NgbNavLink,
  NgbNavLinkButton,
  NgbNavItem,
  NgbTooltipModule,
} from '@ng-bootstrap/ng-bootstrap';

@NgModule({
  declarations: [NativesComponent, UserComponent, HomeComponent],
  imports: [
    CommonModule,
    ResourcesRoutingModule,
    FormsModule,
    StorageCommonModule,
    NgbDropdownModule,
    NgbNav,
    // NgbNavLink,
    NgbNavLinkButton,
    NgbNavItem,
    NgbTooltipModule,
  ],
})
export class ResourcesModule {}
