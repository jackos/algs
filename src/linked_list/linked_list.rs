#[derive(Debug)]
pub struct LinkedList {
    pub val: i32,
    pub next: Option<Box<LinkedList>>,
}

impl LinkedList {
    pub fn new(val: i32) -> LinkedList {
        LinkedList { val, next: None }
    }

    // Convenience function to convert array to LinkedList
    pub fn from_array(array: Vec<i32>) -> LinkedList {
        let mut current = LinkedList {
            val: array[0],
            next: None,
        };

        for i in array.iter().skip(1) {
            let new = LinkedList {
                val: *i,
                next: Some(Box::new(current)),
            };
            current = new;
        }
        current
    }
}
