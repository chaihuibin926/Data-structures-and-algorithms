pub trait HasValue {
    type Value: PartialOrd;
    fn value(&self) -> Self::Value; 
}

pub struct MaxHeap<T: HasValue> {
    heap: Vec<T>,
    size: usize,
    count: usize,
}

impl<T: HasValue> MaxHeap<T> {
    pub fn new(size: usize) -> MaxHeap<T> {
        MaxHeap {
            heap: Vec::new(),
            size,
            count: 0,
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.size <= self.count {
            return false
        }

        self.heap.push(item);
        
        let mut i = self.count;
        self.count += 1;

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

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None
        } 

        self.swap(0, self.count - 1);
        let item = self.heap.swap_remove(self.count - 1);
        self.count -= 1;

        let mut i = 0;
        while i * 2 + 1 < self.count {
            let mut max_pos = i;
            if self.heap[i].value() < self.heap[i * 2 + 1].value() {
                max_pos = i * 2 + 1;
            }
            if i * 2 + 2 < self.count && self.heap[i].value() < self.heap[i * 2 + 2].value() {
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
}


pub struct MinHeap<T: HasValue> {
    heap: Vec<T>,
    size: usize,
    count: usize,
}

impl<T: HasValue> MinHeap<T> {
    pub fn new(size: usize) -> MinHeap<T> {
        MinHeap {
            heap: Vec::new(),
            size,
            count: 0,
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.size <= self.count {
            return false
        }

        self.heap.push(item);
        
        let mut i = self.count;
        self.count += 1;

        while i > 0 {
            if self.heap[(i - 1) >> 1].value() > self.heap[i].value() {
                self.swap(i, (i - 1) >> 1);
                i = (i - 1) >> 1;
            } else {
                break
            }
        }
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None
        } 

        self.swap(0, self.count - 1);
        let item = self.heap.swap_remove(self.count - 1);
        self.count -= 1;

        let mut i = 0;
        while i * 2 + 1 < self.count {
            let mut max_pos = i;
            if self.heap[i].value() > self.heap[i * 2 + 1].value() {
                max_pos = i * 2 + 1;
            }
            if i * 2 + 2 < self.count && self.heap[i].value() > self.heap[i * 2 + 2].value() {
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
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

