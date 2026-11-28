use crate::chart::{homogeneous, Chart, Divisor};
use crate::field::Field;
use crate::inherited::{padd, peval, pmod, pmonic, pmul, pscale, psub, ptrim, Fld};

pub struct Kernel {
    pub model: Vec<u32>,
    pub g: [Vec<u32>; 3],
    pub h: [Vec<u32>; 3],
    pub delta: u32,
    pub codomain: Vec<u32>,
}

fn derivative(f: &Field, p: &[u32]) -> Vec<u32> {
    let mut d: Vec<_> = p.iter().enumerate().skip(1)
        .map(|(i, c)| f.mul(c, &(i as u32 % 3))).collect();
    ptrim(f, &mut d);
    d
}

fn brackets(f: &Field, g: &[Vec<u32>; 3]) -> [Vec<u32>; 3] {
    [(1, 2), (2, 0), (0, 1)].map(|(j, k)| {
        psub(f, &pmul(f, &g[j], &derivative(f, &g[k])),
            &pmul(f, &derivative(f, &g[j]), &g[k]))
    })
}

fn determinant(f: &Field, g: &[Vec<u32>; 3]) -> u32 {
    assert!(g.iter().all(|p| p.len() == 3 && p[2] != 0));
    let row = g.each_ref().map(|p| [p[2], p[1], p[0]]);
    let minor = |j: usize, k: usize| f.sub(
        &f.mul(&row[1][j], &row[2][k]),
        &f.mul(&row[1][k], &row[2][j]),
    );
    f.add(&f.sub(&f.mul(&row[0][0], &minor(1, 2)),
        &f.mul(&row[0][1], &minor(0, 2))),
        &f.mul(&row[0][2], &minor(0, 1)))
}

impl Kernel {
    pub fn new(f: &Field, model: &[u32], g: [Vec<u32>; 3]) -> Result<Self, &'static str> {
        assert_eq!(pmul(f, &pmul(f, &g[0], &g[1]), &g[2]), model);
        let delta = determinant(f, &g);
        if delta == 0 {
            return Err("actual zero determinant/product endpoint");
        }
        let h = brackets(f, &g);
        let codomain = pscale(f, &pmul(f, &pmul(f, &h[0], &h[1]), &h[2]), &f.inv(&delta));
        assert_eq!(codomain.len(), 7);
        assert!(codomain.iter().all(|&c| c < 3));
        // Check the full coefficient identity, not selected evaluation points.
        for j in 0..=2 {
            let mut coefficient = Vec::new();
            for i in 0..3 {
                coefficient = padd(f, &coefficient, &pscale(f, &g[i], &h[i][j]));
            }
            let mut expected = vec![0; 3];
            expected[2 - j] = f.neg(&delta);
            if j == 1 {
                expected[1] = f.mul(&2, &delta);
            }
            ptrim(f, &mut expected);
            assert_eq!(coefficient, expected);
        }
        Ok(Self { model: model.to_vec(), g, h, delta, codomain })
    }

    pub fn dual(&self, f: &Field) -> Self {
        let raw = brackets(f, &self.h);
        let factor = f.neg(&f.mul(&2, &self.delta));
        for (a, b) in raw.iter().zip(&self.g) {
            assert_eq!(*a, pscale(f, b, &factor));
        }
        assert_eq!(determinant(f, &self.h), f.neg(&f.mul(&2, &f.mul(&self.delta, &self.delta))));
        let mut g = self.h.clone();
        g[0] = pscale(f, &g[0], &f.inv(&self.delta));
        let dual = Self::new(f, &self.codomain, g).unwrap();
        assert_eq!(dual.codomain, pscale(f, &self.model, &f.mul(&2, &2)));
        assert_eq!(dual.codomain, self.model);
        dual
    }
}

pub fn image_point(target: &Chart<'_>, kernel: &Kernel, a: u32, b: u32, y: u32) -> Divisor {
    assert!(a != 0 || b != 0);
    assert_ne!(y, 0);
    let f = target.h.f;
    let mut value = 0;
    for (i, c) in kernel.model.iter().enumerate() {
        value = f.add(&value, &f.mul(c, &f.mul(&f.pow(a, i as u32), &f.pow(b, (6 - i) as u32))));
    }
    assert_eq!(f.mul(&y, &y), value);
    let binary = |p: &[u32]| {
        let mut value = 0;
        for (i, c) in p.iter().enumerate() {
            value = f.add(&value, &f.mul(c, &f.mul(&f.pow(a, i as u32), &f.pow(b, (2 - i) as u32))));
        }
        value
    };
    let g0 = binary(&kernel.g[0]);
    let g1 = binary(&kernel.g[1]);
    let relation = padd(f, &pscale(f, &kernel.h[0], &g0), &pscale(f, &kernel.h[1], &g1));
    assert!(!relation.is_empty());
    let numerator = pscale(f, &pmul(f, &kernel.h[0], &[a, f.neg(&b)]), &f.mul(&g0, &f.inv(&y)));
    let u = pmonic(f, &homogeneous(f, &relation, 2, target.r));
    assert!(!u.is_empty() && u.len() <= 3);
    let v = pmod(f, &pscale(f, &homogeneous(f, &numerator, 3, target.r), &f.inv(&target.scale)), &u);
    let d = Divisor { u, v };
    assert!(target.h.is_valid(&d));
    d
}

