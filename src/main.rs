struct Queue {
    queue: Vec<u8>,
}

impl<T> Queue {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn enqueue(&mut self, item: u8) -> Vec<u8> {
        self.queue.push(item)
    }

    fn length(self) -> u8 {
        return self.queue.len();
    }
}

fn main() {
    let mut q: Queue = Queue::new();
    println!("{}", q.length());
    q.enqueue(2)
}

fn main() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
    println!("{:?}", x)
}
