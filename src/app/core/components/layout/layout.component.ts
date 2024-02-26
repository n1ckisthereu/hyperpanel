import { Component } from "@angular/core";
import { RouterModule } from "@angular/router";

import { ButtonModule } from "primeng/button";
import { SidebarModule } from "primeng/sidebar";

import { NgIconComponent, provideIcons } from "@ng-icons/core";
import { remixSignalWifi2Fill } from "@ng-icons/remixicon";
import { NavLink } from "../../models/navlink";

@Component({
  standalone: true,
  selector: "app-layout",
  templateUrl: "./layout.component.html",
  styleUrl: "./layout.component.scss",
  viewProviders: [provideIcons({ remixSignalWifi2Fill })],
  imports: [RouterModule, SidebarModule, ButtonModule, NgIconComponent],
})
export class LayoutComponent {
  navLinks: NavLink[] = [
    {
      tooltip: "Wi-Fi",
      icon: "remixSignalWifi2Fill",
      route: "wifi",
    },
  ];
}
