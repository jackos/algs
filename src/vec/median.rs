use std::cmp::Ordering::Equal;
use std::ops::Add;

struct SortedVecsIter<T> {
    vec1: Vec<T>,
    vec2: Vec<T>,
}

impl<T> SortedVecsIter<T> {
    fn len(&self) -> usize {
        self.vec1.len() + self.vec2.len()
    }
}

// Will loop through the elements as though it's one vector but backwards
impl<T: PartialOrd> Iterator for SortedVecsIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match (self.vec1.last(), self.vec2.last()) {
            (Some(n1), Some(n2)) if n1 >= n2 => self.vec1.pop(),
            (Some(_), Some(_)) => self.vec2.pop(),
            (Some(_), _) => self.vec1.pop(),
            (_, Some(_)) => self.vec2.pop(),
            _ => None,
        }
    }
}

/**
Takes a vec of a numeric type that is able to be added, partially orderable
and convertible to f64. It saves cpu cycles by looping over both arrays as
though they're one.
## Examples
#### Two `Vec<i32>` median
```
let nums1 = vec![1, 5, 7];
let nums2 = vec![1, 4, 7];
assert_eq!(algs::vec::median_sorted_two_vecs(nums1, nums2), 4.5);
```

#### Two `Vec<f32>` median
**/
pub fn median_sorted_two_vecs<T>(nums1: Vec<T>, nums2: Vec<T>) -> f64
where
    T: PartialOrd + Add + Into<f64>,
    f64: From<<T as Add>::Output>,
{
    let mut vecs_iter = SortedVecsIter {
        vec1: nums1,
        vec2: nums2,
    };
    let length = vecs_iter.len();
    let middle = length / 2;
    if 0 == length % 2 {
        // Take the two middle points and average them
        let mut halves = vecs_iter.skip(middle - 1);
        let first: f64 = halves.next().unwrap().into();
        let second: f64 = halves.next().unwrap().into();
        (first + second) / 2.0
    } else {
        vecs_iter.nth(middle).unwrap().into()
    }
}

/**
The sorted version of this alg runs much faster, this is just to demonstrate how to implement this
in a simple way.

# Examples
## Two unsorted `Vec<i32>` median
```
let mut nums1 = vec![5, 1, 7];
let mut nums2 = vec![7, 1, 4];
assert_eq!(algs::vec::median_unsorted_two_vecs(&mut nums1, &mut nums2), 4.5);
```
**/
pub fn median_unsorted_two_vecs<T>(nums1: &mut Vec<T>, nums2: &mut Vec<T>) -> f64
where
    T: PartialOrd + Add + Copy + Into<f64>,
    f64: From<<T as Add>::Output>,
{
    nums1.append(nums2);
    nums1.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    let mut median: f64;
    let midpoint = nums1.len() / 2;
    if nums1.len() % 2 == 0 {
        median = (nums1[midpoint - 1] + nums1[midpoint]).into();
        median /= 2.0;
    } else {
        median = nums1[midpoint].into();
    }
    median
}

#[test]
fn test_unsorted_i32() {
    let mut nums1 = vec![5, 1, 7];
    let mut nums2 = vec![7, 1, 4];
    assert_eq!(median_unsorted_two_vecs(&mut nums1, &mut nums2), 4.5);
}

#[test]
fn test_unsorted_f64() {
    let mut nums1 = vec![5.6, 1.5, 7.5];
    let mut nums2 = vec![7.5, 1.5, 4.5];
    assert_eq!(median_unsorted_two_vecs(&mut nums1, &mut nums2), 5.05);
}

#[test]
fn test_sorted() {
    let nums1 = vec![1, 5, 7];
    let nums2 = vec![1, 4, 7];
    assert_eq!(median_sorted_two_vecs(nums1, nums2), 4.5);
}

#[test]
fn test_sorted_f64() {
    let nums1 = vec![1.25, 15.2, 50.47];
    let nums2 = vec![0.25, 15.77, 200.6];
    assert_eq!(median_sorted_two_vecs(nums1, nums2), 15.485);
}
