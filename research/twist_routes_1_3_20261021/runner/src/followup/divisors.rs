//! Generic finite-chart norms, explicit 3-cover pullbacks and independent CAS replay.
use super::coupled::public_target;
use crate::{
    covers::Selected,
    digest,
    jacobian::{Chart, Div, Poly, Ring},
    next,
    toy_field::*,
    write_json, Fixture, Result,
};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};

fn binom(n: usize, k: usize) -> u64 {
    (0..k).fold(1, |a, i| a * (n - i) as u64 / (i + 1) as u64)
}
fn transform(k: Sext, f: &[S], n: usize, r: S, lambda: S) -> Poly {
    assert!(f.len() <= n + 1);
    let mut p = vec![Z; n + 1];
    for (i, c) in f.iter().enumerate() {
        for j in 0..=i {
            p[n - j] = k.add(
                p[n - j],
                k.mul(
                    *c,
                    k.mul(
                        k.c(binom(i, j)),
                        k.mul(k.pow(r, (i - j) as u128), k.pow(lambda, j as u128)),
                    ),
                ),
            );
        }
    }
    Ring(k).trim(p)
}
fn derivative(k: Sext, p: &[S]) -> Poly {
    Ring(k).trim(
        p.iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| k.mul(k.c(i as u64), *c))
            .collect(),
    )
}
fn ppow(r: Ring, mut p: Poly, mut n: u128, m: &[S]) -> Poly {
    let mut q = vec![ONE];
    while n != 0 {
        if n & 1 != 0 {
            q = r.rem(&r.mul(&q, &p), m);
        }
        p = r.rem(&r.mul(&p, &p), m);
        n >>= 1;
    }
    q
}
fn root(k: Sext, f: &[S]) -> S {
    let r = Ring(k);
    let mut p = r.monic(f);
    let mut state = 202610230019u64;
    for _ in 0..256 {
        if p.len() == 2 {
            return k.neg(p[0]);
        }
        let v = (0..p.len() - 1)
            .map(|_| S(std::array::from_fn(|_| next(&mut state) % k.p)))
            .collect();
        let a = r.sub(&ppow(r, v, ((k.p as u128).pow(6) - 1) / 2, &p), &[ONE]);
        let (d, _, _) = r.egcd(&p, &a);
        if d.len() > 1 && d.len() < p.len() {
            p = d;
        }
    }
    panic!("branch root factor cap");
}
#[derive(Clone)]
struct NormMap {
    xn: Poly,
    xd: Poly,
    yn: Poly,
    yd: Poly,
    a: S,
    b: S,
    base: SP,
}
impl NormMap {
    fn norm(&self, h: &Chart, d: &Div) -> SP {
        h.valid(d);
        let k = h.k;
        let r = Ring(k);
        let degree = d.u.len() - 1;
        let mut u = d.u.clone();
        loop {
            let (g, _, _) = r.egcd(&u, &self.xd);
            if g.len() == 1 {
                break;
            }
            u = r.monic(&r.exact(&u, &g));
        }
        let sum = if u.len() == 1 {
            None
        } else {
            let x = r.rem(&r.mul(&self.xn, &r.inverse(&self.xd, &u)), &u);
            let y = r.rem(&r.mul(&r.mul(&d.v, &self.yn), &r.inverse(&self.yd, &u)), &u);
            let at = |p: &Poly, i| *p.get(i).unwrap_or(&Z);
            let (al, be, ga, de) = (at(&x, 0), at(&x, 1), at(&y, 0), at(&y, 1));
            if u.len() == 2 {
                Some((al, ga))
            } else if be == Z {
                if de == Z {
                    k.pa(Some((al, ga)), Some((al, ga)), self.a)
                } else {
                    assert_eq!(k.sub(k.mul(k.c(2), ga), k.mul(de, u[1])), Z);
                    None
                }
            } else {
                let slope = k.div(de, be);
                let trace_x = k.sub(k.mul(k.c(2), al), k.mul(be, u[1]));
                let x3 = k.sub(k.mul(slope, slope), trace_x);
                Some((x3, k.sub(k.mul(slope, k.sub(al, x3)), ga)))
            }
        };
        assert!(k.on(sum, self.a, self.b));
        let p = k.pa(sum, k.pn(k.pm(self.base, degree as u128, self.a)), self.a);
        assert!(k.on(p, self.a, self.b));
        p
    }
    fn gp(&self, e: &str) -> String {
        format!(
            "{}, {}, {}, {}, {}, {}",
            e,
            poly(&self.xn),
            poly(&self.xd),
            poly(&self.yn),
            poly(&self.yd),
            point(self.base)
        )
    }
}
fn element(s: S) -> String {
    format!(
        "({}+{}*aa+{}*aa^2+({}+{}*aa+{}*aa^2)*ww)",
        s.0[0], s.0[1], s.0[2], s.0[3], s.0[4], s.0[5]
    )
}
fn poly(p: &[S]) -> String {
    if p.is_empty() {
        "0".into()
    } else {
        p.iter()
            .enumerate()
            .map(|(i, c)| format!("{}*x^{}", element(*c), i))
            .collect::<Vec<_>>()
            .join("+")
    }
}
fn div(d: &Div) -> String {
    format!("[{},{}]", poly(&d.u), poly(&d.v))
}
fn point(p: SP) -> String {
    p.map(|(x, y)| format!("[{},{}]", element(x), element(y)))
        .unwrap_or("[0]".into())
}
fn field_setup(f: &Fixture) -> String {
    let k = Sext::new(f.curve());
    let beta = f.sextic_flat_generator.expect("registered flat generator");
    let powers = (0..6).map(|i| k.pow(beta, i)).collect::<Vec<_>>();
    let solve = |col: usize| {
        let mut a = (0..6)
            .map(|i| {
                let mut v = powers.iter().map(|p| p.0[i]).collect::<Vec<_>>();
                v.push(u64::from(i == col));
                v
            })
            .collect::<Vec<_>>();
        for j in 0..6 {
            let pivot = (j..6).find(|i| a[*i][j] != 0).unwrap();
            a.swap(j, pivot);
            let q = inv(a[j][j], f.p);
            for c in &mut a[j] {
                *c = *c * q % f.p;
            }
            for i in 0..6 {
                if i != j {
                    let q = a[i][j];
                    for l in j..7 {
                        a[i][l] = (a[i][l] + f.p - q * a[j][l] % f.p) % f.p;
                    }
                }
            }
        }
        let coeff = (0..6).map(|i| a[i][6]).collect::<Vec<_>>();
        let actual = coeff
            .iter()
            .zip(&powers)
            .fold(Z, |v, (c, p)| k.add(v, k.mul(k.c(*c), *p)));
        assert_eq!(actual, S(std::array::from_fn(|i| u64::from(i == col))));
        coeff
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{c}*z^{i}"))
            .collect::<Vec<_>>()
            .join("+")
    };
    let mut coefficients = f.sextic_modulus.clone().unwrap();
    assert_eq!(
        coefficients.len(),
        6,
        "registered modulus stores the six lower coefficients"
    );
    coefficients.push(1);
    let evaluation = coefficients.iter().enumerate().fold(Z, |v, (i, c)| {
        k.add(v, k.mul(k.c(*c), k.pow(beta, i as u128)))
    });
    assert_eq!(
        evaluation, Z,
        "registered monic modulus does not annihilate the declared flat generator"
    );
    let modulus = coefficients
        .iter()
        .enumerate()
        .map(|(i, c)| format!("{c}*t^{i}"))
        .collect::<Vec<_>>()
        .join("+");
    format!("p={};modulus=Mod(1,p)*({});if(!polisirreducible(modulus),error(\"registered field reducible\"));z=ffgen(modulus,'z);aa={};ww={};if(aa^3+{}*aa+{}!=0||ww^2!={},error(\"tower to flat field bridge\"));\n",f.p,modulus,solve(1),solve(3),f.a,f.b,f.d)
}
fn lift_point(k: Sext, p: P) -> SP {
    p.map(|(x, y)| (k.lift(x), k.lift(y)))
}
fn fp_point(k: Sext, p: SP) -> P {
    p.map(|(x, y)| (Q(k.fp(x).unwrap(), 0), Q(k.fp(y).unwrap(), 0)))
}

