//! Exact small-field quadratic GHS transfer, degree-one index calculus, and matched rho.
use clap::Parser;
use crypto_lib::binary_ecc::hyperelliptic::MumfordDivisor;
use crypto_lib::binary_ecc::{F2mElement, operation_tally::OperationTally};
use crypto_lib::cryptanalysis::ghs_descent::{ECurve, Pt};
use crypto_lib::cryptanalysis::ghs_generalized::GeneralizedContext;
use crypto_lib::cryptanalysis::ghs_quadratic::{QuadraticDescent, divisor_json};
use crypto_lib::cryptanalysis::ghs_screen::GhsCurveInput;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{Value, json};
use std::{cell::Cell, collections::HashMap, time::Instant};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    degree: u32,
    #[arg(long)]
    modulus: String,
    #[arg(long)]
    a: String,
    #[arg(long)]
    b: String,
    #[arg(long, default_value_t = 37)]
    target_scalar: u64,
    #[arg(long, default_value_t = 20000)]
    relation_limit: u64,
    #[arg(long, default_value_t = 20000)]
    descent_limit: u64,
    #[arg(long, default_value_t = false)]
    two_isogeny: bool,
    #[arg(long, default_value_t = 0)]
    isogeny_degree: u32,
}
fn parse(s: &str) -> Result<BigUint, String> {
    let (s, radix) = s.strip_prefix("0x").map_or((s, 10), |v| (v, 16));
    BigUint::parse_bytes(s.as_bytes(), radix).ok_or("invalid integer".into())
}
fn e(v: u64, n: u32) -> F2mElement {
    F2mElement::from_biguint(&v.into(), n)
}
fn val(x: &F2mElement) -> u64 {
    x.to_biguint().to_u64().unwrap()
}
fn key(p: &Pt) -> (u64, u64, u8) {
    match p {
        Pt::Inf => (0, 0, 1),
        Pt::Aff { x, y } => (val(x), val(y), 0),
    }
}
fn primes(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            out.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}
fn inv(a: u64, p: u64) -> Option<u64> {
    let (mut t, mut nt, mut r, mut nr) = (0i128, 1i128, p as i128, a as i128);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    if r != 1 {
        None
    } else {
        Some(t.rem_euclid(p as i128) as u64)
    }
}
fn phase<T>(f: impl FnOnce() -> Result<T, String>) -> Result<(T, Value), String> {
    let tally = OperationTally::begin();
    let start = Instant::now();
    let result = f();
    let nanos = start.elapsed().as_nanos();
    let counts = tally.finish();
    Ok((
        result?,
        json!({"wall_ns":nanos,"field_operations":counts,"wall_level":"L0_descriptive"}),
    ))
}
fn points(curve: &ECurve) -> Vec<Pt> {
    let q = 1u64 << curve.m;
    let mut out = vec![Pt::Inf];
    let beta = curve.b.clone();
    let mut root = beta;
    for _ in 0..curve.m - 1 {
        root = root.square(&curve.irr);
    }
    out.push(Pt::Aff {
        x: e(0, curve.m),
        y: root,
    });
    let mut artin_schreier = HashMap::new();
    for z in 0..q {
        let zz = e(z, curve.m);
        artin_schreier
            .entry(val(&zz.square(&curve.irr).add(&zz)))
            .or_insert(z);
    }
    for x in 1..q {
        let xx = e(x, curve.m);
        let xsq = xx.square(&curve.irr);
        let right = xx.add(&curve.a).add(
            &curve
                .b
                .mul(&xsq.flt_inverse(&curve.irr).unwrap(), &curve.irr),
        );
        if let Some(&z) = artin_schreier.get(&val(&right)) {
            let yy = xx.mul(&e(z, curve.m), &curve.irr);
            out.push(Pt::Aff {
                x: xx.clone(),
                y: yy.clone(),
            });
            out.push(Pt::Aff {
                x: xx.clone(),
                y: yy.add(&xx),
            });
        }
    }
    out
}
struct TwoIsogeny {
    source: ECurve,
    codomain: ECurve,
    kernel: Vec<Pt>,
    y_shift: F2mElement,
}
impl TwoIsogeny {
    fn image(&self, p: &Pt) -> Pt {
        if *p == Pt::Inf || self.kernel.contains(p) {
            return Pt::Inf;
        }
        let Pt::Aff { x, y } = p else { unreachable!() };
        let (mut nx, mut ny) = (x.clone(), y.clone());
        for t in &self.kernel {
            let Pt::Aff { x: tx, y: ty } = t else {
                unreachable!()
            };
            let Pt::Aff { x: sx, y: sy } = self.source.add(p, t) else {
                unreachable!()
            };
            nx = nx.add(&sx).add(tx);
            ny = ny.add(&sy).add(ty);
        }
        Pt::Aff {
            x: nx,
            y: ny.add(&self.y_shift),
        }
    }
    fn construct(
        source: ECurve,
        all: &[Pt],
        degree: u32,
        beta: F2mElement,
    ) -> Result<Self, String> {
        if ![2, 3, 5, 7].contains(&degree) {
            return Err("supported isogeny degrees: 2,3,5,7".into());
        }
        let t = if degree == 2 {
            Pt::Aff {
                x: e(0, source.m),
                y: beta,
            }
        } else {
            all.iter()
                .find(|p| **p != Pt::Inf && source.scalar_mul(p, &degree.into()) == Pt::Inf)
                .cloned()
                .ok_or(format!("no rational {degree}-torsion kernel"))?
        };
        if source.scalar_mul(&t, &degree.into()) != Pt::Inf {
            return Err("kernel generator has wrong order".into());
        }
        let kernel = (1..degree)
            .map(|k| source.scalar_mul(&t, &k.into()))
            .collect::<Vec<_>>();
        if kernel.iter().any(|p| *p == Pt::Inf) || {
            let mut unique = kernel.iter().map(key).collect::<Vec<_>>();
            unique.sort_unstable();
            unique.dedup();
            unique.len() != kernel.len()
        } {
            return Err("kernel does not have exact requested degree".into());
        }
        let provisional = Self {
            source: source.clone(),
            codomain: source.clone(),
            kernel: kernel.clone(),
            y_shift: e(0, source.m),
        };
        let mut samples = Vec::new();
        for p in all {
            if let Pt::Aff { x, y } = provisional.image(p) {
                let x2 = x.square(&source.irr);
                let rhs = y
                    .square(&source.irr)
                    .add(&x.mul(&y, &source.irr))
                    .add(&x2.mul(&x, &source.irr));
                if samples.iter().all(|(prev, _)| *prev != x) {
                    samples.push((x, rhs));
                }
                if samples.len() == 3 {
                    break;
                }
            }
        }
        if samples.len() != 3 {
            return Err("insufficient distinct image abscissae".into());
        }
        // Fit the general binary Weierstrass model, then remove the x term
        // by Y -> Y + a4. Three distinct x values give a Vandermonde system.
        let mut matrix = Vec::new();
        for (x, rhs) in samples {
            matrix.push(vec![x.square(&source.irr), x, e(1, source.m), rhs]);
        }
        for col in 0..3 {
            let pivot = (col..3)
                .find(|&row| !matrix[row][col].is_zero())
                .ok_or("singular isogeny fit")?;
            matrix.swap(col, pivot);
            let scale = matrix[col][col]
                .flt_inverse(&source.irr)
                .ok_or("singular isogeny fit")?;
            for j in col..4 {
                matrix[col][j] = matrix[col][j].mul(&scale, &source.irr);
            }
            for row in 0..3 {
                if row != col {
                    let factor = matrix[row][col].clone();
                    for j in col..4 {
                        matrix[row][j] =
                            matrix[row][j].add(&factor.mul(&matrix[col][j], &source.irr));
                    }
                }
            }
        }
        let ain = matrix[0][3].clone();
        let shift = matrix[1][3].clone();
        let bin = matrix[2][3].add(&shift.square(&source.irr));
        if bin.is_zero() {
            return Err("isogeny codomain is singular".into());
        }
        let codomain = ECurve::new(source.m, source.irr.clone(), ain, bin);
        let edge = Self {
            source,
            codomain,
            kernel,
            y_shift: shift,
        };
        for p in all {
            if !edge.codomain.is_on_curve(&edge.image(p)) {
                return Err("isogeny image misses fitted codomain".into());
            }
        }
        for p in all.iter().take(20) {
            for q in all.iter().take(20) {
                if edge.image(&edge.source.add(p, q))
                    != edge.codomain.add(&edge.image(p), &edge.image(q))
                {
                    return Err("isogeny map fails additivity".into());
                }
            }
        }
        Ok(edge)
    }
}
fn fold_negation(curve: &ECurve, x: &mut Pt, a: &mut u64, b: &mut u64, r: u64) -> bool {
    let negative = curve.neg(x);
    if key(&negative) < key(x) {
        *x = negative;
        *a = (r - *a) % r;
        *b = (r - *b) % r;
        true
    } else {
        false
    }
}
fn source_rho(curve: &ECurve, p: &Pt, q: &Pt, r: u64) -> Result<Value, String> {
    let mut adds = 0u64;
    let mut doubles = 0u64;
    let mut lookups = 0u64;
    let mut inserts = 0u64;
    let mut folds = 0u64;
    for trial in 0..12u64 {
        let mut a = (1 + trial) % r;
        let mut b = (trial * 7) % r;
        let mut x = curve.add(
            &curve.scalar_mul(p, &a.into()),
            &curve.scalar_mul(q, &b.into()),
        );
        adds += 1;
        folds += u64::from(fold_negation(curve, &mut x, &mut a, &mut b, r));
        let mut seen: HashMap<(u64, u64, u8), (u64, u64)> = HashMap::new();
        for step in 0..(50 * r).max(1000) {
            lookups += 1;
            if let Some(&(old_a, old_b)) = seen.get(&key(&x)) {
                let den = (b + r - old_b) % r;
                if let Some(di) = inv(den, r) {
                    let answer = ((old_a + r - a) % r * di) % r;
                    if curve.scalar_mul(p, &answer.into()) == *q {
                        return Ok(
                            json!({"scalar":answer,"steps":step,"restarts":trial,"elliptic_additions":adds,"elliptic_doublings":doubles,"hash_lookups":lookups,"hash_inserts":inserts,"algorithm":"rho.negation","negation_folds":folds}),
                        );
                    }
                }
                break;
            }
            seen.insert(key(&x), (a, b));
            inserts += 1;
            let part = match &x {
                Pt::Inf => 0,
                Pt::Aff { x, y } => {
                    let mix = val(x).wrapping_mul(0x9e3779b97f4a7c15)
                        ^ val(y).rotate_left(7)
                        ^ trial.wrapping_mul(0xbf58476d1ce4e5b9);
                    (mix ^ (mix >> 29)) % 3
                }
            };
            match part {
                0 => {
                    x = curve.add(&x, p);
                    a = (a + 1) % r;
                    adds += 1;
                }
                1 => {
                    x = curve.double(&x);
                    a = 2 * a % r;
                    b = 2 * b % r;
                    doubles += 1;
                }
                _ => {
                    x = curve.add(&x, q);
                    b = (b + 1) % r;
                    adds += 1;
                }
            }
            folds += u64::from(fold_negation(curve, &mut x, &mut a, &mut b, r));
        }
    }
    Err("rho exhausted deterministic restarts".into())
}
struct FactorBase {
    points: Vec<MumfordDivisor>,
    coords: HashMap<(u64, u64), usize>,
    q: u64,
    hash_lookups: Cell<u64>,
    root_tests: Cell<u64>,
    decomposition_checks: Cell<u64>,
}
impl FactorBase {
    fn build(d: &QuadraticDescent) -> Result<Self, String> {
        let q = 1u64 << d.base_degree;
        let mut points = Vec::new();
        let mut coords = HashMap::new();
        for x in 0..q {
            for y in 0..q {
                if let Ok(div) = MumfordDivisor::try_from_point(
                    &d.curve,
                    &e(x, d.base_degree),
                    &e(y, d.base_degree),
                ) {
                    coords.insert((x, y), points.len());
                    points.push(div);
                }
            }
        }
        Ok(Self {
            points,
            coords,
            q,
            hash_lookups: Cell::new(0),
            root_tests: Cell::new(0),
            decomposition_checks: Cell::new(0),
        })
    }
    fn split(
        &self,
        d: &QuadraticDescent,
        div: &MumfordDivisor,
    ) -> Result<Option<Vec<usize>>, String> {
        let degree = div.u.degree().unwrap_or(0);
        if degree == 0 {
            return Ok(Some(Vec::new()));
        }
        let mut roots = Vec::new();
        for x in 0..self.q {
            self.root_tests.set(self.root_tests.get() + 1);
            let xx = e(x, d.base_degree);
            if div.u.eval(&xx, &d.curve.irr).is_zero() {
                let y = val(&div.v.eval(&xx, &d.curve.irr));
                self.hash_lookups.set(self.hash_lookups.get() + 1);
                let idx = *self
                    .coords
                    .get(&(x, y))
                    .ok_or("split root missing from factor base")?;
                roots.push(idx);
            }
        }
        if degree == 2 && roots.len() == 1 && div.u.coeff(1).is_zero() {
            roots.push(roots[0]);
        }
        if roots.len() != degree {
            return Ok(None);
        }
        let mut reconstructed = MumfordDivisor::identity(d.base_degree);
        for &idx in &roots {
            reconstructed = reconstructed
                .try_add(&self.points[idx], &d.curve)
                .map_err(|e| e.to_string())?;
        }
        self.decomposition_checks
            .set(self.decomposition_checks.get() + 1);
        if reconstructed != *div {
            return Ok(None);
        }
        Ok(Some(roots))
    }
}
struct Matrix {
    pivots: Vec<Option<(Vec<u64>, u64)>>,
    r: u64,
    rows: u64,
    ops: u64,
}
impl Matrix {
    fn new(n: usize, r: u64) -> Self {
        Self {
            pivots: vec![None; n],
            r,
            rows: 0,
            ops: 0,
        }
    }
    fn reduce(&mut self, mut v: Vec<u64>, mut rhs: u64, insert: bool) -> Option<u64> {
        for j in 0..v.len() {
            if v[j] == 0 {
                continue;
            }
            if let Some((row, value)) = &self.pivots[j] {
                let scale = v[j];
                for k in j..v.len() {
                    v[k] = (v[k] + self.r - scale * row[k] % self.r) % self.r;
                    self.ops += 1;
                }
                rhs = (rhs + self.r - scale * value % self.r) % self.r;
            } else if insert {
                let inverse = inv(v[j], self.r).unwrap();
                for item in v.iter_mut().skip(j) {
                    *item = *item * inverse % self.r;
                    self.ops += 1;
                }
                rhs = rhs * inverse % self.r;
                self.pivots[j] = Some((v, rhs));
                self.rows += 1;
                return None;
            } else {
                return None;
            }
        }
        Some(rhs)
    }
    fn rank(&self) -> usize {
        self.pivots.iter().filter(|x| x.is_some()).count()
    }
}
fn vector(n: usize, roots: &[usize], r: u64) -> Vec<u64> {
    let mut v = vec![0; n];
    for &j in roots {
        v[j] = (v[j] + 1) % r;
    }
    v
}
fn attack(
    d: &QuadraticDescent,
    p: &Pt,
    q: &Pt,
    r: u64,
    order: u64,
    relation_limit: u64,
    descent_limit: u64,
) -> Result<Value, String> {
    let (mapped, map_cost) = phase(|| Ok((d.map(p)?, d.map(q)?)))?;
    let (gp, gq) = mapped;
    if gp.is_identity()
        || gp
            .try_scalar_mul(&r.into(), &d.curve)
            .map_err(|e| e.to_string())?
            .is_identity()
            == false
    {
        return Err("generator image fails prime-order gate".into());
    }
    if gq
        .try_scalar_mul(&r.into(), &d.curve)
        .map_err(|e| e.to_string())?
        .is_identity()
        == false
    {
        return Err("target image fails prime-order gate".into());
    }
    let (fb, fb_cost) = phase(|| FactorBase::build(d))?;
    let cofactor = order / r;
    let mut matrix = Matrix::new(fb.points.len(), r);
    let (relations, relation_cost) = phase(|| {
        let mut smooth = 0u64;
        let mut nonsmooth = 0u64;
        let mut attempts = 0u64;
        let mut jac_adds = 0u64;
        let mut cur = MumfordDivisor::identity(d.base_degree);
        for k in 1..=relation_limit.min(r - 1) {
            cur = cur.try_add(&gp, &d.curve).map_err(|e| e.to_string())?;
            jac_adds += 1;
            attempts += 1;
            if let Some(roots) = fb.split(d, &cur)? {
                smooth += 1;
                let rhs = (cofactor % r) * (k % r) % r;
                matrix.reduce(vector(fb.points.len(), &roots, r), rhs, true);
            } else {
                nonsmooth += 1;
            }
        }
        Ok(
            json!({"attempts":attempts,"smooth":smooth,"nonsmooth":nonsmooth,"jacobian_additions":jac_adds}),
        )
    })?;
    let (answer, descent_cost) = phase(|| {
        let mut cur = gq.clone();
        let mut jac_adds = 0u64;
        let mut smooth = 0u64;
        for offset in 0..descent_limit.min(r) {
            if let Some(roots) = fb.split(d, &cur)? {
                smooth += 1;
                if let Some(hx) = matrix.reduce(vector(fb.points.len(), &roots, r), 0, false) {
                    // Reduction returns negative of the projected log.
                    let h_log = (r - hx) % r;
                    let scalar = (h_log * inv(cofactor % r, r).ok_or("cofactor not invertible")?
                        + r
                        - offset % r)
                        % r;
                    if d.source.scalar_mul(p, &scalar.into()) == *q {
                        return Ok(
                            json!({"scalar":scalar,"offset":offset,"smooth":smooth,"jacobian_additions":jac_adds}),
                        );
                    }
                }
            }
            cur = cur.try_add(&gp, &d.curve).map_err(|e| e.to_string())?;
            jac_adds += 1;
        }
        Err("individual descent exhausted trial cap or matrix span".into())
    })?;
    Ok(
        json!({"status":"verified","map":d.model_json(),"image_generator":divisor_json(&gp),"image_target":divisor_json(&gq),
        "factor_base_size":fb.points.len(),"matrix_rank":matrix.rank(),"matrix_rows":matrix.rows,"matrix_entry_operations":matrix.ops,
        "relation_counts":relations,"target_descent":answer,"unpriced_operations":{"factor_base_hash_inserts":fb.points.len(),"factor_base_hash_lookups":fb.hash_lookups.get(),"polynomial_root_tests":fb.root_tests.get(),"decomposition_reconstructions":fb.decomposition_checks.get(),"matrix_entry_operations":matrix.ops},
        "phases":{"map":map_cost,"factor_base":fb_cost,"relations_and_linear_algebra":relation_cost,"target_descent_and_replay":descent_cost}}),
    )
}
fn run() -> Result<Value, String> {
    let args = Args::parse();
    let input = GhsCurveInput {
        absolute_degree: args.degree,
        modulus: parse(&args.modulus)?,
        a: parse(&args.a)?,
        b: parse(&args.b)?,
    };
    let context = GeneralizedContext::new(&input)?;
    let source = ECurve::new(
        args.degree,
        context.irr,
        context.a,
        F2mElement::from_biguint(&input.b, args.degree),
    );
    let (all, order_cost) = phase(|| Ok(points(&source)))?;
    let order = all.len() as u64;
    let requested_degree = if args.two_isogeny {
        if args.isogeny_degree != 0 && args.isogeny_degree != 2 {
            return Err("conflicting isogeny degree switches".into());
        }
        2
    } else {
        args.isogeny_degree
    };
    let (edge, edge_cost) = if requested_degree != 0 {
        let (iso, cost) =
            phase(|| TwoIsogeny::construct(source.clone(), &all, requested_degree, context.beta))?;
        (Some(iso), Some(cost))
    } else {
        (None, None)
    };
    let descent_input = if let Some(iso) = &edge {
        GhsCurveInput {
            absolute_degree: args.degree,
            modulus: input.modulus.clone(),
            a: iso.codomain.a.to_biguint(),
            b: iso.codomain.b.to_biguint(),
        }
    } else {
        input.clone()
    };
    let (d, construction_cost) = phase(|| QuadraticDescent::construct(&descent_input))?;
    let r = *primes(order).last().ok_or("no prime factor")?;
    if r < 11 || order / r % r == 0 {
        return Err(format!("largest prime factor {r} is too small or repeated"));
    }
    let p = all
        .iter()
        .map(|x| source.scalar_mul(x, &(order / r).into()))
        .find(|x| {
            if *x == Pt::Inf {
                return false;
            }
            let image = edge.as_ref().map_or_else(|| x.clone(), |iso| iso.image(x));
            image != Pt::Inf && d.map(&image).is_ok_and(|v| !v.is_identity())
        })
        .ok_or("no subgroup generator with nonzero image")?;
    let scalar = args.target_scalar % r;
    if scalar == 0 {
        return Err("target scalar vanishes mod r".into());
    }
    let q = source.scalar_mul(&p, &scalar.into());
    let mapped_p = edge.as_ref().map_or_else(|| p.clone(), |iso| iso.image(&p));
    let mapped_q = edge.as_ref().map_or_else(|| q.clone(), |iso| iso.image(&q));
    if d.source.scalar_mul(&mapped_p, &r.into()) != Pt::Inf
        || d.source.scalar_mul(&mapped_p, &scalar.into()) != mapped_q
    {
        return Err("isogeny subgroup map check failed".into());
    }
    let (subgroup_checked, subgroup_check_cost) = phase(|| {
        let mut source_multiple = Pt::Inf;
        let mut jacobian_multiple = MumfordDivisor::identity(d.base_degree);
        let image_generator = d.map(&mapped_p)?;
        for k in 0..r {
            let transported = edge.as_ref().map_or_else(
                || source_multiple.clone(),
                |iso| iso.image(&source_multiple),
            );
            let image = d.map(&transported)?;
            if image != jacobian_multiple {
                return Err(format!("subgroup transport fails at scalar {k}"));
            }
            source_multiple = source.add(&source_multiple, &p);
            jacobian_multiple = jacobian_multiple
                .try_add(&image_generator, &d.curve)
                .map_err(|e| e.to_string())?;
        }
        if source_multiple != Pt::Inf || !jacobian_multiple.is_identity() {
            return Err("subgroup closure fails after full enumeration".into());
        }
        Ok(r)
    })?;
    let (ghs, ghs_status, ghs_error) = match attack(
        &d,
        &mapped_p,
        &mapped_q,
        r,
        order,
        args.relation_limit,
        args.descent_limit,
    ) {
        Ok(v) => (Some(v), "verified", None),
        Err(error) => (None, "failed", Some(error)),
    };
    let (rho, rho_cost) = phase(|| source_rho(&source, &p, &q, r))?;
    Ok(
        json!({"schema":"ghs-quadratic-attack/v1","source":{"absolute_degree":args.degree,"modulus":args.modulus,"a":args.a,"b":args.b,"group_order":order,"subgroup_order":r,"generator":key(&p),"target":key(&q),"target_scalar_for_replay":scalar},
        "isogeny":edge.as_ref().map(|iso|json!({"degree":iso.kernel.len()+1,"kernel_generator":key(&iso.kernel[0]),"kernel_points":iso.kernel.iter().map(key).collect::<Vec<_>>(),"codomain_a":format!("0x{:x}",iso.codomain.a.to_biguint()),"codomain_b":format!("0x{:x}",iso.codomain.b.to_biguint()),"y_shift":format!("0x{:x}",iso.y_shift.to_biguint()),"formula":"X=x(P)+sum_{T in kernel_except_O}(x(P+T)+x(T)); Y=y(P)+sum_{T in kernel_except_O}(y(P+T)+y(T))+y_shift","image_generator":key(&mapped_p),"image_target":key(&mapped_q),"construction_and_validation_cost":edge_cost,"subgroup_map_verified":true,"subgroup_map_checked_scalars":subgroup_checked,"curve_equation_verified_on_all_rational_points":true,"group_law_checked_pairs":400})),
        "ghs_status":ghs_status,"ghs_error":ghs_error,"ghs":ghs,"rho":{"result":rho,"cost":rho_cost},"setup_phases":{"construction":construction_cost,"source_order_enumeration":order_cost,"full_subgroup_map_check":subgroup_check_cost,"full_subgroup_checked_scalars":subgroup_checked},
        "limits":{"relations":args.relation_limit,"target_descent":args.descent_limit},"same_source_target":true,
        "cost_comparison_status":"uncalibrated_field_degree_and_memory_operations"}),
    )
}
fn main() {
    match run() {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => {
            eprintln!("ghs_quadratic_attack: {e}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artin_schreier_enumeration_matches_curve_equation() {
        for (degree, modulus, a, b) in [(6u32, 0x43u64, 0u64, 32u64), (8, 0x11b, 0, 32)] {
            let input = GhsCurveInput {
                absolute_degree: degree,
                modulus: modulus.into(),
                a: a.into(),
                b: b.into(),
            };
            let ctx = GeneralizedContext::new(&input).unwrap();
            let curve = ECurve::new(degree, ctx.irr, ctx.a, e(b, degree));
            let enumerated = points(&curve);
            let q = 1u64 << degree;
            let brute = 1
                + (0..q)
                    .flat_map(|x| (0..q).map(move |y| (x, y)))
                    .filter(|&(x, y)| {
                        curve.is_on_curve(&Pt::Aff {
                            x: e(x, degree),
                            y: e(y, degree),
                        })
                    })
                    .count();
            assert_eq!(enumerated.len(), brute);
            assert!(enumerated.iter().all(|p| curve.is_on_curve(p)));
            let mut keys: Vec<_> = enumerated.iter().map(key).collect();
            keys.sort_unstable();
            keys.dedup();
            assert_eq!(keys.len(), brute);
        }
    }

    #[test]
    fn negation_rho_recovers_same_source_challenge() {
        for (degree, modulus, a, b, r) in [
            (6u32, 0x43u64, 0u64, 32u64, 19u64),
            (8, 0x11b, 0, 32, 67),
            (10, 0x409, 0, 32, 43),
            (12, 0x1009, 1, 512, 199),
        ] {
            let input = GhsCurveInput {
                absolute_degree: degree,
                modulus: modulus.into(),
                a: a.into(),
                b: b.into(),
            };
            let ctx = GeneralizedContext::new(&input).unwrap();
            let curve = ECurve::new(degree, ctx.irr, ctx.a, e(b, degree));
            let all = points(&curve);
            let p = all
                .iter()
                .map(|x| curve.scalar_mul(x, &((all.len() as u64) / r).into()))
                .find(|x| *x != Pt::Inf)
                .unwrap();
            let scalar = 37u64 % r;
            let q = curve.scalar_mul(&p, &scalar.into());
            let receipt = source_rho(&curve, &p, &q, r).unwrap();
            assert_eq!(receipt["algorithm"], "rho.negation");
            assert_eq!(receipt["scalar"], scalar);
        }
    }

    #[test]
    fn odd_degree_isogenies_preserve_prime_subgroups() {
        for (degree, modulus, a, b, ell, r) in [
            (10u32, 0x409u64, 0u64, 32u64, 3u32, 43u64),
            (12, 0x1009, 1, 512, 5, 199),
        ] {
            let input = GhsCurveInput {
                absolute_degree: degree,
                modulus: modulus.into(),
                a: a.into(),
                b: b.into(),
            };
            let ctx = GeneralizedContext::new(&input).unwrap();
            let curve = ECurve::new(degree, ctx.irr, ctx.a, e(b, degree));
            let all = points(&curve);
            let edge = TwoIsogeny::construct(curve.clone(), &all, ell, ctx.beta).unwrap();
            assert_eq!(edge.kernel.len(), (ell - 1) as usize);
            assert_eq!(all.len(), points(&edge.codomain).len());
            let p = all
                .iter()
                .map(|x| curve.scalar_mul(x, &((all.len() as u64) / r).into()))
                .find(|x| *x != Pt::Inf && edge.image(x) != Pt::Inf)
                .unwrap();
            for scalar in [1u64, 2, 3, 7, 37] {
                assert_eq!(
                    edge.image(&curve.scalar_mul(&p, &scalar.into())),
                    edge.codomain.scalar_mul(&edge.image(&p), &scalar.into())
                );
            }
            assert_eq!(
                edge.codomain.scalar_mul(&edge.image(&p), &r.into()),
                Pt::Inf
            );
        }
    }

    #[test]
    fn degree_two_isogeny_is_additive_on_all_degree_six_pairs() {
        let input = GhsCurveInput {
            absolute_degree: 6,
            modulus: 0x43u32.into(),
            a: 0u32.into(),
            b: 32u32.into(),
        };
        let ctx = GeneralizedContext::new(&input).unwrap();
        let curve = ECurve::new(6, ctx.irr, ctx.a, e(32, 6));
        let all = points(&curve);
        let edge = TwoIsogeny::construct(curve.clone(), &all, 2, ctx.beta).unwrap();
        for p in &all {
            for q in &all {
                assert_eq!(
                    edge.image(&curve.add(p, q)),
                    edge.codomain.add(&edge.image(p), &edge.image(q))
                );
            }
        }
        assert_eq!(all.len(), points(&edge.codomain).len());
    }
}
