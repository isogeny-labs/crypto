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
crate::config::install(crate::config::discover(super::setup::qf()));
let cfg=crate::config::get();
let j=QuarticJacobian::with_points(631,cfg.qf,cfg.points.clone()).unwrap();
let sec=good::Sections::new();good::verify_canonical_line();
let mut h=j.zero();for p in j.d_inf{let d=target::difference(&j,p,cfg.o);h=j.add_raw(&h,&j.add_raw(&d,&d));}for p in cfg.canonical{h=j.add_raw(&h,&j.neg_raw(&target::difference(&j,p,cfg.o)));}
let text=std::fs::read_to_string("../hec_calibration631_20261115/alignment/CENTERING.txt").unwrap();let line=text.lines().next().unwrap();let ns:Vec<u32>=line.trim_start_matches("beta=[").trim_end_matches(']').split(',').map(|s|s.trim().parse().unwrap()).collect();let beta=QClass(ns.try_into().unwrap());assert_eq!(j.add_raw(&beta,&beta),h);let shift=j.neg_raw(&beta);
let ns:Vec<u64>=std::fs::read_to_string("../hec_calibration631_20261115/run/anchors01/CALIBRATION.txt").unwrap().split_whitespace().map(|s|s.parse().unwrap()).collect();let n=ns.len();let pair=&ns[n-6..];let g=target::scalar(&j,target::difference(&j,pair[..3].try_into().unwrap(),pair[3..].try_into().unwrap()),4);assert_ne!(g,j.zero());assert_eq!(target::scalar(&j,g,59296481),j.zero());
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
        if image == j.zero() || target::scalar(j, image, 59296481) != j.zero() {
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
        assert_eq!(target::scalar(j, result, 59296481), j.zero());
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
