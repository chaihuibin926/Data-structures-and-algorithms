// n 为索引
// 左子节点 2n 右子节点为 2n + 1
// 父节点为 n >> 1

class MaxHeap<T extends {value: number}> {
  private count: number;
  private heap: T[];
  public readonly size?: number;

  constructor(size?: number) {
    this.heap = new Array(size && size + 1);
    this.count = 0;
    this.size = size;
  }

  swap(i: number, j: number) {
    [this.heap[i], this.heap[j]] = [this.heap[j], this.heap[i]]
  }

  push(item: T) {
    if (this.size !== undefined && this.count === this.size) {
      return false
    }
    this.count++;
    this.heap[this.count] = item;

    let i = this.count;
    while (i > 1) {
      if (this.heap[i].value > this.heap[i >> 1].value) {
        this.swap(i, i >> 1)
        i = i >> 1
      } else {
        break
      }
    }
    return true
  }

  pop() {
    if (this.count === 0) {
      return null
    }

    const item = this.heap[1]
    this.heap[1] = this.heap[this.count];
    this.heap[this.count] = undefined
    let i = 1
    while (this.heap[2 * i]) {
      let maxPos = i;
      if (this.heap[2 * i].value > this.heap[maxPos].value) {
        maxPos = 2 * i
      }
      if (this.heap[2 * i + 1] && this.heap[2 * i + 1].value > this.heap[maxPos].value) {
        maxPos = 2 * i + 1
      }
      if (maxPos === i) {
        break
      } else {
        this.swap(i, maxPos);
        i = maxPos;
      }
    }
    this.count--
    return item
  }
}

class MinHeap<T extends {value: number}> {
  private count: number;
  private heap: T[];
  public readonly size?: number;

  constructor(size?: number) {
    this.heap = new Array(size && size + 1);
    this.count = 0;
    this.size = size;
  }

  swap(i: number, j: number) {
    [this.heap[i], this.heap[j]] = [this.heap[j], this.heap[i]]
  }

  push(item: T) {
    if (this.size !== undefined && this.count === this.size) {
      return false
    }
    this.count++;
    this.heap[this.count] = item;

    let i = this.count;
    while (i > 1) {
      if (this.heap[i].value < this.heap[i >> 1].value) {
        this.swap(i, i >> 1)
        i = i >> 1
      } else {
        break
      }
    }
    return true
  }

  pop() {
    if (this.count === 0) {
      return null
    }

    const item = this.heap[1]
    this.heap[1] = this.heap[this.count];
    this.heap[this.count] = undefined
    let i = 1
    while (this.heap[2 * i]) {
      let minPos = i;
      if (this.heap[2 * i].value < this.heap[minPos].value) {
        minPos = 2 * i
      }
      if (this.heap[2 * i + 1] && this.heap[2 * i + 1].value < this.heap[minPos].value) {
        minPos = 2 * i + 1
      }
      if (minPos === i) {
        break
      } else {
        this.swap(i, minPos);
        i = minPos;
      }
    }
    this.count--
    return item
  }
}