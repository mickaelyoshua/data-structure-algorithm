use queue_using_two_stacks::queue::Queue;

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(42);
    queue.dequeue();
    queue.enqueue(14);
    queue.print();
    queue.enqueue(28);
    queue.print();
    queue.enqueue(60);
    queue.enqueue(78);
    queue.dequeue();
    queue.dequeue();
}
