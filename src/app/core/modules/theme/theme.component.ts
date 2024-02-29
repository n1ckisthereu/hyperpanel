import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";

import { ImageModule } from "primeng/image";
import { DividerModule } from "primeng/divider";
import { ButtonModule } from "primeng/button";

import { remixCheckFill } from "@ng-icons/remixicon";
import { NgIconComponent, provideIcons } from "@ng-icons/core";

@Component({
  standalone: true,
  selector: "app-theme",
  templateUrl: "./theme.component.html",
  styleUrl: "./theme.component.scss",
  viewProviders: [
    provideIcons({
      remixCheckFill,
    }),
  ],
  imports: [
    CommonModule,
    NgIconComponent,
    ImageModule,
    DividerModule,
    ButtonModule,
  ],
})
export class ThemeComponent {
  currentWallpaper =
    "https://primefaces.org/cdn/primeng/images/galleria/galleria1.jpg";
}
