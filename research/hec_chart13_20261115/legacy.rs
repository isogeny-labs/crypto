pub struct Reference {
    j: QuarticJacobian,
    s: good::Sections,
    shift: QClass,
    base: QClass,
    current: QClass,
}
impl Reference {
    pub fn new(p: [u64; 3], q: [u64; 3]) -> Self {
        let j = QuarticJacobian::with_points(
            127,
            [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111],
            vec![
                [18, 52, 1],
                [28, 86, 1],
                [43, 125, 1],
                [101, 43, 1],
                [34, 113, 1],
            ],
        )
        .unwrap();
        let s = good::Sections::new();
        let o = [18, 52, 1];
        let mut h = j.zero();
        for p in j.d_inf {
            let d = target::difference(&j, p, o);
            h = j.add_raw(&h, &j.add_raw(&d, &d));
        }
        for p in [[101, 43, 1], [34, 113, 1]] {
            h = j.add_raw(&h, &j.neg_raw(&target::difference(&j, p, o)));
        }
        let beta = target::scalar(&j, h, 514447);
        let diff = target::difference(&j, p, q);
        let base = j.add_raw(&diff, &diff);
        assert_ne!(base, j.zero());
        assert_eq!(target::scalar(&j, base, 1028893), j.zero());
        let current = j.zero();
        let shift = j.neg_raw(&beta);
        Self {
            j,
            s,
            shift,
            base,
            current,
        }
    }
    pub fn next(&mut self) -> Option<[u64; 8]> {
        self.current = self.j.add_raw(&self.current, &self.base);
        coord(&self.j, &self.s, self.current, self.shift).map(|c| c.0)
    }
}
