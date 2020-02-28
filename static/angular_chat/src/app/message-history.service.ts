import { Injectable } from '@angular/core';
import { MessageEntry } from './message-entry';
import { Subject, Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})

export class MessageHistoryService {
  messageHistory: MessageEntry[];
  messageHistoryKey = 'messageHistory';
  userIdKey = 'userId';

  constructor() {
    if(localStorage.getItem(this.messageHistoryKey) != null) {
      this.messageHistory = JSON.parse(localStorage.getItem(this.messageHistoryKey));
    } else {
      this.messageHistory = [];
    }
  }

  saveMessage(me: MessageEntry) {
    this.messageHistory.push(me)
    localStorage.setItem(this.messageHistoryKey, JSON.stringify(this.messageHistory));
    return me;
  }

  saveUserId(id: string) {
    localStorage.setItem(this.userIdKey, id);
  }

  isOwnMessage(id: string) {
    return localStorage.getItem(this.userIdKey) == id;
  }

  clear() {
    localStorage.removeItem(this.messageHistoryKey);
  }

}
