    // Opt-in exact degree1/2 roots. The original generic path remains reference.
    fn short_roots(&self, a: &[F::E]) -> Result<Vec<F::E>> {
        self.check()?;
        let mut out = if a.len() == 2 {
            vec![self.mul(&self.neg(&a[0]), &self.inv(&a[1])?)]
        } else {
            let disc = self.sub(&self.mul(&a[1], &a[1]), &self.mul(&self.n(4), &self.mul(&a[0], &a[2])));
            if let Some(root) = self.sqrt(&disc)? {
                let scale = self.inv(&self.mul(&self.n(2), &a[2]))?;
                let minus_b = self.neg(&a[1]);
                vec![self.mul(&self.add(&minus_b, &root), &scale), self.mul(&self.sub(&minus_b, &root), &scale)]
            } else { vec![] }
        };
        out.sort(); out.dedup();
        for root in &out {
            if self.eval(a, root) != self.zero() { return Err(Error::Inconsistent("closed roots fail polynomial check")); }
        }
        self.check()?;
        Ok(out)
    }
