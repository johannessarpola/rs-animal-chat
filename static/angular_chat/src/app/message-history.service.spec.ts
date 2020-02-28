import { TestBed } from '@angular/core/testing';

import { MessageHistoryService } from './message-history.service';

describe('MessageHistoryService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: MessageHistoryService = TestBed.get(MessageHistoryService);
    expect(service).toBeTruthy();
  });
});
