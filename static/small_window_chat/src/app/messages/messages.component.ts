import { Component, OnInit } from '@angular/core';
import { MessageHistoryService } from '../message-history.service';
import { MessageEntry } from '../MessageEntry';

@Component({
  selector: 'app-messages',
  templateUrl: './messages.component.html',
  styleUrls: ['./messages.component.css']
})
export class MessagesComponent implements OnInit {
  messageHistory: MessageEntry[];
  constructor(private historyService: MessageHistoryService ) {
    this.messageHistory = historyService.messageHistory;
  }

  ngOnInit() {

  }

}
