#[derive(Debug)]
struct Queue<T> {
    queue: Vec<T>,
    head: usize,
    tail: usize,
}

impl<T: Clone> Queue<T> {

    fn new(size: usize) -> Self {
        Queue {
            queue: Vec::with_capacity(size),
            head: 0,
            tail: 0,
        }
    }

    fn enququ(&mut self, item: T) -> bool {
        let c = self.queue.capacity();
        if self.head == 0 && self.tail == c {
            return false
        }

        if self.head > 0 {
            for i in 0..self.head {
                self.queue[i] = self.queue[self.head + i].clone();
            }
            self.tail = self.tail - self.head;
            self.head = 0;

            self.queue[self.tail] = item;
        } else {
            self.queue.push(item);
        }
        self.tail += 1;
        true
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.head < self.tail {
            let item = self.queue[self.head].clone();
            self.head += 1;
            Some(item)
        } else {
            None
        }
    }

    fn expand(&mut self) {
        let old_capacity = self.queue.capacity();
        let ne_capacity = old_capacity * 2;
        let mut new_queue = Vec::with_capacity(ne_capacity);
        for i in self.head..self.tail {
            new_queue.push(self.queue[i].clone());
        }
        self.queue = new_queue;
        self.tail = self.tail - self.head;
        self.head = 0;
    }
}