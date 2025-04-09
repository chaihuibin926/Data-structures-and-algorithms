export class Queue<T> {
  private queue: T[] = [];
  private head: number;
  private tail: number;
  size: number;

  constructor(size: number) {
    this.queue = new Array(size)
    this.head = 0;
    this.tail = 0;
    this.size = size;
  }

  expand() {
    this.size = this.size * 2
  }

  enqueue(item: T) {
    if (this.tail === this.size) {
      if (this.head > 0) {
        for (let i = 0; i < this.size; i++) {
          this.queue[i] = this.queue[this.head + i]
        }
        this.tail = this.size - this.head;
        this.head = 0;
      } else {
        return false
      }
    }

    this.queue[this.tail] = item;
    this.tail += 1;
    return true
  }

  dequeue() {
    if (this.head === this.tail) {
      return null
    }
    
    return this.queue[this.head++];
  }
}

export class LoopQueue<T> {
  private queue: T[] = [];
  private head: number;
  private tail: number;
  size: number;
  count = 0;

  constructor(size: number) {
    this.queue = new Array(size)
    this.head = 0;
    this.tail = 0;
    this.size = size;
  }

  moveIndex(idx: number) {
    return (idx + 1) % this.size
  }

  enqueue(item: T) {
    if (this.count === this.size) {
      return false
    } else {
      this.queue[this.tail] = item;
      this.tail = this.moveIndex(this.tail)
      this.count++;
    }
  }

  dequeue() {
    if (this.count > 0) {
      const item = this.queue[this.head];
      this.head = this.moveIndex(this.head)
      this.count--;
      return item
    }
    return null
  }
}

const queue = new LoopQueue<number>(3);