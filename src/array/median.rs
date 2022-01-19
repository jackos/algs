struct SortedVecsIter {
    vec1: Vec<i32>,
    vec2: Vec<i32>,
}

impl SortedVecsIter {
    fn len(&self) -> usize {
        self.vec1.len() + self.vec2.len()
    }
}

// Will loop through the elements as though it's one vector but backwards
impl Iterator for SortedVecsIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        match (self.vec1.last(), self.vec2.last()) {
            (Some(n1), Some(n2)) if n1 >= n2 => self.vec1.pop(),
            (Some(_), Some(_)) => self.vec2.pop(),
            (Some(_), _) => self.vec1.pop(),
            (_, Some(_)) => self.vec2.pop(),
            _ => None,
        }
    }
}

pub fn median_sorted(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut vecs_iter = SortedVecsIter {
        vec1: nums1,
        vec2: nums2,
    };
    let length = vecs_iter.len();
    let middle = length / 2;
    if 0 == length % 2 {
        // Take the two middle points and average them
        let mut halves = vecs_iter.skip(middle - 1);
        (halves.next().unwrap() as f64 + halves.next().unwrap() as f64) / 2.0
    } else {
        vecs_iter.nth(middle).unwrap() as f64
    }
}

pub fn median_unsorted(nums1: &mut Vec<i32>, nums2: &mut Vec<i32>) -> f64 {
    nums1.append(nums2);
    nums1.sort();
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
