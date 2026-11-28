#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub variables: u32,
    pub generators: usize,
    pub terms: usize,
    pub degree: usize,
    pub work: u64,
    pub bytes: u64,
}
