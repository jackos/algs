use super::LinkedList;
// Take two linked list and add the numbers together.

// Recursive function to add the linked lists together, it calls itself until it hits a None in either list
// Adding the remainder is a little hacky where it just calls itself with one LinkedList having
// a value of `1`, but it's nicer and more concise than the alternatives
pub fn add(l1: Option<Box<LinkedList>>, l2: Option<Box<LinkedList>>) -> Option<Box<LinkedList>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            if sum < 10 {
                Some(Box::new(LinkedList {
                    val: sum,
                    next: add(n1.next, n2.next),
                }))
            } else {
                let remainder = Some(Box::new(LinkedList::new(1)));
                Some(Box::new(LinkedList {
                    val: sum - 10,
                    next: add(add(remainder, n1.next), n2.next),
                }))
            }
        }
    }
}

#[test]
fn add_simple() {
    let v1 = vec![3, 5, 7];
    let v2 = vec![5, 1, 2];
    let l1 = Some(Box::new(LinkedList::from_array(v1)));
    let l2 = Some(Box::new(LinkedList::from_array(v2)));

    let result = add(l1, l2);
    println!("{:?}", result)
}
