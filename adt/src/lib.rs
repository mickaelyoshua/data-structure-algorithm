pub mod linked_list {
    // By using a `type` alias, we can avoid writing the long type signature.
    type Link<T> = Option<Box<Node<T>>>;

    // A struct-based implementation of a linked list.
    pub struct LinkedList<T> {
        head: Link<T>,
    }

    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    impl<T> LinkedList<T> {
        pub fn search(&self, item: T) {

        }
    }
}
