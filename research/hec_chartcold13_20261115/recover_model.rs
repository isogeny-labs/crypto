pub struct Model {
    j: QuarticJacobian,
    sec: good::Sections,
    beta: QClass,
    shift: QClass,
    g: QClass,
}
impl Model {
    pub fn new() -> Self {
        super::setup::verify_model();
        let f = super::setup::qf();
        let pts = crate::config::get().points.clone();
        let j = QuarticJacobian::with_points(127, f, pts).unwrap();
        let sec = good::Sections::new();
        good::verify_canonical_line();
        let o = crate::config::get().o;
        let mut h = j.zero();
        for p in j.d_inf {
            let d = target::difference(&j, p, o);
            h = j.add_raw(&h, &j.add_raw(&d, &d));
        }
        for p in crate::config::get().canonical {
            h = j.add_raw(&h, &j.neg_raw(&target::difference(&j, p, o)));
        }
        let beta = target::scalar(&j, h, 514447);
        assert_eq!(j.add_raw(&beta, &beta), h);
        let shift = j.neg_raw(&beta);
        let ns: Vec<u64> = std::fs::read_to_string("generated/TARGET-PAIR.txt")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(ns.len(), 6);
        let d = target::difference(&j, ns[..3].try_into().unwrap(), ns[3..].try_into().unwrap());
        let g = j.add_raw(&d, &d);
        assert_ne!(g, j.zero());
        assert_eq!(target::scalar(&j, g, 1028893), j.zero());
        Self {
            j,
            sec,
            beta,
            shift,
            g,
        }
    }
    pub fn candidate(
        &self,
        ann: Vec<Vec<u64>>,
        lambda: [u64; 8],
        next: [u64; 8],
        offset: u64,
    ) -> Option<([u32; 30], bool)> {
        let j = &self.j;
        let z = j.class_from_conditions(ann).ok()?;
        let image = j.add_raw(&z, &self.beta);
        if image == j.zero() || target::scalar(j, image, 1028893) != j.zero() {
            return None;
        }
        let (c, _) = coord(j, &self.sec, image, self.shift)?;
        if !proportional(c, lambda) {
            return None;
        }
        let neg = j.neg_raw(&image);
        let a = coord(j, &self.sec, j.add_raw(&image, &self.g), self.shift)
            .map(|(c, _)| proportional(c, next))
            .unwrap_or(false);
        let b = coord(j, &self.sec, j.add_raw(&neg, &self.g), self.shift)
            .map(|(c, _)| proportional(c, next))
            .unwrap_or(false);
        if a == b {
            return None;
        }
        let oriented = if a { image } else { neg };
        let result = j.add_raw(&oriented, &j.neg_raw(&target::scalar(j, self.g, offset)));
        assert_ne!(result, j.zero());
        assert_eq!(target::scalar(j, result, 1028893), j.zero());
        Some((result.0, a))
    }
    pub fn monomials() -> Vec<(u8, u8, u8)> {
        target::monomials(3)
    }
}

impl Model {
    pub fn verify_ratio(&self, a: [u32; 30], b: [u32; 30], k: u64) {
        assert_eq!(
            target::scalar(&self.j, QClass(a), k),
            QClass(b),
            "destination_scalar_verification"
        );
    }
}