pub fn independent(fixtures: &[Fixture], selected: &Path, out: &Path) -> Result<Vec<Value>> {
    let ss: Vec<Selected> = serde_json::from_slice(&fs::read(selected)?)?;
    let mut rows = vec![];
    for s in ss {
        let f = fixtures.iter().find(|f| f.id == s.fixture).unwrap();
        let e = f.curve();
        let h = Chart::new(e, s.c);
        let k = h.k;
        let r = Ring(k);
        let complement = Quad::new(e.p, h.ad, h.bd);
        let ep = e.rational();
        let dp = complement.rational();
        let es = ep.iter().map(|p| h.pull_e(*p)).collect::<Vec<_>>();
        let ds = dp.iter().map(|p| h.pull_d(*p)).collect::<Vec<_>>();
        let alpha = k.add(k.mul(h.root, h.root), k.c(s.c));
        let fc = k.c(e.rhs(Q(s.c, 0)).0);
        let shift = k.c((3 * s.c * s.c + e.a) % e.p * inv(3, e.p) % e.p);
        let x = vec![Z, ONE];
        let ru = vec![h.lambda, h.root];
        let ru2 = r.mul(&ru, &ru);
        let maps = [
            NormMap {
                xn: r.add(&ru2, &r.scale(&r.mul(&x, &x), k.c(s.c))),
                xd: r.mul(&x, &x),
                yn: vec![h.lambda],
                yd: r.mul(&r.mul(&x, &x), &x),
                a: k.c(e.a),
                b: k.c(e.b),
                base: Some((alpha, Z)),
            },
            NormMap {
                xn: r.add(&r.scale(&ru2, shift), &r.scale(&r.mul(&x, &x), fc)),
                xd: ru2,
                yn: vec![k.mul(fc, h.lambda)],
                yd: r.mul(&r.mul(&ru, &ru), &ru),
                a: k.c(h.ad),
                b: k.c(h.bd),
                base: Some((k.add(shift, k.div(fc, k.mul(h.root, h.root))), Z)),
            },
        ];
        let exhaustive = f.p == 59;
        let budget = if exhaustive { es.len() * ds.len() } else { 256 };
        let mut classes = BTreeMap::new();
        for i in 0..budget {
            let ei = if exhaustive {
                i / ds.len()
            } else {
                i * 31 % es.len()
            };
            let di = if exhaustive {
                i % ds.len()
            } else {
                (i * 67 + 1) % ds.len()
            };
            let d = h.add(&es[ei], &ds[di]);
            for j in 0..2 {
                assert_eq!(maps[j].norm(&h, &d), h.norm(&d, j == 1));
            }
            classes.insert((maps[0].norm(&h, &d), maps[1].norm(&h, &d)), d);
        }
        if exhaustive {
            assert_eq!(classes.len(), 3551);
        }
        let classes = classes.values().cloned().collect::<Vec<_>>();
        let mut gp = String::from(include_str!("divisors.gp"));
        gp.push_str(&field_setup(f));
        gp.push_str(&format!(
            "F={};E=ellinit([{},{}],z);D=ellinit([{},{}],z);C=vector({});\n",
            poly(&h.f),
            e.a,
            e.b,
            h.ad,
            h.bd,
            classes.len()
        ));
        for (i, d) in classes.iter().enumerate() {
            gp.push_str(&format!("C[{}]={};twist_div_valid(C[{}],F);if(twist_div_norm(C[{}],{})!={}||twist_div_norm(C[{}],{})!={},error(\"independent norm replay\"));if(twist_div_add(C[{}],[C[{}][1],-C[{}][2]],F)!=[1,0],error(\"independent inverse replay\"));\n",i+1,div(d),i+1,i+1,maps[0].gp("E"),point(maps[0].norm(&h,d)),i+1,maps[1].gp("D"),point(maps[1].norm(&h,d)),i+1,i+1,i+1));
        }
        for (i, d) in classes.iter().enumerate() {
            let j = (i * 37 + 11) % classes.len();
            let sum = h.add(d, &classes[j]);
            for m in &maps {
                assert_eq!(
                    m.norm(&h, &sum),
                    k.pa(m.norm(&h, d), m.norm(&h, &classes[j]), m.a)
                );
            }
            gp.push_str(&format!("if(twist_div_add(C[{}],C[{}],F)!={},error(\"independent arbitrary-class addition\"));\n",i+1,j+1,div(&sum)));
        }
        gp.push_str("print(\"ALL_DIVISOR_REPLAY_OK\");quit;\n");
        let file = format!("{}.gp", f.id);
        fs::write(out.join(&file), gp)?;
        let row = json!({"experiment":"E24","fixture":f.id,"field":f.sextic_extension_id,"classes":classes.len(),"exhaustive_smallest_Jacobian":exhaustive,"independent_input_script":file,"native_generic_norm_matches_old_both_maps":true,"all_class_addition_and_inverse_cases_emitted":true,"independent_CAS_executed":false,"balanced_base_field_implementation":false,"full_E24_complete":false,"status":"independent-arbitrary-class-check-input-generated"});
        write_json(out.join(format!("{}.json", f.id)), &row)?;
        rows.push(row);
        println!(
            "{}",
            json!({"experiment":"E24","fixture":f.id,"classes":classes.len(),"stage":"native-replay-and-CAS-input-complete"})
        );
    }
    Ok(rows)
}

