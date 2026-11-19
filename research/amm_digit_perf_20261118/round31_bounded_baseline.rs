// Frozen pre-round31 implementations from src/cryptanalysis/prime_root_bounded.rs.
// Do not change: these methods provide the true compiled-Rust A/B reference.
// Frozen pre-round31 gate: two clock reads on every successful scalar op.
impl Gate {
    fn before_scalar(&mut self) -> Result<()> {
        self.clock()?;
        if self.metrics.field_work >= self.limits.field_work {
            return Err(Stop::FieldLimit);
        }
        self.charge(32)?;
        self.metrics.field_work += 1;
        self.metrics.arithmetic_invoked = true;
        Ok(())
    }
}
impl Producer<'_> {
    fn before_add(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.before_scalar()?;
        Ok(((a as u128 + b as u128) % self.p as u128) as u64)
    }
    fn before_sub(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.before_scalar()?;
        Ok(((a as u128 + self.p as u128 - b as u128) % self.p as u128) as u64)
    }
    fn before_mul(&mut self, a: u64, b: u64) -> Result<u64> {
        self.gate.before_scalar()?;
        Ok((a as u128 * b as u128 % self.p as u128) as u64)
    }
}
impl Replay {
    fn before_times(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.before_scalar()?;
        Ok((u128::from(x) * u128::from(y) % u128::from(self.p)) as u64)
    }
    fn before_plus(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.before_scalar()?;
        Ok((x + y) % self.p)
    }
    fn before_minus(&mut self, x: u64, y: u64) -> Result<u64> {
        self.gate.before_scalar()?;
        Ok((x + self.p - y) % self.p)
    }
}
