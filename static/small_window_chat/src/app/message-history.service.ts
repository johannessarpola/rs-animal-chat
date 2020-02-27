import { Injectable } from '@angular/core';
import { MessageEntry } from './MessageEntry';
import { Subject, Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})

export class MessageHistoryService {
  messageHistory: MessageEntry[];
  key = 'messageHistory';

  constructor() {
    if(localStorage.getItem(this.key) != null) {
      this.messageHistory = JSON.parse(localStorage.getItem(this.key));
    } else {
      this.messageHistory = [];
    }
  }

  save(me: MessageEntry) {
    this.messageHistory.push(me)
    localStorage.setItem(this.key, JSON.stringify(this.messageHistory));
    return me;
  }

  clear() {
    localStorage.removeItem(this.key);
  }

}
