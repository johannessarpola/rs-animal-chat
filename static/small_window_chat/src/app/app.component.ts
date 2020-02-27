import { Component } from '@angular/core';
import {ChatService} from "./chat.service";
import { MessageHistoryService } from './message-history.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'swc';
  constructor(private cs: ChatService, private ms:MessageHistoryService) {}
  ngOnInit(): void {
    this.cs.connect('ws://127.0.0.1:7777/ws');
  }
  send(msg: string) {
    this.cs.send(msg);
  }
  clearHistory() {
    this.ms.clear();
  }
}
