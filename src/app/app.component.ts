import { Component, OnInit } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterOutlet } from "@angular/router";

import { ButtonModule } from "primeng/button";

import { appConfigDir } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";

@Component({
  standalone: true,
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.scss",

  imports: [CommonModule, RouterOutlet, ButtonModule],
})
export class AppComponent implements OnInit {
  async mounted() {
    const config_dir = await appConfigDir();
    const timestamp = new Date().getTime();

    //await listen("theme-changed", () => {
    const element = document.getElementById(
      "style-injector",
    ) as HTMLLinkElement;

    element.href =
      convertFileSrc(config_dir + "style/hyperpanel.css") +
      "?timestamp=" +
      timestamp;
  }

  async ngOnInit() {
    await this.mounted();
  }
}
