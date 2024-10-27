import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PageChangeComponent } from './page-change.component';

describe('PageChangeComponent', () => {
  let component: PageChangeComponent;
  let fixture: ComponentFixture<PageChangeComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [PageChangeComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PageChangeComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
