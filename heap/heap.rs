trait HasValue {
    type Value: PartialOrd;
    fn value(&self) -> &Self::Value; 
}

struct MaxHeap<T: HasValue> {
    heap: Vec<T>,
    size: usize,
    count: usize,
}

impl<T: HasValue> MaxHeap<T> {
fn new(size: usize) -> MaxHeap<T> {
    MaxHeap {
        heap: Vec::new(),
        size,
        count: 0,
    }
}

fn swap(&mut self, i: usize, j: usize) {
    self.heap.swap(i, j);
}

fn push(&mut self, item: T) -> bool {
    if self.size <= self.count {
        return false
    }

    self.heap[self.count] = item;
    self.count += 1;

    let mut i = self.count;

    while i > 0 {
        if self.heap[(i - 1) >> 1].value() < self.heap[i].value() {
            self.swap(i, (i - 1) >> 1);
            i = (i - 1) >> 1;
        } else {
            break
        }
    }
    true
}

fn pop(&mut self) -> Option<T> {
    if self.count == 0 {
        return None
    } 

    let item = self.heap.swap_remove(1);
    self.heap[1] = self.heap.swap_remove(self.count);
    self.count -= 1;

    let mut i = 0;
    while i * 2 + 1 <= self.count {
        let mut max_pos = i;
        if self.heap[i].value() < self.heap[i * 2 + 1].value() {
            max_pos = i * 2 + 1;
        }
        if i * 2 + 2 <= self.count && self.heap[i].value() < self.heap[i * 2 + 2].value() {
            max_pos = i * 2 + 2;
        }
        
        if max_pos == i {
            break;
        }
        self.swap(i, max_pos);
        i = max_pos;
    }

    Some(item)
}