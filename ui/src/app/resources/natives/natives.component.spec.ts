import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NativesComponent } from './natives.component';

describe('NativesComponent', () => {
  let component: NativesComponent;
  let fixture: ComponentFixture<NativesComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [NativesComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(NativesComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