pub fn eligible(source: &Chart<'_>, d: &Divisor) -> bool {
    source.h.is_valid(d)
        && (source.h.is_identity(d) || d.u.len() == 3)
        && source.frobenius(d) == *d
}

pub fn support(source: &Chart<'_>, d: &Divisor) -> [(u32, u32, u32); 2] {
    assert!(eligible(source, d) && !source.h.is_identity(d));
    let f = source.h.f;
    assert_eq!(d.u[2], 1);
    let discriminant = f.sub(&f.mul(&d.u[1], &d.u[1]), &d.u[0]);
    let root = f.sqrt(discriminant).expect("rational degree2 support must split in F3^6");
    let roots = [root, f.neg(&root)].map(|s| f.mul(&f.sub(&s, &d.u[1]), &2));
    roots.map(|t| {
        assert_eq!(peval(f, &d.u, &t), 0);
        let z = peval(f, &d.v, &t);
        let a = f.add(&f.mul(&source.r, &t), &1);
        let b = t;
        let y = f.mul(&source.scale, &z);
        assert_ne!(y, 0, "rational degree2 support cannot contain a branch orbit of length3 or6");
        (a, b, y)
    })
}

pub fn map(source: &Chart<'_>, target: &Chart<'_>, kernel: &Kernel, d: &Divisor) -> Divisor {
    assert!(eligible(source, d));
    assert_eq!(source.model, kernel.model);
    assert_eq!(target.model, kernel.codomain);
    if source.h.is_identity(d) {
        return target.h.identity();
    }
    let points = support(source, d);
    let images = points.map(|(a, b, y)| image_point(target, kernel, a, b, y));
    let image = target.h.add(&images[0], &images[1]);
    assert!(eligible(target, &image));
    image
}

pub fn matching_key(f: &Field, g: &[Vec<u32>; 3]) -> Vec<Vec<u32>> {
    let mut monic: Vec<_> = g.iter().map(|p| pmonic(f, p)).collect();
    monic.sort();
    monic
}

pub fn stable_matchings(label: &str, source: &Chart<'_>, dual: &Kernel) -> usize {
    fn enumerate(remaining: &[usize], pairs: &mut Vec<(usize, usize)>, out: &mut Vec<Vec<(usize, usize)>>) {
        if remaining.is_empty() {
            out.push(pairs.clone());
            return;
        }
        let a = remaining[0];
        for i in 1..remaining.len() {
            let b = remaining[i];
            let rest: Vec<_> = remaining[1..].iter().copied().filter(|&x| x != b).collect();
            pairs.push((a, b));
            enumerate(&rest, pairs, out);
            pairs.pop();
        }
    }
    let f = source.h.f;
    let mut all = Vec::new();
    enumerate(&[0, 1, 2, 3, 4, 5], &mut Vec::new(), &mut all);
    assert_eq!(all.len(), 15);
    let frobenius: Vec<_> = source.roots.iter().map(|&r| {
        source.roots.iter().position(|&s| s == f.pow(r, 3)).unwrap()
    }).collect();
    println!("TARGET_ROOTS label={label} roots={:?} Frobenius={frobenius:?}", source.roots);
    let dual_key = matching_key(f, &dual.g);
    let mut stable = 0;
    let mut dual_found = 0;
    for pairs in all {
        let mut masks: Vec<_> = pairs.iter().map(|&(a, b)| (1u32 << a) | (1u32 << b)).collect();
        let mut images: Vec<_> = pairs.iter().map(|&(a, b)| (1u32 << frobenius[a]) | (1u32 << frobenius[b])).collect();
        masks.sort();
        images.sort();
        let fixed = masks == images;
        println!("TARGET_MATCHING label={label} masks={masks:?} image={images:?} stable={fixed}");
        if !fixed {
            continue;
        }
        stable += 1;
        let mut g: [Vec<u32>; 3] = std::array::from_fn(|i| {
            let (a, b) = pairs[i];
            vec![f.mul(&source.roots[a], &source.roots[b]), f.neg(&f.add(&source.roots[a], &source.roots[b])), 1]
        });
        g[2] = pscale(f, &g[2], source.model.last().unwrap());
        let is_dual = matching_key(f, &g) == dual_key;
        dual_found += usize::from(is_dual);
        match Kernel::new(f, &source.model, g) {
            Ok(k) => println!("TARGET_STABLE label={label} is_exact_dual_kernel={is_dual} delta={} codomain={:?}", k.delta, k.codomain),
            Err(reason) => println!("TARGET_STABLE label={label} is_exact_dual_kernel={is_dual} product={reason}"),
        }
    }
    assert_eq!(dual_found, 1);
    println!("TARGET_CLASSIFICATION label={label} all_matchings=15 stable={stable} exact_dual=1 nondual={}", stable - 1);
    stable - 1
}
