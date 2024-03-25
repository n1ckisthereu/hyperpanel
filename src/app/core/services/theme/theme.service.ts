import { Injectable } from "@angular/core";

import { invoke } from "@tauri-apps/api/tauri";
import { from } from "rxjs";
import { UtilService } from "../util/util.service";
import { ThemeInfo } from "../../models/theme";

@Injectable({
  providedIn: "root",
})
export class ThemeService {
  pictures = [];
  themeInfo = {} as ThemeInfo;

  constructor(private utilService: UtilService) {}

  getThemeInfos() {
    return from(
      invoke("run", { pname: "hyprc", command: "get_infos" }).then(
        (themeInfo) => {
          return this.utilService.decodeParse(themeInfo) as ThemeInfo;
        },
      ),
    );
  }
}