struct Three {
    h: Chart,
    map: NormMap,
    h3: Poly,
    yn3: Poly,
    old: Poly,
    xn: Poly,
    xd: Poly,
    yn: Poly,
    base: Div,
}
impl Three {
    fn new(f: &Fixture, v: &Value) -> Self {
        let k = Sext::new(f.curve());
        let r = Ring(k);
        let read = |a: &Value| -> Poly {
            a.as_array()
                .unwrap()
                .iter()
                .map(|c| k.c(c.as_str().unwrap().parse().unwrap()))
                .collect()
        };
        let old = r.scale(
            &read(&v["curve"]["f"]),
            k.inv(k.c(v["curve"]["t"].as_str().unwrap().parse().unwrap())),
        );
        let m = &v["maps"][0];
        let xn = read(&m["x_numerator"]);
        let xd = read(&m["x_denominator"]);
        let yn = read(&m["y_factor_numerator"]);
        let rr = root(k, &xd);
        assert_eq!(r.eval(&xd, rr), Z);
        let lambda = r.eval(&derivative(k, &old), rr);
        assert_ne!(lambda, Z);
        let ff = r.scale(
            &transform(k, &old, 6, rr, lambda),
            k.inv(k.mul(lambda, lambda)),
        );
        assert_eq!(ff.len(), 6);
        assert_eq!(ff[5], ONE);
        let h = Chart {
            k,
            c: 0,
            f: ff,
            root: rr,
            lambda,
            ad: 0,
            bd: 0,
        };
        let h3 = transform(k, &xd, 3, rr, lambda);
        let yn3 = transform(k, &yn, 3, rr, lambda);
        let x3 = transform(k, &xn, 3, rr, lambda);
        assert_eq!(h3.len(), 3);
        let base = h.reduce(h3.clone(), vec![]);
        assert_eq!(h.add(&base, &base), Div::zero());
        let map = NormMap {
            xn: x3,
            xd: h3.clone(),
            yn: r.scale(&yn3, lambda),
            yd: r.mul(&h3, &h3),
            a: k.c(f.a),
            b: k.c(f.b),
            base: None,
        };
        Self {
            h,
            map,
            h3,
            yn3,
            old,
            xn,
            xd,
            yn,
            base,
        }
    }
    fn pull(&self, p: P) -> Div {
        let Some((x, y)) = p else { return Div::zero() };
        assert_eq!((x.1, y.1), (0, 0));
        let h = &self.h;
        let k = h.k;
        let r = Ring(k);
        let eq = r.sub(&r.scale(&self.xd, k.c(x.0)), &self.xn);
        assert!(!eq.is_empty());
        let u = r.monic(&transform(k, &eq, eq.len() - 1, h.root, h.lambda));
        let v = r.rem(
            &r.mul(
                &r.scale(&r.mul(&self.h3, &self.h3), k.div(k.c(y.0), h.lambda)),
                &r.inverse(&self.yn3, &u),
            ),
            &u,
        );
        let mut d = h.reduce(u, v);
        if eq.len() < 4 {
            let multiplicity = 4 - eq.len();
            let fy = k
                .sqrt(*self.old.last().unwrap())
                .expect("fiber at infinity requires its actual square root");
            let mut found = 0;
            for s in [fy, k.neg(fy)] {
                let image_x = k.div(*self.xn.get(3).unwrap_or(&Z), self.xd[3]);
                let image_y = k.div(
                    k.mul(s, *self.yn.get(3).unwrap_or(&Z)),
                    k.pow(self.xd[3], 2),
                );
                if (image_x, image_y) == (k.c(x.0), k.c(y.0)) {
                    let inf = Div {
                        u: vec![Z, ONE],
                        v: vec![k.mul(k.pow(h.lambda, 2), s)],
                    };
                    h.valid(&inf);
                    d = h.add(&d, &h.scalar(&inf, multiplicity as u128));
                    found += 1;
                }
            }
            assert_eq!(found, 1, "infinity fiber multiplicity");
        }
        h.add(&d, &h.neg(&self.base))
    }
}
fn bsgs(h: &Chart, g: &Div, q: &Div, n: u64) -> Option<u64> {
    let mut m = 1;
    while m * m < n {
        m += 1;
    }
    let mut table = BTreeMap::new();
    let mut x = Div::zero();
    for j in 0..m {
        table.entry(x.clone()).or_insert(j);
        x = h.add(&x, g);
    }
    let step = h.neg(&h.scalar(g, m as u128));
    let mut x = q.clone();
    for i in 0..=m {
        if let Some(j) = table.get(&x) {
            let k = (i * m + j) % n;
            if h.scalar(g, k as u128) == *q {
                return Some(k);
            }
        }
        x = h.add(&x, &step);
    }
    None
}
pub fn transfer(fixtures: &[Fixture], input: &Path, out: &Path) -> Result<Vec<Value>> {
    let index: Vec<String> = serde_json::from_slice(&fs::read(input)?)?;
    let mut rows = vec![];
    for path in index {
        let vs: Value = serde_json::from_slice(&fs::read(&path)?)?;
        let v = &vs["rows"][0];
        let name = v["source"]["name"].as_str().unwrap();
        let Some(f) = fixtures.iter().find(|f| f.id == name) else {
            rows.push(json!({"experiment":"E19","source":name,"status":"public-parameter-only-no-NIST-DLP-attempt"}));
            continue;
        };
        let model = Three::new(f, v);
        let h = &model.h;
        let k = h.k;
        let e = f.curve();
        let ps = e.rational();
        let before = reference_calls();
        let mut pulls = vec![];
        for &p in &ps {
            let d = model.pull(p);
            assert_eq!(model.map.norm(h, &d), lift_point(k, e.pm(p, 3)));
            assert_eq!(h.add(&d, &h.neg(&d)), Div::zero());
            pulls.push(d);
        }
        let g = model.pull(f.group());
        assert_ne!(g, Div::zero());
        assert_eq!(h.scalar(&g, f.order as u128), Div::zero());
        for (p, d) in ps.iter().zip(&pulls) {
            assert_eq!(h.add(d, &g), model.pull(e.pa(*p, f.group())));
        }
        let mut controls = vec![];
        for label in 0..8 {
            let q = public_target(f, "E19", label);
            let jq = model.pull(q);
            let scalar =
                bsgs(h, &g, &jq, f.order).ok_or("toy destination subgroup logarithm unresolved")?;
            assert_eq!(e.pm(f.group(), scalar as u128), q);
            assert_eq!(
                e.pm(fp_point(k, model.map.norm(h, &jq)), inv(3, f.order) as u128),
                q
            );
            controls.push(json!({"label":label,"public_source_target":q,"destination_target":jq,"recovered_scalar":scalar,"source_scalar_verified":true}));
        }
        let mut gp = String::from(include_str!("divisors.gp"));
        gp.push_str(&field_setup(f));
        gp.push_str(&format!(
            "F={};E=ellinit([{},{}],z);C=vector({});\n",
            poly(&h.f),
            f.a,
            f.b,
            pulls.len()
        ));
        for (i, d) in pulls.iter().enumerate() {
            gp.push_str(&format!("C[{}]={};twist_div_valid(C[{}],F);if(twist_div_norm(C[{}],{})!={},error(\"three-cover norm-pullback\"));\n",i+1,div(d),i+1,i+1,model.map.gp("E"),point(lift_point(k,e.pm(ps[i],3)))));
        }
        for i in 0..pulls.len() {
            let j = (i * 37 + 11) % pulls.len();
            gp.push_str(&format!("if(twist_div_add(C[{}],C[{}],F)!={},error(\"three-cover independent addition\"));\n",i+1,j+1,div(&h.add(&pulls[i],&pulls[j]))));
        }
        gp.push_str("print(\"ALL_DIVISOR_REPLAY_OK\");quit;\n");
        fs::write(out.join(format!("{}.gp", f.id)), gp)?;
        let row = json!({"experiment":"E19","fixture":f.id,"constructor_input":path,"input_sha256":digest(&fs::read(&path)?),"selected_parameter_presentation":v["constructor_parameters"],"branch_root":h.root,"chart_scale":h.lambda,"odd_model":h.f,"origin_fiber_class":model.base,
          "all_source_points":ps.len(),"composition_norm_pullback":"[3]","all_generator_translation_checks":ps.len(),"source_subgroup_order":f.order,"pullback_generator":g,"toy_destination_BSGS_controls":controls,"unpriced_native_reference_calls":reference_calls()-before,
          "independent_CAS_executed":false,"polarized_graph_identification":false,"formal_group_chart_bridge":false,"full_E19_complete":false,"performance_claim":false,"status":"actual-three-cover-divisor-transfer-and-toy-subgroup-recovery"});
        write_json(out.join(format!("{}.json", f.id)), &row)?;
        rows.push(row);
        println!(
            "{}",
            json!({"experiment":"E19","fixture":f.id,"points":ps.len(),"stage":"three-transfer-native-complete"})
        );
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generic_map_norm_handles_old_cover_poles() {
        let e = Quad::new(59, 1, 13);
        let h = Chart::new(e, 0);
        let k = h.k;
        let r = Ring(k);
        let ru = vec![h.lambda, h.root];
        let x = vec![Z, ONE];
        let xn = r.mul(&ru, &ru);
        let xd = r.mul(&x, &x);
        let m = NormMap {
            xn,
            xd,
            yn: vec![h.lambda],
            yd: r.mul(&r.mul(&x, &x), &x),
            a: k.c(1),
            b: k.c(13),
            base: Some((k.mul(h.root, h.root), Z)),
        };
        for p in e.rational() {
            let d = h.pull_e(p);
            assert_eq!(m.norm(&h, &d), h.norm(&d, false));
        }
        assert_eq!(m.norm(&h, &Div::zero()), None);
    }
}
