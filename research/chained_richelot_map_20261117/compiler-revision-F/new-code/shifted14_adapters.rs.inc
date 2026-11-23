
// New visibility-only adapter around the unchanged unit12 chart/witness code.
pub(crate) struct Bridge14(Chart);
impl Bridge14 {
    pub(crate) fn new(source: &Balanced, a: u64, b: u64) -> Result<Self> {
        Chart::new(source, a, b).map(Self)
    }
    pub(crate) fn target(&self) -> &Balanced {
        &self.0.target
    }
    pub(crate) fn forward(&self, d: &Class) -> Result<Class> {
        self.0.map(d, Direction::Forward).map(|m| m.class)
    }
    pub(crate) fn backward(&self, d: &Class) -> Result<Class> {
        self.0.map(d, Direction::Reverse).map(|m| m.class)
    }
}
pub(crate) fn witnesses14(source: &Balanced) -> Result<Vec<(String, Class)>> {
    let k = Fp2Ctx::try_new(P).map_err(|e| e.to_string())?;
    let chart = Chart::new(source, 0, 20)?;
    let roots = quadratic_square_table(&k);
    Ok(witnesses(&chart, &k, &roots, false)?
        .into_iter()
        .map(|w| (w.label, w.class))
        .collect())
}
