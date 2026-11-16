pub struct Model {
    j: QuarticJacobian,
    sec: good::Sections,
    beta: QClass,
    shift: QClass,
    g: QClass,
}
impl Model {
    pub fn new() -> Self {
        let f = [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111];
        let pts = vec![
            [18, 52, 1],
            [28, 86, 1],
            [43, 125, 1],
            [101, 43, 1],
            [34, 113, 1],
        ];
        let j = QuarticJacobian::with_points(127, f, pts).unwrap();
        let sec = good::Sections::new();
        good::verify_canonical_line();
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
        assert_eq!(j.add_raw(&beta, &beta), h);
        let shift = j.neg_raw(&beta);
        let d = target::difference(&j, [43, 125, 1], [101, 43, 1]);
        let g = target::scalar(&j, j.add_raw(&d, &d), 1005372);
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
        oracle: bool,
    ) -> Option<[u32; 30]> {
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
        let result = j.add_raw(&oriented, &j.neg_raw(&self.g));
        assert_ne!(result, j.zero());
        assert_eq!(target::scalar(j, result, 1028893), j.zero());
        // Oracle is verification ONLY, after public adjacent-Kummer orientation.
        if oracle {
            let p = target::difference(j, [28, 86, 1], [18, 52, 1]);
            let expected = target::scalar(j, j.add_raw(&p, &p), 203071);
            assert_eq!(result, expected, "post-recovery geometric oracle");
            println!("ORIENTATION adjacent_public_input=true chosen_positive={a} oracle_used_to_choose=false IMAGE_B={:?}",result.0);
        }
        Some(result.0)
    }
    pub fn monomials() -> Vec<(u8, u8, u8)> {
        target::monomials(3)
    }
}
