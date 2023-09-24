use std::{cmp::Ordering, fmt};

struct MinHeap<T: Ord + Copy> {
    data: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: Ord + Copy,
{
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    fn get_parent(&self, child: usize) -> usize {
        match child % 2 {
            0 => (child - 2) / 2,
            _ => (child - 1) / 2,
        }
    }

    fn get_left_child(&self, parent: usize) -> Option<usize> {
        let child = 2 * parent + 1;
        let max_size = self.data.len() - 1;

        if child <= max_size {
            Some(child)
        } else {
            None
        }
    }

    fn get_right_child(&self, parent: usize) -> Option<usize> {
        let child = 2 * parent + 2;
        let max_size = self.data.len() - 1;

        if child <= max_size {
            Some(child)
        } else {
            None
        }
    }

    pub fn insert(&mut self, element: T) {
        // c = 2*p + 1|2
        // p = (c-1|2)/2
        self.data.push(element);
        let mut current_idx = self.data.len() - 1;
        while current_idx > 0 {
            let parent_idx = self.get_parent(current_idx);

            if self.data[parent_idx] > self.data[current_idx] {
                self.data.swap(parent_idx, current_idx)
            }

            current_idx = parent_idx
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        // c = 2*p + 1|2
        // p = (c-1|2)/2
        //self.data.swap(0, self.data.len()-1);

        let min_element = match self.data.len() {
            0 => None,
            1 => self.data.pop(),
            idx => {
                self.data.swap(0, idx - 1);
                let min_element = self.data.pop();

                let mut parent = 0;
                loop {
                    let (left, right) = (self.get_left_child(parent), self.get_right_child(parent));
                    let min_idx = match (left, right) {
                        (None, Some(right)) => {
                            if self.data[parent] < self.data[right] {
                                parent
                            } else {
                                right
                            }
                        }
                        (Some(left), None) => {
                            if self.data[parent] < self.data[left] {
                                parent
                            } else {
                                left
                            }
                        }
                        (Some(left), Some(right)) => {
                            let min_value =
                                self.data[parent].min(self.data[right]).min(self.data[left]);
                            let min_idx = match min_value {
                                min_value if min_value == self.data[parent] => parent,
                                min_value if min_value == self.data[right] => right,
                                _ => left,
                            };
                            min_idx
                        }
                        (None, None) => parent,
                    };

                    if parent == min_idx {
                        break;
                    } else {
                        self.data.swap(parent, min_idx);
                        parent = min_idx;
                    }
                }

                return min_element;
            }
        };
        return min_element;
    }
}

#[test]
fn test() {
    let mut minHeap: MinHeap<i32> = MinHeap::new();
    assert_eq!(minHeap.data, vec![]);

    minHeap.insert(1);
    minHeap.insert(-2);
    minHeap.insert(8);
    minHeap.insert(4);
    minHeap.insert(3);

    assert_eq!(minHeap.pop(), Some(-2));
    assert_eq!(minHeap.pop(), Some(1));
    assert_eq!(minHeap.pop(), Some(3));
    assert_eq!(minHeap.pop(), Some(4));
    assert_eq!(minHeap.pop(), Some(8));
    assert_eq!(minHeap.pop(), None);
    assert_eq!(minHeap.pop(), None);

}
