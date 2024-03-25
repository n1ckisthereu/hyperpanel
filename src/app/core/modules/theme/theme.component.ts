import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";

import { ImageModule } from "primeng/image";
import { DividerModule } from "primeng/divider";
import { ButtonModule } from "primeng/button";

import { remixCheckFill } from "@ng-icons/remixicon";
import { NgIconComponent, provideIcons } from "@ng-icons/core";
import { ThemeService } from "../../services/theme/theme.service";
import { ThemeInfo } from "../../models/theme";
import { convertFileSrc } from "@tauri-apps/api/tauri";

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
export class ThemeComponent implements OnInit {
  currentWallpaper = "";
  pictures: string[] = [];
  color_scheme: string[] = [];

  constructor(private themeService: ThemeService) {}

  ngOnInit() {
    this.themeService.getThemeInfos().subscribe({
      next: (info) => {
        this.color_scheme = info.color_scheme;
        this.currentWallpaper = convertFileSrc(info.current_wallpaper);
        info.pictures.forEach((picture) => {
          this.pictures.push(convertFileSrc(picture));
        });
      },
    });
  }
}
