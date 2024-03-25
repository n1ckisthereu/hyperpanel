import { Injectable } from "@angular/core";

@Injectable({
  providedIn: "root",
})
export class UtilService {
  constructor() {}

  parseToJson(text: string) {
    return JSON.parse(text);
  }

  decodeBase64(base64: string) {
    return window.atob(base64);
  }

  decodeParse(content: unknown) {
    return this.parseToJson(this.decodeBase64(String(content)));
  }
}
