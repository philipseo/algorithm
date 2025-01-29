const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
const _N = input.shift();

class Node {
  constructor(value) {
    this.value = value;
    this.next = null;
    this.prev = null;
  }
}

class Deque {
  constructor() {
    this.head = null;
    this.tail = null;
    this.size = 0;
  }

  pushFront(value) {
    const node = new Node(value);

    if (!this.head) {
      this.head = this.tail = node;
    } else {
      node.next = this.head;
      this.head.prev = node;
      this.head = node;
    }

    this.size++;
  }

  pushBack(value) {
    const node = new Node(value);

    if (!this.tail) {
      this.head = this.tail = node;
    } else {
      node.prev = this.tail;
      this.tail.next = node;
      this.tail = node;
    }

    this.size++;
  }

  popFront() {
    if (!this.head) {
      return -1;
    } else {
      const value = this.head.value;
      this.head = this.head.next;

      if (this.head) {
        this.head.prev = null;
      } else {
        this.tail = null;
      }

      this.size--;
      return value;
    }
  }

  popBack() {
    if (!this.tail) {
      return -1;
    } else {
      const value = this.tail.value;
      this.tail = this.tail.prev;

      if (this.tail) {
        this.tail.next = null;
      } else {
        this.head = null;
      }

      this.size--;
      return value;
    }
  }

  getSize() {
    return this.size;
  }

  isEmpty() {
    return this.size === 0 ? 1 : 0;
  }

  getFront() {
    return this.head ? this.head.value : -1;
  }

  getBack() {
    return this.tail ? this.tail.value : -1;
  }
}

const deque = new Deque();
const result = [];

for (const line of input) {
  const [command, value] = line.split(" ").map(Number);

  switch (command) {
    case 1:
      deque.pushFront(value);
      break;
    case 2:
      deque.pushBack(value);
      break;
    case 3:
      result.push(deque.popFront());
      break;
    case 4:
      result.push(deque.popBack());
      break;
    case 5:
      result.push(deque.getSize());
      break;
    case 6:
      result.push(deque.isEmpty());
      break;
    case 7:
      result.push(deque.getFront());
      break;
    case 8:
      result.push(deque.getBack());
      break;
  }
}

console.log(result.join("\n"));
