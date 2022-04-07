use algs::vec::median_sorted_two_vecs;

fn main() {
    let vec1 = [1, 2, 4, 6];
    let vec2 = [5, 8, 10, 15];
    let result = median_sorted_two_vecs(vec1, vec2);

    println!("result: {result}")
}
