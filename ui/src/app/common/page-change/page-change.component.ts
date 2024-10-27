import { Component, EventEmitter, Input, Output } from '@angular/core';

@Component({
  selector: 'app-page-change',
  templateUrl: './page-change.component.html',
  styleUrl: './page-change.component.scss',
})
export class PageChangeComponent {
  @Input()
  public total = 0;
  @Input()
  public page = 0;

  @Output()
  public pageChange = new EventEmitter<number>();

  // 翻页参数，start：开始页籹，end:结束页数，per: 一次显示最大页数
  // start从1开始
  get start(): number {
    return Math.floor(this.page / this.per) * this.per + 1;
  }
  get end(): number {
    let end = this.start + this.per - 1;

    if (end > this.total) end = this.total;
    return end;
  }
  @Input()
  per = 5;

  previous(): void {
    // page最少到0
    if (this.page <= 0) return;
    this.page -= 1;
    this.pageChange.emit(this.page);
  }

  next(): void {
    // page 最多到total-1
    if (this.page >= this.total - 1) return;
    this.page += 1;
    this.pageChange.emit(this.page);
  }

  toPage(n: number): void {
    // 0<=page<total
    if (n >= this.total || n < 0) return;
    if (this.page === n) return;
    this.page = n;

    this.pageChange.emit(this.page);
  }
}
