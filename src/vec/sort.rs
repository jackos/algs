fn swap(numbers: &mut Vec<i32>, i: usize, j: usize) {
    let temp = numbers[i];
    numbers[i] = numbers[j];
    numbers[j] = temp;
}
pub fn insertion_sorter(numbers: &mut Vec<i32>) {
    for i in 1..numbers.len() {
        let mut j = i;
        while j > 0 && numbers[j - 1] > numbers[j] {
            swap(numbers, j, j - 1);
            j = j - 1;
        }
    }
}

#[test]
fn test_insertion_sort() {
    let mut numbers: Vec<i32> = vec![21, 54, 55, 11, 25, 4, 5];
    insertion_sorter(&mut numbers);
    for i in 0..(numbers.len() - 1) {
        assert!(numbers[i] <= numbers[i + 1]);
    }
}
