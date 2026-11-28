mod inherited_counts;
mod inherited_polynomials;

use inherited_polynomials::{padd, pmod, pmonic, pmul, pscale, psub, ptrim, peval, pxgcd, Fld};

const ORIGINAL: [u32; 7] = [1, 0, 1, 0, 0, 1, 1];
type Matching = [[usize; 2]; 3];

struct F27;

impl F27 {
    fn pow(&self, mut a: u32, mut n: u32) -> u32 {
        let mut value = 1;
        while n != 0 {
            if n & 1 == 1 { value = self.mul(&value, &a); }
            n >>= 1;
            if n != 0 { a = self.mul(&a, &a); }
        }
        value
    }

    fn controls(&self) {
        assert!((0_u32..3).all(|a| !(a * a * a + 2 * a + 2).is_multiple_of(3)));
        assert_eq!(self.pow(3, 3), 4);
        for a in 0..27 {
            assert_eq!(self.add(&a, &0), a);
            assert_eq!(self.mul(&a, &1), a);
            assert_eq!(self.pow(a, 27), a);
            assert_eq!(self.add(&a, &self.neg(&a)), 0);
            if a != 0 { assert_eq!(self.mul(&a, &self.inv(&a)), 1); }
            for b in 0..27 {
                assert_eq!(self.add(&a, &b), self.add(&b, &a));
                assert_eq!(self.mul(&a, &b), self.mul(&b, &a));
                for c in 0..27 {
                    assert_eq!(self.add(&self.add(&a, &b), &c), self.add(&a, &self.add(&b, &c)));
                    assert_eq!(self.mul(&self.mul(&a, &b), &c), self.mul(&a, &self.mul(&b, &c)));
                    assert_eq!(self.mul(&a, &self.add(&b, &c)), self.add(&self.mul(&a, &b), &self.mul(&a, &c)));
                }
            }
        }
    }
}

impl Fld for F27 {
    type E = u32;
    fn zero(&self) -> u32 { 0 }
    fn one(&self) -> u32 { 1 }
    fn add(&self, a: &u32, b: &u32) -> u32 {
        (a % 3 + b % 3) % 3
            + 3 * ((a / 3 % 3 + b / 3 % 3) % 3)
            + 9 * ((a / 9 + b / 9) % 3)
    }
    fn sub(&self, a: &u32, b: &u32) -> u32 { self.add(a, &self.neg(b)) }
    fn neg(&self, a: &u32) -> u32 { self.add(a, a) }
    fn mul(&self, a: &u32, b: &u32) -> u32 {
        let aa = [a % 3, a / 3 % 3, a / 9];
        let bb = [b % 3, b / 3 % 3, b / 9];
        let mut product = [0; 5];
        for (i, x) in aa.iter().enumerate() {
            for (j, y) in bb.iter().enumerate() {
                product[i + j] = (product[i + j] + x * y) % 3;
            }
        }
        for k in (3..5).rev() {
            let c = product[k];
            product[k - 3] = (product[k - 3] + c) % 3;
            product[k - 2] = (product[k - 2] + c) % 3;
            product[k] = 0;
        }
        product[0] + 3 * product[1] + 9 * product[2]
    }
    fn inv(&self, a: &u32) -> u32 {
        assert_ne!(*a, 0);
        self.pow(*a, 25)
    }
}

fn matchings(labels: &[usize]) -> Vec<Vec<[usize; 2]>> {
    if labels.is_empty() { return vec![vec![]]; }
    let mut result = Vec::new();
    for j in 1..labels.len() {
        let remaining: Vec<_> = labels.iter().enumerate()
            .filter_map(|(i, &x)| (i != 0 && i != j).then_some(x)).collect();
        for mut tail in matchings(&remaining) {
            tail.insert(0, [labels[0], labels[j]]);
            result.push(tail);
        }
    }
    result
}

fn canonical(mut matching: Matching) -> Matching {
    for pair in &mut matching { pair.sort(); }
    matching.sort();
    matching
}

fn derivative(f: &F27, p: &[u32]) -> Vec<u32> {
    let mut result: Vec<_> = p.iter().enumerate().skip(1)
        .map(|(i, c)| f.mul(c, &(i as u32 % 3))).collect();
    ptrim(f, &mut result);
    result
}

