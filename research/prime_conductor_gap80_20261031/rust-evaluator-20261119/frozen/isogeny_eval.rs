//! Complete square-root Velu and twist transport in Rust.
//! Public inputs only. Existing Montgomery/Jacobian arithmetic is included from
//! its repository source; FLINT primitives are loaded through the small C ABI.
#[path = "isogeny_eval/flint.rs"]
mod flint;
// Treat the unchanged included arithmetic as the existing library dependency:
// its two indexed Montgomery loops retain their original implementation.
#[allow(dead_code, clippy::needless_range_loop)]
#[path = "../../ecc/mont_vt.rs"]
mod mont;

use flint::{Api, Context, Poly};
use mont::{Jac, Mont};
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::json;
use std::path::Path;
use std::rc::Rc;

type F = [u64; 4];
type Point = Option<(F, F)>;
type Jet = [Poly; 4];

fn require(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn decimal(s: &str) -> Result<BigUint, String> {
    require(
        !s.is_empty()
            && (s.len() == 1 || !s.starts_with('0'))
            && s.bytes().all(|b| b.is_ascii_digit()),
        "noncanonical unsigned decimal",
    )?;
    BigUint::parse_bytes(s.as_bytes(), 10).ok_or_else(|| "invalid decimal".into())
}

struct Field {
    p: BigUint,
    f: Mont<4>,
    exponent: BigUint,
}
impl Field {
    fn new(p: BigUint) -> Result<Self, String> {
        let f = Mont::<4>::new(&p).ok_or("field outside four-limb range")?;
        let exponent = &p - 2u8;
        Ok(Self { p, f, exponent })
    }
    fn enter(&self, n: &BigUint) -> F {
        self.f.enter(&(n % &self.p)).expect("reduced field value")
    }
    fn small(&self, n: u64) -> F {
        self.enter(&BigUint::from(n))
    }
    fn add(&self, a: F, b: F) -> F {
        self.f.add(&a, &b)
    }
    fn sub(&self, a: F, b: F) -> F {
        self.f.sub(&a, &b)
    }
    fn neg(&self, a: F) -> F {
        self.sub([0; 4], a)
    }
    fn mul(&self, a: F, b: F) -> F {
        self.f.mul(&a, &b)
    }
    fn square(&self, a: F) -> F {
        self.mul(a, a)
    }
    fn inverse(&self, a: F) -> Result<F, String> {
        require(a != [0; 4], "inverse of zero")?;
        Ok(self.f.pow(&a, &self.exponent))
    }
    fn leave(&self, a: F) -> BigUint {
        self.f.leave(&a)
    }
    fn point(&self, x: &str, y: &str) -> Result<Point, String> {
        let x = decimal(x)?;
        let y = decimal(y)?;
        require(x < self.p && y < self.p, "noncanonical point coordinates")?;
        Ok(Some((self.enter(&x), self.enter(&y))))
    }
    fn on(&self, p: Point, a: F, b: F) -> bool {
        p.is_none_or(|(x, y)| {
            self.square(y) == self.add(self.add(self.mul(self.square(x), x), self.mul(a, x)), b)
        })
    }
    fn affine(&self, p: Option<Jac<4>>) -> Result<Point, String> {
        let Some(p) = p else { return Ok(None) };
        if p.z == [0; 4] {
            return Ok(None);
        }
        let zi = self.inverse(p.z)?;
        let z2 = self.square(zi);
        Ok(Some((self.mul(p.x, z2), self.mul(self.mul(p.y, z2), zi))))
    }
    fn lift(&self, p: Point) -> Option<Jac<4>> {
        p.map(|(x, y)| Jac {
            x,
            y,
            z: self.f.one(),
        })
    }
    fn mixed(&self, p: Option<Jac<4>>, q: Point, a: F) -> Option<Jac<4>> {
        match (p, q) {
            (_, None) => p,
            (None, Some(_)) => self.lift(q),
            (Some(p), Some((x, y))) => self.f.add_affine(&p, &x, &y, &a),
        }
    }
    fn add_point(&self, p: Point, q: Point, a: F) -> Result<Point, String> {
        self.affine(self.mixed(self.lift(p), q, a))
    }
    fn multiply(&self, p: Point, n: &BigUint, a: F) -> Result<Point, String> {
        let mut current = None;
        for bit in (0..n.bits()).rev() {
            current = current.and_then(|p| self.f.double(&p, &a));
            if n.bit(bit) {
                current = self.mixed(current, p, a);
            }
        }
        self.affine(current)
    }
    fn encoded(&self, p: Point) -> serde_json::Value {
        p.map_or(serde_json::Value::Null, |(x, y)| {
            json!([self.leave(x).to_string(), self.leave(y).to_string()])
        })
    }
}

#[derive(Default)]
struct Walk {
    inverse_calls: u128,
    additions: u128,
    points: u128,
    peak: usize,
}
impl Field {
    fn walk_x(
        &self,
        start: Point,
        step: Point,
        a: F,
        count: usize,
        batch: usize,
        diagnostic: &mut Walk,
    ) -> Result<Vec<F>, String> {
        require(
            (1..=65536).contains(&batch),
            "projective batch outside1..65536",
        )?;
        let mut result = Vec::with_capacity(count);
        let mut current = self.lift(start);
        while result.len() < count {
            let size = batch.min(count - result.len());
            let mut xs = Vec::with_capacity(size);
            let mut zs = Vec::with_capacity(size);
            let mut prefix = Vec::with_capacity(size);
            let mut acc = self.f.one();
            for k in 0..size {
                let p = current.ok_or("projective walk infinity")?;
                require(p.z != [0; 4], "projective walk infinity")?;
                xs.push(p.x);
                zs.push(p.z);
                prefix.push(acc);
                acc = self.mul(acc, p.z);
                if result.len() + k + 1 < count {
                    current = self.mixed(current, step, a);
                    diagnostic.additions += 1;
                }
            }
            let mut inv = self.inverse(acc)?;
            diagnostic.inverse_calls += 1;
            for k in (0..size).rev() {
                let zi = self.mul(inv, prefix[k]);
                inv = self.mul(inv, zs[k]);
                xs[k] = self.mul(xs[k], self.square(zi));
            }
            result.extend(xs);
            diagnostic.points += size as u128;
            diagnostic.peak = diagnostic.peak.max(size);
        }
        Ok(result)
    }
}

fn zero_jet(ctx: &Rc<Context>) -> Jet {
    std::array::from_fn(|_| Poly::zero(ctx.clone()))
}
fn one_jet(ctx: &Rc<Context>) -> Jet {
    let mut j = zero_jet(ctx);
    j[0] = Poly::constant(ctx.clone(), &BigUint::one());
    j
}
fn jet_product(a: &Jet, b: &Jet, ctx: &Rc<Context>) -> Jet {
    let mut out = zero_jet(ctx);
    for k in 0..4 {
        for i in 0..=k {
            out[k] = out[k].add(&a[i].mul(&b[k - i]));
        }
    }
    out
}
fn ingest(bins: &mut Vec<Option<Jet>>, mut value: Jet, ctx: &Rc<Context>) {
    let mut i = 0;
    while i < bins.len() && bins[i].is_some() {
        value = jet_product(&bins[i].take().expect("present"), &value, ctx);
        i += 1;
    }
    if i == bins.len() {
        bins.push(None)
    }
    bins[i] = Some(value);
}
fn finish(bins: &mut [Option<Jet>], ctx: &Rc<Context>) -> Jet {
    let mut out = one_jet(ctx);
    for value in bins.iter_mut().rev().filter_map(Option::take) {
        out = jet_product(&out, &value, ctx)
    }
    out
}

struct Norm {
    h: Poly,
    hi: Poly,
    weights: Poly,
}
impl Norm {
    fn new(h: Poly) -> Result<Self, String> {
        let n = h.len().checked_sub(1).ok_or("zero norm modulus")?;
        require(
            n > 0 && h.coefficient(n)? == BigUint::one(),
            "norm modulus must be monic nonconstant",
        )?;
        let hi = h.reverse(n + 1).inverse_series(n);
        let weights = h.derivative().reverse(n).mullow(&hi, n);
        Ok(Self { h, hi, weights })
    }
    fn evaluate(&self, j: &Jet, field: &Field) -> Result<Option<[F; 3]>, String> {
        if self.h.gcd(&j[0]).len() > 1 {
            return Ok(None);
        }
        let gi = j[0].inverse_mod(&self.h)?;
        let u = j[1].mulmod(&gi, &self.h);
        let v = j[2].mulmod(&gi, &self.h);
        let w = j[3].mulmod(&gi, &self.h);
        let u2 = u.mulmod_preinv(&u, &self.h, &self.hi);
        let u3 = u2.mulmod_preinv(&u, &self.h, &self.hi);
        let uv = u.mulmod_preinv(&v, &self.h, &self.hi);
        let half = field.leave(field.inverse(field.small(2))?);
        let third = field.leave(field.inverse(field.small(3))?);
        let c2 = v.sub(&u2.scaled(&half));
        let c3 = w.sub(&uv).add(&u3.scaled(&third));
        Ok(Some([
            field.enter(&u.dot(&self.weights)?),
            field.enter(&c2.dot(&self.weights)?),
            field.enter(&c3.dot(&self.weights)?),
        ]))
    }
}

struct Coverage {
    b: usize,
    g: usize,
    block: usize,
    i: usize,
    j: usize,
    covered: BigUint,
    left: BigUint,
    expected_left: BigUint,
}
impl Coverage {
    fn new(b: usize, g: usize, block: usize, n: &BigUint) -> Result<Self, String> {
        let rectangular = BigUint::from(b) * g * 2u8;
        require(
            b > 0 && g > 0 && block > 0 && rectangular <= *n,
            "invalid coverage geometry",
        )?;
        Ok(Self {
            b,
            g,
            block,
            i: 0,
            j: 0,
            covered: BigUint::zero(),
            left: BigUint::zero(),
            expected_left: n - rectangular,
        })
    }
    fn tile(&mut self, i: usize, ni: usize, j: usize, nj: usize) -> Result<(), String> {
        require(
            self.i < self.b && i == self.i && j == self.j,
            "coverage gap/duplicate/order",
        )?;
        require(
            ni == self.block.min(self.b - self.i) && nj == self.block.min(self.g - self.j),
            "coverage extent",
        )?;
        self.covered += BigUint::from(ni) * nj * 2u8;
        self.j += nj;
        if self.j == self.g {
            self.j = 0;
            self.i += ni;
        }
        Ok(())
    }
    fn leftover(&mut self, ordinal: &BigUint) -> Result<(), String> {
        require(
            self.i == self.b
                && self.j == 0
                && ordinal == &self.left
                && self.left < self.expected_left,
            "leftover coverage",
        )?;
        self.left += 1u8;
        Ok(())
    }
    fn complete(&self, n: &BigUint) -> Result<(), String> {
        require(
            self.i == self.b
                && self.j == 0
                && self.left == self.expected_left
                && &self.covered + &self.left == *n,
            "incomplete coverage; assembly refused",
        )
    }
}

#[derive(Default)]
struct Work {
    baby_products: u128,
    baby_starts: u128,
    baby_additions: u128,
    giant_starts: u128,
    giant_additions: u128,
    leftover_starts: u128,
    leftover_additions: u128,
    tiles: u128,
    norm_calls: u128,
    prepared_calls: u128,
    walk: Walk,
}
struct Input {
    field: Field,
    a: F,
    b: F,
    ell: BigUint,
    k: Point,
    points: Vec<Point>,
    twist: F,
}
fn parse_input(text: &str, twist: &str, api: &Api) -> Result<Input, String> {
    let mut words = text.split_whitespace();
    let mut next = || words.next().ok_or("missing case header".to_string());
    let p = decimal(next()?)?;
    require(
        p > BigUint::from(3u8) && p.bits() <= 256,
        "field outside5..2^256",
    )?;
    require(api.is_prime(&p), "nonprime field modulus")?;
    let aa = decimal(next()?)?;
    let bb = decimal(next()?)?;
    let ell = decimal(next()?)?;
    require(aa < p && bb < p, "noncanonical source coefficients")?;
    require(
        ell >= BigUint::from(5u8) && ell != p && api.is_prime(&ell),
        "invalid or nonprime degree",
    )?;
    let field = Field::new(p)?;
    let a = field.enter(&aa);
    let b = field.enter(&bb);
    let twist = decimal(twist)?;
    require(
        !twist.is_zero() && twist < field.p,
        "noncanonical or zero twist scale",
    )?;
    let twist = field.enter(&twist);
    let kx = next()?;
    let ky = next()?;
    let k = field.point(kx, ky)?;
    let count = decimal(next()?)?.to_usize().ok_or("point count width")?;
    require(count <= 64, "at most64 point inputs")?;
    let mut points = Vec::with_capacity(count);
    for _ in 0..count {
        let x = words.next().ok_or("missing point input")?;
        points.push(if x == "O" {
            None
        } else {
            field.point(x, words.next().ok_or("missing point ordinate")?)?
        });
    }
    require(words.next().is_none(), "extra input after declared points")?;
    require(
        field.add(
            field.mul(field.small(4), field.mul(field.square(a), a)),
            field.mul(field.small(27), field.square(b)),
        ) != [0; 4],
        "singular source",
    )?;
    Ok(Input {
        field,
        a,
        b,
        ell,
        k,
        points,
        twist,
    })
}

fn construct(
    input: &Input,
    ctx: Rc<Context>,
    block: usize,
    max_window: usize,
    baby_window: usize,
    batch: usize,
) -> Result<serde_json::Value, String> {
    let Input {
        field: f,
        a,
        b,
        ell,
        k,
        points,
        twist,
    } = input;
    let (a, b, k, twist) = (*a, *b, *k, *twist);
    require(
        block > 0 && max_window > 0 && (1..=65536).contains(&batch),
        "invalid block/window/batch",
    )?;
    let n = (ell - 1u8) / 2u8;
    let mut bs = (&n / 2u8).sqrt().max(BigUint::one());
    if baby_window != 0 {
        bs = BigUint::from(baby_window);
        require(&bs * 2u8 <= n, "baby window outside coverage range")?;
    }
    let bcount = bs.to_usize().ok_or("baby index width")?;
    let gcount = (&n / (&bs * 2u8)).to_usize().ok_or("giant index width")?;
    require(
        gcount > 0 && bcount <= max_window && gcount <= max_window,
        "window exceeds declared limit; assembly refused",
    )?;
    require(
        f.on(k, a, b) && k.is_some() && f.multiply(k, ell, a)?.is_none(),
        "kernel witness",
    )?;
    let untwist = f.inverse(twist)?;
    let t2 = f.square(twist);
    let t3 = f.mul(t2, twist);
    let sourcea = f.mul(t2, a);
    let sourceb = f.mul(t3, b);
    for &p in points {
        require(f.on(p, sourcea, sourceb), "source membership")?;
    }
    let mut coverage = Coverage::new(bcount, gcount, block, &n)?;
    let mut work = Work::default();
    let mut sums = vec![[[0; 4]; 3]; points.len() + 1];
    let mut kernel: Vec<bool> = points.iter().map(Option::is_none).collect();
    let constant = |x: F| Poly::constant(ctx.clone(), &f.leave(x));
    let mut z = Poly::zero(ctx.clone());
    z.set_coefficient(1, &BigUint::one());
    let baby_step = f.multiply(k, &BigUint::from(2u8), a)?;
    let giant_step = f.multiply(k, &(BigUint::from(bcount) * 4u8), a)?;
    let two = f.small(2);
    let three = f.small(3);
    let four = f.small(4);
    let twelve = f.small(12);
    for i in (0..bcount).step_by(block) {
        let ni = block.min(bcount - i);
        let start = f.multiply(k, &(BigUint::from(i) * 2u8 + 1u8), a)?;
        work.baby_starts += 1;
        let roots = f.walk_x(start, baby_step, a, ni, batch, &mut work.walk)?;
        work.baby_additions += ni.saturating_sub(1) as u128;
        let roots: Vec<BigUint> = roots.into_iter().map(|x| f.leave(x)).collect();
        let norm = Norm::new(Poly::roots(ctx.clone(), &roots)?)?;
        work.baby_products += 1;
        for j in (0..gcount).step_by(block) {
            let nj = block.min(gcount - j);
            let mut bins: Vec<Vec<Option<Jet>>> =
                (0..points.len() + 1).map(|_| Vec::new()).collect();
            let start = f.multiply(k, &((BigUint::from(j) * 4u8 + 2u8) * bcount), a)?;
            work.giant_starts += 1;
            for x in f.walk_x(start, giant_step, a, nj, batch, &mut work.walk)? {
                let zm = z.sub(&constant(x));
                let zp = z.add(&constant(x));
                let xz = z.scaled(&f.leave(x));
                let f0 = zm.mul(&zm);
                let f1 = xz
                    .add(&constant(a))
                    .mul(&zp)
                    .scaled(&f.leave(f.neg(two)))
                    .sub(&constant(f.mul(four, b)));
                let xa = xz.sub(&constant(a));
                let f2 = xa.mul(&xa).sub(&zp.scaled(&f.leave(f.mul(four, b))));
                let mut jet = zero_jet(&ctx);
                jet[0] = f0.clone();
                jet[1] = f1.clone();
                jet[2] = f2.clone();
                ingest(&mut bins[0], jet, &ctx);
                for q in 0..points.len() {
                    if !kernel[q] {
                        let alpha = f.mul(points[q].expect("nonzero").0, untwist);
                        let mut jet = zero_jet(&ctx);
                        jet[0] = f0
                            .scaled(&f.leave(f.square(alpha)))
                            .add(&f1.scaled(&f.leave(alpha)))
                            .add(&f2);
                        jet[1] = f0.scaled(&f.leave(f.mul(two, alpha))).add(&f1);
                        jet[2] = f0.clone();
                        ingest(&mut bins[q + 1], jet, &ctx);
                    }
                }
            }
            work.giant_additions += nj.saturating_sub(1) as u128;
            for q in 0..sums.len() {
                if q == 0 || !kernel[q - 1] {
                    work.norm_calls += 1;
                    let value = norm.evaluate(&finish(&mut bins[q], &ctx), f)?;
                    work.prepared_calls += 1;
                    if let Some(value) = value {
                        for (s, v) in sums[q].iter_mut().zip(value) {
                            *s = f.add(*s, v);
                        }
                    } else {
                        require(q > 0, "infinity-jet collision")?;
                        kernel[q - 1] = true;
                    }
                }
            }
            coverage.tile(i, ni, j, nj)?;
            work.tiles += 1;
        }
    }
    let mut moments = [
        f.neg(sums[0][0]),
        f.neg(f.mul(two, sums[0][1])),
        f.neg(f.mul(three, sums[0][2])),
    ];
    let mut index = BigUint::from(bcount) * gcount * 4u8 + 1u8;
    let mut ordinal = BigUint::zero();
    let mut leftover = if &index < ell {
        work.leftover_starts += 1;
        f.multiply(k, &index, a)?
    } else {
        None
    };
    let half = f.inverse(two)?;
    let third = f.inverse(three)?;
    while &index < ell {
        let x = leftover.ok_or("leftover infinity")?.0;
        let x2 = f.square(x);
        moments[0] = f.add(moments[0], x);
        moments[1] = f.add(moments[1], x2);
        moments[2] = f.add(moments[2], f.mul(x2, x));
        for q in 0..points.len() {
            if !kernel[q] {
                let alpha = f.mul(points[q].expect("nonzero").0, untwist);
                if alpha == x {
                    kernel[q] = true;
                    continue;
                }
                let r = f.inverse(f.sub(alpha, x))?;
                let r2 = f.square(r);
                sums[q + 1][0] = f.add(sums[q + 1][0], r);
                sums[q + 1][1] = f.sub(sums[q + 1][1], f.mul(r2, half));
                sums[q + 1][2] = f.add(sums[q + 1][2], f.mul(f.mul(r2, r), third));
            }
        }
        coverage.leftover(&ordinal)?;
        ordinal += 1u8;
        index += 2u8;
        if &index < ell {
            leftover = f.add_point(leftover, baby_step, a)?;
            work.leftover_additions += 1;
        }
    }
    coverage.complete(&n)?;
    let nf = f.enter(&n);
    let ef = f.enter(ell);
    let ca = f.mul(
        t2,
        f.sub(
            a,
            f.mul(f.small(10), f.add(f.mul(three, moments[1]), f.mul(a, nf))),
        ),
    );
    let cb = f.mul(
        t3,
        f.sub(
            b,
            f.mul(
                f.small(14),
                f.add(
                    f.add(
                        f.mul(f.small(5), moments[2]),
                        f.mul(f.mul(three, a), moments[0]),
                    ),
                    f.mul(f.mul(two, b), nf),
                ),
            ),
        ),
    );
    let mut images = Vec::with_capacity(points.len());
    for q in 0..points.len() {
        let image = if kernel[q] {
            None
        } else {
            let (px, y) = points[q].expect("nonzero");
            let x = f.mul(px, untwist);
            let d = sums[q + 1];
            let fx = f.add(f.add(f.mul(f.square(x), x), f.mul(a, x)), b);
            let df = f.add(f.mul(three, f.square(x)), a);
            let hx = f.mul(
                twist,
                f.sub(
                    f.sub(
                        f.sub(f.mul(ef, x), f.mul(two, moments[0])),
                        f.mul(f.mul(two, df), d[0]),
                    ),
                    f.mul(f.mul(f.small(8), fx), d[1]),
                ),
            );
            let hy = f.mul(
                y,
                f.sub(
                    f.sub(
                        f.sub(ef, f.mul(f.mul(twelve, x), d[0])),
                        f.mul(f.mul(twelve, df), d[1]),
                    ),
                    f.mul(f.mul(f.small(24), fx), d[2]),
                ),
            );
            Some((hx, hy))
        };
        require(f.on(image, ca, cb), "destination membership")?;
        images.push(json!({"input":f.encoded(points[q]),"image":f.encoded(image)}));
    }
    Ok(
        json!({"status":"complete_evaluation","implementation":"rust_complete_twist_v1","p":f.p.to_string(),"source":[f.leave(sourcea).to_string(),f.leave(sourceb).to_string()],"ell":ell.to_string(),"auxiliary_kernel":f.encoded(k),"twist_scale":f.leave(twist).to_string(),"auxiliary_source":[f.leave(a).to_string(),f.leave(b).to_string()],"codomain":[f.leave(ca).to_string(),f.leave(cb).to_string()],"baby_count":bcount,"giant_count":gcount,"block":block,"max_window":max_window,"kernel_pairs_covered":n.to_string(),"images":images,
       "work_counts":{"baby_products":work.baby_products.to_string(),"baby_scalar_starts":work.baby_starts.to_string(),"baby_recurrence_additions":work.baby_additions.to_string(),"giant_scalar_starts":work.giant_starts.to_string(),"giant_recurrence_additions":work.giant_additions.to_string(),"leftover_scalar_starts":work.leftover_starts.to_string(),"leftover_recurrence_additions":work.leftover_additions.to_string(),"rectangles":work.tiles.to_string(),"lognorm_calls":work.norm_calls.to_string()},
       "walk_backend":{"name":"rust_existing_mont_jacobian","batch_cap":batch,"batch_inverse_calls":work.walk.inverse_calls.to_string(),"projective_additions":work.walk.additions.to_string(),"projective_points":work.walk.points.to_string(),"peak_batch_points":work.walk.peak},
       "norm_backend":{"name":"rust_prepared_flint_primitives_v1","modulus_preparations":work.baby_products.to_string(),"evaluations":work.prepared_calls.to_string(),"decimal_ffi_marshalling":true},"ordinary_status":null,"conductor_gap":null,"runtime_measurements":null,"cost_accounting":"native arithmetic and FFI conversion work unpriced; no performance admission"}),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn run(
    input: &Path,
    out: &Path,
    library: &Path,
    expected_sha: &str,
    twist: &str,
    max_window: usize,
    block: usize,
    baby_window: usize,
    batch: usize,
    lock: &str,
) -> Result<(), String> {
    let api = Api::load(library, expected_sha)?;
    let text = std::fs::read_to_string(input).map_err(|e| e.to_string())?;
    let input = parse_input(&text, twist, &api)?;
    let _lock = crypto_lib::cryptanalysis::ecbench::isolation::BenchLock::acquire(lock, false)?;
    // Refuse overwriting output before any complete construction is started.
    require(!out.exists(), "fresh output path required")?;
    let ctx = Context::new(api, &input.field.p)?;
    let mut value = construct(&input, ctx, block, max_window, baby_window, batch)?;
    value["native_library_sha256"] = json!(expected_sha);
    let (binary, _) =
        crypto_lib::store::hash_file(&std::env::current_exe().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    value["binary_sha256"] = json!(binary);
    let mut stream = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(out)
        .map_err(|e| e.to_string())?;
    use std::io::Write;
    serde_json::to_writer(&mut stream, &value).map_err(|e| e.to_string())?;
    stream.write_all(b"\n").map_err(|e| e.to_string())?;
    println!("complete Rust evaluation -> {}", out.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn coverage_rejects_incomplete_duplicate_extent_and_leftover() {
        let n = BigUint::from(9u8);
        let mut c = Coverage::new(2, 2, 1, &n).unwrap();
        assert!(c.complete(&n).is_err());
        assert!(c.tile(0, 1, 1, 1).is_err());
        assert!(c.tile(0, 2, 0, 1).is_err());
        c.tile(0, 1, 0, 1).unwrap();
        assert!(c.tile(0, 1, 0, 1).is_err());
        c.tile(0, 1, 1, 1).unwrap();
        c.tile(1, 1, 0, 1).unwrap();
        c.tile(1, 1, 1, 1).unwrap();
        assert!(c.complete(&n).is_err());
        assert!(c.leftover(&BigUint::one()).is_err());
        c.leftover(&BigUint::zero()).unwrap();
        c.complete(&n).unwrap();
    }
    #[test]
    fn canonical_decimal_guards_reject_signs_and_aliases() {
        for s in ["", "+1", "-1", "01", "1e3", " 1"] {
            assert!(decimal(s).is_err(), "{s}");
        }
        assert_eq!(decimal("0").unwrap(), BigUint::zero());
    }
}
