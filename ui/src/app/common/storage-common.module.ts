import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

// 双向绑定
import { FormsModule } from '@angular/forms';

import {
  // NgbToastModule,
  // NgbDropdownModule,
  NgbAlertModule,
  // NgbProgressbarModule,
} from '@ng-bootstrap/ng-bootstrap';

// import { NgbTooltipModule } from '@ng-bootstrap/ng-bootstrap';

import { AlertComponent } from './alert/alert.component';
import { PageChangeComponent } from './page-change/page-change.component';

@NgModule({
  declarations: [AlertComponent, PageChangeComponent],
  imports: [
    CommonModule,
    FormsModule,
    // NgbToastModule,
    // NgbDropdownModule,
    NgbAlertModule,
    // NgbProgressbarModule,
    // NgbTooltipModule,
  ],
  exports: [
    // FormsModule,
    // HeaderComponent,
    // ToastComponent,
    AlertComponent,
    PageChangeComponent,
    // ChipsComponent,
    // UploadComponent,
    // NgbTooltipModule,
  ],
})
export class StorageCommonModule {}
