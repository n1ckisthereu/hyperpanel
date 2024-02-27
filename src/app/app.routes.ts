import { Routes } from "@angular/router";
import { LayoutComponent } from "./core/components/layout/layout.component";
import { GeneralComponent } from "./core/modules/general/general.component";
import { AudioComponent } from "./core/modules/audio/audio.component";
import { NetworkComponent } from "./core/modules/network/network.component";
import { ThemeComponent } from "./core/modules/theme/theme.component";

export const routes: Routes = [
  {
    path: "",
    component: LayoutComponent,
    children: [
      { path: "", component: GeneralComponent },
      { path: "audio", component: AudioComponent },
      { path: "network", component: NetworkComponent },
      { path: "theme", component: ThemeComponent },
    ],
  },
];
