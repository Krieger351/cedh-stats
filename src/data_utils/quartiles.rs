use anyhow::Result;
#[derive(Debug)]
pub struct Quartiles<T: PartialOrd + Copy> {
    pub top: T,
    pub bottom: T,
}

pub fn get_quartiles<K: PartialOrd + Copy, T: IntoIterator>(vec: T) -> Result<Quartiles<T>> {
    let mut sorted = vec.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());


    let top = (sorted.len() as f32 * 0.75).floor() as usize;
    let bottom = (sorted.len() as f32 * 0.25).floor() as usize;

    Ok(Quartiles {
        top: sorted[top],
        bottom: sorted[bottom],
    })
}