fn determinant(f: &F27, g: &[Vec<u32>; 3]) -> u32 {
    let a = g.each_ref().map(|p| [p[2], p[1], p[0]]);
    let minor = |j: usize, k: usize| f.sub(&f.mul(&a[1][j], &a[2][k]), &f.mul(&a[1][k], &a[2][j]));
    f.add(&f.sub(&f.mul(&a[0][0], &minor(1, 2)), &f.mul(&a[0][1], &minor(0, 2))), &f.mul(&a[0][2], &minor(0, 1)))
}

fn co_wronskians(f: &F27, g: &[Vec<u32>; 3]) -> [Vec<u32>; 3] {
    [(1, 2), (2, 0), (0, 1)].map(|(j, k)| {
        psub(f, &pmul(f, &g[j], &derivative(f, &g[k])), &pmul(f, &derivative(f, &g[j]), &g[k]))
    })
}

fn bivariate_identity(f: &F27, g: &[Vec<u32>; 3], h: &[Vec<u32>; 3], delta: u32) {
    let mut sum = [[0; 3]; 3];
    for i in 0..3 {
        for (x, &a) in g[i].iter().enumerate() {
            for (z, &b) in h[i].iter().enumerate() { sum[x][z] = f.add(&sum[x][z], &f.mul(&a, &b)); }
        }
    }
    let mut expected = [[0; 3]; 3];
    expected[0][2] = f.neg(&delta);
    expected[2][0] = f.neg(&delta);
    expected[1][1] = f.mul(&2, &delta);
    assert_eq!(sum, expected);
}

fn curve_counts(polynomial: &[u32]) -> (i128, i128, i128, i128, Vec<u8>, Vec<u8>) {
    assert!(polynomial.iter().all(|&c| c < 3));
    let p: Vec<_> = polynomial.iter().map(|&c| c as u8).collect();
    let degree = p.len() - 1;
    assert!(degree == 5 || degree == 6);
    let infinity3 = if degree == 5 { 1 } else if p[degree] == 1 { 2 } else { 0 };
    let infinity9 = if degree == 5 { 1 } else { 2 };
    let (n1, fibres1) = inherited_counts::point_counts(&p, false, infinity3);
    let (n2, fibres2) = inherited_counts::point_counts(&p, true, infinity9);
    let (t, b) = inherited_counts::weil(n1, n2);
    (n1, n2, t, b, fibres1, fibres2)
}

