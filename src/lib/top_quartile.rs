pub fn top_quartile<T: PartialOrd + Copy>(vec: &Vec<T>) -> T {
    let mut sorted_vec = vec.clone();
    sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let index = (sorted_vec.len() as f32 * 0.75).ceil() as usize - 1;
    sorted_vec[index]
}