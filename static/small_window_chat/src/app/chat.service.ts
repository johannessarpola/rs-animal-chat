import {Injectable} from '@angular/core';
import {WebSocketSubject} from 'rxjs/webSocket';
import {MessageHistoryService} from './message-history.service';
import {MessageEntry} from './MessageEntry';
import {WebSocketSubjectConfig} from "rxjs/internal/observable/dom/WebSocketSubject";


// const socket = new WebSocket("ws://127.0.0.1:7777/ws");
@Injectable({
  providedIn: 'root'
})
export class ChatService {
  address: string;
  socket: WebSocketSubject<any>;
  user_id: string;

  constructor(private msgHistory: MessageHistoryService) {
  }

  connect(address: string) {
    this.address = address;

    let config = {
      url: address,
      openObserver: {
        next(e) {
          console.log("openObserver");
          console.log(e);
        }

      }
    } as WebSocketSubjectConfig<any>;

    this.socket = new WebSocketSubject(config);

    this.socket.subscribe(
      msg => {
        const me = msg as MessageEntry;
        if(me.msg == null && me.timestamp == null) {
          // ID Assigned
          console.log("Assigned id " + me.id);
          this.user_id = me.id;
          this.msgHistory.saveUserId(me.id)
        } else {
          console.log("Received message");
          console.log(me);
          this.msgHistory.saveMessage((me));
        }
      },
      err => console.error(err),
      () => {
      }
    );
  }

  send(msg: string) {
    const timestamp = new Date();
    const entry = {msg: msg, timestamp: timestamp} as MessageEntry;
    // this.msgHistory.save(entry);
    this.socket.next(entry);
  }

}

