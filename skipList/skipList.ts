const MAX_LEVEL = 16;

class Node {
  data: number;
  maxLevel: number;
  refer = new Array(MAX_LEVEL)

  constructor(data: number, maxLevel: number) {
    this.data = data;
    this.maxLevel = maxLevel;
  }
}

export class SkipList {
  levelCount = 1;
  head = new Node(-1, 0);

  randomLevel() {
    let level = 1;
    for (let i = 1; i < MAX_LEVEL; i++) {
        if (Math.random() < 0.5) {
            level++;
        }
    }
    return level;
  }

  insert(value: number) {
    const level = this.randomLevel();
    const newNode = new Node(value, level);
    const update = new Array(level);
    let current = this.head;

    for (let i = level - 1; i >= 0; i--) {
      while (current.refer[i] !== undefined && current.refer[i].data < value) {
        current = current.refer[i]
      }
      update[i] = current
    }

    for (let i = level - 1; i >= 0; i--) {
      newNode.refer[i] = update[i].refer[i]
      update[i].refer[i] = newNode
    }

    if (this.levelCount < level) {
      this.levelCount = level
    }
  }

  find(value: number) {
    let level = this.levelCount;
    let current = this.head;
    for (let i = level - 1; i >= 0; i--) {
      while (current.refer[i] !== undefined && current.refer[i].data < value) {
        current = current.refer[i]
      }
    }

    if (current.refer[0] !== undefined && current.refer[0].data === value) {
      return current.refer[0]
    }
    return null
  }

  remove(value: number) {
    let level = this.levelCount;
    let current = this.head;
    const update = new Array(level)

    for (let i = level - 1; i >= 0; i--) {
      while (current.refer[i] !== undefined && current.refer[i].data < value) {
        current = current.refer[i]
      }
      update[i] = current
    }

    if (current.refer[0] !== undefined && current.refer[0].data === value) {
      const remove_node = current.refer[0];

      for (let i = 0; i < remove_node.maxLevel; i++) {
        update[i].refer[i] = update[i].refer[i].refer[i]
      }
      return remove_node;
    }

    return null
  }
}