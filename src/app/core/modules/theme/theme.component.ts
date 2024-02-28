import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";

import { ImageModule } from "primeng/image";
import { DividerModule } from "primeng/divider";

@Component({
  standalone: true,
  selector: "app-theme",
  templateUrl: "./theme.component.html",
  imports: [CommonModule, ImageModule, DividerModule],
})
export class ThemeComponent {}
