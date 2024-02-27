import { Component } from "@angular/core";
import { RouterModule } from "@angular/router";

import { ButtonModule } from "primeng/button";
import { SidebarModule } from "primeng/sidebar";
import { NgIconComponent, provideIcons } from "@ng-icons/core";
import {
  remixSignalWifi2Fill,
  remixFilePaper2Fill,
  remixSoundModuleFill,
  remixHomeGearLine,
} from "@ng-icons/remixicon";

import { NavLink } from "../../models/navlink";

@Component({
  standalone: true,
  selector: "app-layout",
  templateUrl: "./layout.component.html",
  styleUrl: "./layout.component.scss",
  viewProviders: [
    provideIcons({
      remixSignalWifi2Fill,
      remixFilePaper2Fill,
      remixSoundModuleFill,
      remixHomeGearLine,
    }),
  ],
  imports: [RouterModule, SidebarModule, ButtonModule, NgIconComponent],
})
export class LayoutComponent {
  navLinks: NavLink[] = [
    {
      tooltip: "General",
      icon: "remixHomeGearLine",
      route: "/",
      exact: true,
    },
    {
      tooltip: "Network",
      icon: "remixSignalWifi2Fill",
      route: "network",
    },
    {
      tooltip: "Theme",
      icon: "remixFilePaper2Fill",
      route: "theme",
    },
    {
      tooltip: "Audio",
      icon: "remixSoundModuleFill",
      route: "audio",
    },
  ];
}