fn main() {
    let f = F27;
    f.controls();
    let roots: Vec<_> = (0..27).filter(|x| peval(&f, &ORIGINAL, x) == 0).collect();
    assert_eq!(roots.len(), 6);
    let permutation: Vec<_> = roots.iter().map(|&x| roots.iter().position(|&y| y == f.pow(x, 3)).unwrap()).collect();
    assert!(permutation.iter().enumerate().all(|(i, &j)| i != j && permutation[permutation[j]] == i));
    let mut product = vec![1];
    for r in &roots { product = pmul(&f, &product, &[f.neg(r), 1]); }
    assert_eq!(product, ORIGINAL);
    println!("SOURCE q=3 F27_modulus=2,2,0,1 f={ORIGINAL:?} roots={roots:?} Froot={permutation:?} field_controls=pass");
    let mut points = Vec::new();
    let mut fibres = Vec::new();
    for x in 0..27 {
        let fx = peval(&f, &ORIGINAL, &x);
        let mut count = 0;
        for y in 0..27 {
            if f.mul(&y, &y) == fx { points.push((x, y)); count += 1; }
        }
        fibres.push(count);
    }
    assert_eq!(points.len() + 2, 36);
    let generic: Vec<_> = points.iter().copied().filter(|(_, y)| *y != 0).collect();
    assert_eq!(generic.len(), 28);
    println!("SOURCE_F27 affine={} infinity=2 total=36 generic_nonbranch=28 fibres={fibres:?}", points.len());
    let all = matchings(&[0, 1, 2, 3, 4, 5]);
    assert_eq!(all.len(), 15);
    let mut stable = Vec::new();
    for m in all {
        let m: Matching = m.try_into().unwrap();
        let image = m.map(|pair| pair.map(|i| permutation[i]));
        if canonical(image) == canonical(m) { stable.push(canonical(m)); }
    }
    stable.sort();
    stable.dedup();
    assert_eq!(stable.len(), 3);
    let mut products = 0;
    let mut jacobians = 0;
    let mut valid_ideals = 0;
    let mut degree_drops = 0;
    let mut rejected_mutants = 0;
    for (id, m) in stable.iter().enumerate() {
        let g: [Vec<u32>; 3] = m.map(|[i, j]| pmul(&f, &[f.neg(&roots[i]), 1], &[f.neg(&roots[j]), 1]));
        assert_eq!(pmul(&f, &pmul(&f, &g[0], &g[1]), &g[2]), ORIGINAL);
        let delta = determinant(&f, &g);
        let h = co_wronskians(&f, &g);
        bivariate_identity(&f, &g, &h, delta);
        let pointwise_fixed = m.iter().filter(|&&pair| {
            let mut image = pair.map(|i| permutation[i]); image.sort(); image == pair
        }).count();
        assert_eq!(pointwise_fixed, 0);
        println!("KERNEL id={id} matching={m:?} G={g:?} H={h:?} delta={delta} pointwise_fixed=0 bivariate_identity=pass");
        if delta == 0 {
            products += 1;
            println!("ENDPOINT id={id} kind=product no_smooth_genus2_or_point_map_claim=1");
            continue;
        }
        assert!(delta < 3);
        let codomain = pscale(&f, &pmul(&f, &pmul(&f, &h[0], &h[1]), &h[2]), &f.inv(&delta));
        assert!(codomain.iter().all(|&c| c < 3));
        assert_eq!(pxgcd(&f, &codomain, &derivative(&f, &codomain)).0.len(), 1);
        let (n1, n2, t, b, fibres1, fibres9) = curve_counts(&codomain);
        assert_eq!((n1, n2, t, b), (6, 12, -2, 3));
        let wrong_sign = pscale(&f, &codomain, &2);
        let wrong = curve_counts(&wrong_sign);
        assert_ne!((wrong.2, wrong.3), (-2, 3));
        assert_eq!(curve_counts(&codomain).2, -2);
        println!("ENDPOINT id={id} kind=genus2 f={codomain:?} N1={n1} N2={n2} t={t} b={b} fibres1={fibres1:?} fibres9={fibres9:?} wrong_delta_t={} wrong_delta_b={} sign_mutant_rejected=pass restored=pass", wrong.2, wrong.3);
        jacobians += 1;
        let mut local_rejected = 0;
        let mut local_valid = 0;
        for (point_id, &(x, y)) in generic.iter().enumerate() {
            let g0 = peval(&f, &g[0], &x);
            let g1 = peval(&f, &g[1], &x);
            let relation = padd(&f, &pscale(&f, &h[0], &g0), &pscale(&f, &h[1], &g1));
            assert!(!relation.is_empty());
            if relation.len() != 3 {
                degree_drops += 1;
                println!("POINT id={id}:{point_id} x={x} y={y} exception=degree_drop relation={relation:?}");
                continue;
            }
            let u = pmonic(&f, &relation);
            let numerator = pscale(&f, &pmul(&f, &h[0], &[x, f.neg(&1)]), &g0);
            let v = pmod(&f, &pscale(&f, &numerator, &f.inv(&y)), &u);
            assert!(pmod(&f, &psub(&f, &codomain, &pmul(&f, &v, &v)), &u).is_empty());
            let wrong_v = pmod(&f, &pscale(&f, &h[0], &f.mul(&g0, &f.inv(&y))), &u);
            let rejects = !pmod(&f, &psub(&f, &codomain, &pmul(&f, &wrong_v, &wrong_v)), &u).is_empty();
            local_rejected += usize::from(rejects);
            local_valid += 1;
            println!("POINT id={id}:{point_id} x={x} y={y} u={u:?} v={v:?} ordinate_mutant_rejected={} restored=pass", u8::from(rejects));
        }
        assert!(local_valid > 0 && local_rejected > 0);
        valid_ideals += local_valid;
        rejected_mutants += local_rejected;
    }
    assert!(jacobians > 0);
    assert_eq!(products + jacobians, 3);
    assert_eq!(valid_ideals + degree_drops, 28 * jacobians);
    println!("SUMMARY actual_stable_kernels=3 product_endpoints={products} smooth_primefield_endpoints={jacobians} generic_point_ideals={valid_ideals} retained_degree_drops={degree_drops} ordinate_mutants_rejected={rejected_mutants} branch_exceptions_per_endpoint=6 actual_total_Jacobian_map=not_yet_implemented DLP_cost_advantage=not_claimed");
}
