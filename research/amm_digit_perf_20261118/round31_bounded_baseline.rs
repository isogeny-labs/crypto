// Frozen pre-round31 implementations from src/cryptanalysis/prime_root_bounded.rs.
// Do not change: these methods provide the true compiled-Rust A/B reference.
impl Producer<'_> {
    fn before_add(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok(((a as u128 + b as u128) % self.p as u128) as u64)
    }
    fn before_sub(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok(((a as u128 + self.p as u128 - b as u128) % self.p as u128) as u64)
    }
    fn before_mul(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok((a as u128 * b as u128 % self.p as u128) as u64)
    }
}
impl Replay {
    fn before_times(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok((u128::from(x) * u128::from(y) % u128::from(self.p)) as u64)
    }
    fn before_plus(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok((x + y) % self.p)
    }
    fn before_minus(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.scalar()?;
        Ok((x + self.p - y) % self.p)
    }
}
