//! Exact/sampled coupled-component censuses and matched projected-row controls.
use crate::{digest, next, toy_field::*, trace::canonical, write_json, Fixture, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Config {
    product: bool,
    lambda: u64,
    h: u64,
}
impl Config {
    fn name(self) -> String {
        format!(
            "{}-l{}-h{}",
            if self.product { "product" } else { "linear" },
            self.lambda,
            self.h
        )
    }
    fn accepts(self, p: u64, x: u64, u: u64) -> bool {
        if self.product {
            x * u % p < self.h
        } else {
            (x + self.lambda * u) % p < self.h
        }
    }
}
fn configs() -> Vec<Config> {
    let mut c = vec![];
    for h in [2, 4, 8] {
        for lambda in [1, 2, 3] {
            c.push(Config {
                product: false,
                lambda,
                h,
            });
        }
        c.push(Config {
            product: true,
            lambda: 0,
            h,
        });
    }
    c
}
fn seed(id: &str, label: &str) -> u64 {
    let h = digest(format!("twist-followup-20261022-v1|{id}|{label}").as_bytes());
    u64::from_str_radix(&h[..16], 16).unwrap()
}
pub fn public_target(f: &Fixture, experiment: &str, label: u64) -> P {
    let width = f.order - 1;
    let limit = u64::MAX - u64::MAX % width;
    for attempt in 0..128 {
        let k = seed(&f.id, &format!("{experiment}|{label}|{attempt}"));
        if k < limit {
            return f.curve().pm(f.group(), (1 + k % width) as u128);
        }
    }
    panic!("target scalar rejection bound exhausted")
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Image {
    pub fixture: String,
    pub config: String,
    pub product: bool,
    pub lambda: u64,
    pub h: u64,
    pub exhaustive: bool,
    pub point_proposals: u64,
    pub unique_extension_points: usize,
    pub regular_points: u64,
    pub exceptional_origin_or_component_zero: u64,
    pub reference_arithmetic_calls: u64,
    /// Canonical trace point, a concrete extension lift with that trace, multiplicity.
    pub images: Vec<(P, P, u64)>,
}
fn pool(f: &Fixture) -> (Vec<P>, bool, u64) {
    let e = f.curve();
    if [59, 211, 251].contains(&f.p) {
        let p = e.points();
        assert_eq!(p.len() as u64, f.order * (2 * (f.p + 1) - f.order));
        return (p, true, f.p * f.p);
    }
    let mut state = seed(&f.id, "E21-pool");
    let mut points = BTreeSet::new();
    for _ in 0..65536 {
        let x = Q(next(&mut state) % f.p, next(&mut state) % f.p);
        if let Some(y) = e.sqrt(e.rhs(x)) {
            let y = if next(&mut state) & 1 == 0 {
                y
            } else {
                e.neg(y)
            };
            points.insert(Some((x, y)));
        }
    }
    (points.into_iter().collect(), false, 65536)
}
pub fn census(fixtures: &[Fixture], out: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    let mut gp = String::from("default(parisizemax,4000000000);\n");
    gp.push_str("twist_census(p,a,b,d,prod,l,h,expected)={my(E=ellinit([Mod(a,p),Mod(b,p)]),ET=ellinit([Mod(d^2*a,p),Mod(d^3*b,p)]),ts=List(),us=List(),actual=List());for(x=0,p-1,my(ys=ellordinate(E,Mod(x,p)));if(#ys,listput(ts,[x,vecmin(lift(ys)),#ys])));for(u=0,p-1,my(ys=ellordinate(ET,Mod(d*u,p)));if(#ys,listput(us,[u,#ys])));for(i=1,#ts,my(n=0,x=ts[i][1]);for(j=1,#us,my(u=us[j][1]);if(if(prod,(x*u)%p<h,(x+l*u)%p<h),n+=ts[i][3]*us[j][2]));if(n,listput(actual,[x,ts[i][2],n])));if(Vec(actual)!=expected,error(\"coupled image/fiber mismatch\"));};\n");
    gp.push_str("twist_frob(P,p)={if(P==[0],P,[P[1]^p,P[2]^p])};\n");
    for f in fixtures
        .iter()
        .filter(|f| prime(f.order) && f.order % 2 == 1)
    {
        let e = f.curve();
        let before = reference_calls();
        let (ps, exhaustive, proposals) = pool(f);
        let pool_calls = reference_calls() - before;
        let cfgs = configs();
        let mut maps = vec![BTreeMap::<P, (P, u64)>::new(); cfgs.len()];
        let mut regular = 0;
        let mut exceptional = 0;
        let before = reference_calls();
        for p in &ps {
            let t = e.trace(*p);
            let u = e.pa(*p, e.pn(e.frob(*p)));
            assert_eq!(e.frob(t), t);
            assert_eq!(e.frob(u), e.pn(u));
            let (Some((x, _)), Some((v, _))) = (t, u) else {
                exceptional += 1;
                continue;
            };
            assert_eq!(x.1, 0);
            assert_eq!(v.1, 0);
            regular += 1;
            let canon = canonical(e, t);
            let witness = if canon == t { *p } else { e.pn(*p) };
            for (i, c) in cfgs.iter().enumerate() {
                if c.accepts(e.p, x.0, v.0) {
                    let z = maps[i].entry(canon).or_insert((witness, 0));
                    z.0 = z.0.min(witness);
                    z.1 += 1;
                }
            }
        }
        let component_calls = reference_calls() - before;
        for (c, m) in cfgs.into_iter().zip(maps) {
            let image = Image {
                fixture: f.id.clone(),
                config: c.name(),
                product: c.product,
                lambda: c.lambda,
                h: c.h,
                exhaustive,
                point_proposals: proposals,
                unique_extension_points: ps.len(),
                regular_points: regular,
                exceptional_origin_or_component_zero: exceptional,
                reference_arithmetic_calls: pool_calls + component_calls,
                images: m.into_iter().map(|(p, (w, n))| (p, w, n)).collect(),
            };
            for (t, p, _) in &image.images {
                assert_eq!(e.trace(*p), *t);
                let u = e.pa(*p, e.pn(e.frob(*p))).unwrap();
                assert!(c.accepts(e.p, t.unwrap().0 .0, u.0 .0));
            }
            let expected = image
                .images
                .iter()
                .map(|(p, _, n)| {
                    let (x, y) = p.unwrap();
                    format!("[{},{},{}]", x.0, y.0, n)
                })
                .collect::<Vec<_>>()
                .join(",");
            if exhaustive {
                gp.push_str(&format!(
                    "twist_census({},{},{},{},{},{},{},[{}]);\n",
                    f.p,
                    f.a,
                    f.b,
                    f.d,
                    if c.product { 1 } else { 0 },
                    c.lambda,
                    c.h,
                    expected
                ));
            } else {
                gp.push_str(&format!(
                    "p={};z=ffgen(Mod(1,p)*(x^2-{}),'z);E=ellinit([{},{}],z);\n",
                    f.p, f.d, f.a, f.b
                ));
                for (t, w, _) in image.images.iter().take(8) {
                    let (tx, ty) = t.unwrap();
                    let (x, y) = w.unwrap();
                    gp.push_str(&format!("P=[{}+{}*z,{}+{}*z];T=elladd(E,P,twist_frob(P,p));if(T!=[{},{}],error(\"sample trace mismatch\"));U=ellsub(E,P,twist_frob(P,p));if(!({}),error(\"sample predicate mismatch\"));\n",x.0,x.1,y.0,y.1,tx.0,ty.0,if c.product{format!("lift(trace(T[1]*U[1])/2)%p<{}",c.h)}else{format!("lift(trace(T[1]+{}*U[1])/2)%p<{}",c.lambda,c.h)}));
                }
            }
            let file = format!("{}-{}.json", f.id, c.name());
            write_json(out.join(&file), &image)?;
            rows.push(json!({"experiment":"E21","fixture":f.id,"config":c.name(),"file":file,"exhaustive":exhaustive,"distinct_signed_quotient_trace_images":image.images.len(),"distinct_accepted_extension_points":image.images.iter().map(|r|r.2).sum::<u64>(),"regular_points":regular,"exceptional_points":exceptional,"construction_reference_calls":image.reference_arithmetic_calls,"status":"constructed-coupled-image-and-fiber-census","full_E21_complete":false}));
        }
        println!(
            "{}",
            json!({"experiment":"E21","fixture":f.id,"exhaustive":exhaustive,"points":ps.len(),"status":"census-finished"})
        );
    }
    gp.push_str("print(\"ALL_COUPLED_REPLAY_OK\");quit;\n");
    fs::write(out.join("verify.gp"), gp)?;
    Ok(rows)
}
type Row = Vec<i16>;
type Rows = BTreeSet<Row>;
fn signed(e: Quad, base: &[P]) -> Vec<(P, usize, i16)> {
    base.iter()
        .enumerate()
        .flat_map(|(i, p)| [(*p, i, 1), (e.pn(*p), i, -1)])
        .collect()
}
fn direct_rows(e: Quad, base: &[P], arity: usize, target: P) -> (Rows, u64) {
    let s = signed(e, base);
    let lookup = s
        .iter()
        .map(|(p, i, sgn)| (*p, (*i, *sgn)))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Rows::new();
    let mut adds = 0;
    for (p, i, a) in &s {
        if arity == 2 {
            let need = e.pa(target, e.pn(*p));
            adds += 1;
            if let Some((j, b)) = lookup.get(&need) {
                let mut r = vec![0; base.len()];
                r[*i] += a;
                r[*j] += b;
                rows.insert(r);
            }
        } else {
            for (q, j, b) in &s {
                let need = e.pa(target, e.pn(e.pa(*p, *q)));
                adds += 2;
                if let Some((k, c)) = lookup.get(&need) {
                    let mut r = vec![0; base.len()];
                    r[*i] += a;
                    r[*j] += b;
                    r[*k] += c;
                    rows.insert(r);
                }
            }
        }
    }
    (rows, adds)
}
fn auxiliary_index(e: Quad, base: &[P], witnesses: &[P], arity: usize) -> (BTreeMap<P, Rows>, u64) {
    let s = signed(e, witnesses);
    let mut index = BTreeMap::<P, Rows>::new();
    let mut adds = 0;
    for (p, i, a) in &s {
        for (q, j, b) in &s {
            let x = e.pa(*p, *q);
            adds += 1;
            let endings = if arity == 2 {
                vec![(None, 0usize, 0i16)]
            } else {
                s.clone()
            };
            for (z, k, c) in endings {
                let sum = if arity == 2 {
                    x
                } else {
                    adds += 1;
                    e.pa(x, z)
                };
                let t = e.trace(sum);
                adds += 1;
                let mut row = vec![0; base.len()];
                row[*i] += a;
                row[*j] += b;
                if arity == 3 {
                    row[k] += c;
                }
                index.entry(t).or_default().insert(row);
            }
        }
    }
    (index, adds)
}
#[derive(Clone)]
pub struct Linear {
    pub rows: BTreeMap<usize, Vec<u64>>,
    pub r: u64,
    pub ops: u64,
}
impl Linear {
    pub fn new(r: u64) -> Self {
        Self {
            rows: BTreeMap::new(),
            r,
            ops: 0,
        }
    }
    pub fn insert(&mut self, mut row: Vec<u64>) {
        for (&pivot, b) in &self.rows {
            let c = row[pivot];
            if c != 0 {
                for j in pivot..row.len() {
                    row[j] = (row[j] + self.r - c * b[j] % self.r) % self.r;
                    self.ops += 1;
                }
            }
        }
        if let Some(pivot) = row[..row.len() - 1].iter().position(|c| *c != 0) {
            let z = inv(row[pivot], self.r);
            for c in &mut row {
                *c = *c * z % self.r;
                self.ops += 1;
            }
            self.rows.insert(pivot, row);
        } else {
            assert_eq!(
                *row.last().unwrap(),
                0,
                "inconsistent verified relation matrix"
            );
        }
    }
    pub fn value(&mut self, col: usize, width: usize) -> Option<u64> {
        let mut row = vec![0; width + 1];
        row[col] = 1;
        for (&pivot, b) in &self.rows {
            let c = row[pivot];
            for j in pivot..row.len() {
                row[j] = (row[j] + self.r - c * b[j] % self.r) % self.r;
                self.ops += 1;
            }
        }
        if row[..width].iter().all(|x| *x == 0) {
            Some((self.r - row[width]) % self.r)
        } else {
            None
        }
    }
}
fn add_equations(l: &mut Linear, rs: &Rows, width: usize, beta: i16, rhs: u64) {
    for row in rs {
        let mut v = row
            .iter()
            .map(|c| (*c as i64).rem_euclid(l.r as i64) as u64)
            .collect::<Vec<_>>();
        v.push((-(beta as i64)).rem_euclid(l.r as i64) as u64);
        v.push(rhs);
        assert_eq!(v.len(), width + 2);
        l.insert(v);
    }
}
fn recover(
    f: &Fixture,
    base: &[P],
    arity: usize,
    target: P,
    target_rows: &Rows,
) -> (Option<u64>, usize, u64, u64, u64) {
    let e = f.curve();
    let mut l = Linear::new(f.order);
    let before = reference_calls();
    let (zero, nz) = direct_rows(e, base, arity, None);
    let (g, ng) = direct_rows(e, base, arity, f.group());
    let anchor_calls = reference_calls() - before;
    add_equations(&mut l, &zero, base.len(), 0, 0);
    add_equations(&mut l, &g, base.len(), 0, 1);
    add_equations(&mut l, target_rows, base.len(), 1, 0);
    let k = l.value(base.len(), base.len() + 1);
    if let Some(k) = k {
        assert_eq!(
            e.pm(f.group(), k as u128),
            target,
            "recovered scalar does not match target"
        );
    }
    (k, l.rows.len(), l.ops, anchor_calls, nz + ng)
}
#[derive(Default)]
struct Score {
    unsupported: u64,
    unrecovered: u64,
    numerator: u64,
    denominator: u64,
}
pub fn relations(fixtures: &[Fixture], input: &Path, out: &Path) -> Result<Vec<Value>> {
    let mut images = vec![];
    for entry in fs::read_dir(input)? {
        let p = entry?.path();
        if p.extension().is_some_and(|x| x == "json") && p.file_name().unwrap() != "SUMMARY.json" {
            images.push(serde_json::from_slice::<Image>(&fs::read(p)?)?);
        }
    }
    let holdout = |id: &str| {
        fixtures
            .iter()
            .find(|f| f.id == id)
            .unwrap()
            .role
            .ends_with("-1")
    };
    images.sort_by(|a, b| {
        (holdout(&a.fixture), &a.fixture, &a.config).cmp(&(
            holdout(&b.fixture),
            &b.fixture,
            &b.config,
        ))
    });
    let mut rows = vec![];
    let mut cache = BTreeMap::<String, (BTreeMap<P, Rows>, u64, u64)>::new();
    let mut scores = BTreeMap::<String, Score>::new();
    let mut selected: Option<Vec<String>> = None;
    for image in images {
        let f = fixtures.iter().find(|f| f.id == image.fixture).unwrap();
        let e = f.curve();
        let is_holdout = holdout(&f.id);
        if is_holdout && selected.is_none() {
            let mut ranked = scores.keys().cloned().collect::<Vec<_>>();
            ranked.sort_by(|a, b| {
                let x = &scores[a];
                let y = &scores[b];
                (x.unsupported, x.unrecovered)
                    .cmp(&(y.unsupported, y.unrecovered))
                    .then_with(|| {
                        ((x.numerator as u128) * (y.denominator.max(1) as u128))
                            .cmp(&((y.numerator as u128) * (x.denominator.max(1) as u128)))
                    })
                    .then_with(|| a.cmp(b))
            });
            ranked.truncate(2);
            write_json(
                out.join("SELECTION.json"),
                &json!({"schema":"twist.training-selection/v1","selected_configs":ranked,"selected_before_holdout_target_evaluation":true,"rule":"Fewest unsupported training cells, then fewest unrecovered training targets, then smallest cold-model unpriced-reference-call ratio to same-base MITM, then lexical configuration. This is a fixed diagnostic selector, not calibrated cost.","training_scores":scores.iter().map(|(name,s)|json!({"config":name,"unsupported":s.unsupported,"unrecovered":s.unrecovered,"cold_candidate_unpriced_reference_calls":s.numerator,"direct_unpriced_reference_calls":s.denominator})).collect::<Vec<_>>()}),
            )?;
            selected = Some(ranked);
        }
        if is_holdout && !selected.as_ref().unwrap().contains(&image.config) {
            rows.push(json!({"experiment":"E21","fixture":f.id,"config":image.config,"status":"not-evaluated-on-holdout-by-preregistered-selection-rule"}));
            continue;
        }
        for b in [8usize, 12, 16] {
            for arity in [2usize, 3] {
                if image.images.len() < b {
                    if !is_holdout {
                        scores.entry(image.config.clone()).or_default().unsupported += 16;
                    }
                    rows.push(json!({"experiment":"E21","fixture":f.id,"config":image.config,"B":b,"arity":arity,"status":"unsupported-undersized-projected-base"}));
                    continue;
                }
                let base = image.images[..b].iter().map(|r| r.0).collect::<Vec<_>>();
                let witnesses = image.images[..b].iter().map(|r| r.1).collect::<Vec<_>>();
                let key = digest(&serde_json::to_vec(&(
                    f.id.clone(),
                    arity,
                    &base,
                    &witnesses,
                ))?);
                let reused = cache.contains_key(&key);
                let (index, aux_adds, aux_calls) = if let Some(v) = cache.get(&key) {
                    v.clone()
                } else {
                    let c = reference_calls();
                    let (index, n) = auxiliary_index(e, &base, &witnesses, arity);
                    let z = (index, n, reference_calls() - c);
                    cache.insert(key.clone(), z.clone());
                    z
                };
                let mut targets = vec![];
                for label in if is_holdout { 16..32 } else { 0..16 } {
                    let target = public_target(f, "E21", label);
                    let before = reference_calls();
                    let (direct, direct_adds) = direct_rows(e, &base, arity, target);
                    let direct_calls = reference_calls() - before;
                    let projected = index.get(&target).cloned().unwrap_or_default();
                    assert_eq!(
                        projected, direct,
                        "auxiliary projected row set differs from identical-base MITM"
                    );
                    let (scalar, rank, matrix_ops, anchor_calls, anchor_adds) =
                        recover(f, &base, arity, target, &projected);
                    if !is_holdout {
                        let score = scores.entry(image.config.clone()).or_default();
                        score.unrecovered += u64::from(scalar.is_none());
                        score.numerator += image.reference_arithmetic_calls + aux_calls;
                        score.denominator += direct_calls + anchor_calls;
                    }
                    targets.push(json!({"label":label,"target":target,"distinct_projected_rows":projected.len(),"row_sha256":digest(&serde_json::to_vec(&projected)?),"same_base_sets_equal":true,"direct_MITM_group_additions":direct_adds,"direct_MITM_reference_calls":direct_calls,"known_anchor_reference_calls":anchor_calls,"known_anchor_group_additions":anchor_adds,"recovered_scalar":scalar,"scalar_verified":scalar.is_some(),"matrix_rank":rank,"matrix_modular_row_operations":matrix_ops}));
                }
                let record = json!({"experiment":"E21","fixture":f.id,"role":f.role,"config":image.config,"B":b,"arity":arity,"base":base,"witnesses":witnesses,"auxiliary_group_additions":aux_adds,"auxiliary_reference_calls":aux_calls,"constructor_reference_calls_cold":image.reference_arithmetic_calls,"correctness_index_reused":reused,"targets":targets,"status":"same-base-trace-tuple-and-MITM-controls-verified","cost_scope":"Auxiliary tuple index reused for correctness only; its full construction work must be charged to each cold target. Reference calls are unpriced high-level units. No polynomial solver or complete-cost claim.","full_E21_complete":false});
                let file = format!("{}-{}-b{b}-m{arity}.json", f.id, image.config);
                write_json(out.join(&file), &record)?;
                rows.push(json!({"experiment":"E21","fixture":f.id,"config":image.config,"B":b,"arity":arity,"file":file,"index_reused":reused,"targets":16,"status":"verified-identical-projected-row-controls"}));
            }
        }
        println!(
            "{}",
            json!({"experiment":"E21","fixture":f.id,"config":image.config,"stage":"relation-controls-complete"})
        );
    }
    Ok(rows)
}
fn energy(e: Quad, base: &[P]) -> (u64, BTreeMap<P, u64>, BTreeMap<P, u64>) {
    let s = signed(e, base);
    let mut pair = BTreeMap::<P, u64>::new();
    for (p, _, _) in &s {
        for (q, _, _) in &s {
            *pair.entry(e.pa(*p, *q)).or_default() += 1;
        }
    }
    let en = pair.values().map(|n| n * n).sum();
    let mut triple = BTreeMap::<P, u64>::new();
    for (p, n) in &pair {
        for (q, _, _) in &s {
            *triple.entry(e.pa(*p, *q)).or_default() += n;
        }
    }
    assert_eq!(pair.values().sum::<u64>(), (s.len() as u64).pow(2));
    assert_eq!(triple.values().sum::<u64>(), (s.len() as u64).pow(3));
    (en, pair, triple)
}
pub fn energies(fixtures: &[Fixture], input: &Path, out: &Path) -> Result<Vec<Value>> {
    let mut bases = BTreeMap::<String, (Fixture, Vec<P>, Vec<String>)>::new();
    for entry in fs::read_dir(input)? {
        let p = entry?.path();
        if p.extension().is_none_or(|x| x != "json") || p.file_name().unwrap() == "SUMMARY.json" {
            continue;
        }
        let im: Image = serde_json::from_slice(&fs::read(p)?)?;
        let f = fixtures.iter().find(|f| f.id == im.fixture).unwrap();
        for b in [8usize, 12, 16] {
            if im.images.len() >= b {
                let base = im.images[..b].iter().map(|r| r.0).collect::<Vec<_>>();
                let key = digest(&serde_json::to_vec(&(f.id.clone(), &base))?);
                bases
                    .entry(key)
                    .or_insert_with(|| (f.clone(), base, vec![]))
                    .2
                    .push(im.config.clone());
            }
        }
    }
    let mut rows = vec![];
    for (key, (f, base, configs)) in &bases {
        let e = f.curve();
        let (en, pair, triple) = energy(e, base);
        let targets = if [59, 211, 251].contains(&f.p) {
            e.rational()
        } else {
            (0..32).map(|i| public_target(f, "E22", i)).collect()
        };
        let row = json!({"experiment":"E22","fixture":f.id,"base":base,"configs_sharing_this_base":configs,"B":base.len(),"additive_energy_signed_base":en,"pair_histogram_support":pair.len(),"triple_histogram_support":triple.len(),"target_yields":targets.iter().map(|p|json!({"target":p,"pair":pair.get(p).unwrap_or(&0),"triple":triple.get(p).unwrap_or(&0)})).collect::<Vec<_>>(),"fiber_duplication_control":{"copies":2,"same_distinct_base":true,"weighted_energy_if_duplicated":16*en,"new_independent_rows_from_duplicates":0},"status":"exact-group-energy-diagnostic","diagnostic_uses_known_logs":false,"performance_claim":false});
        write_json(out.join(format!("base-{key}.json")), &row)?;
        rows.push(json!({"experiment":"E22","fixture":f.id,"B":base.len(),"key":key,"energy":en,"matched_random":false,"status":"energy-diagnostic-recorded"}));
    }
    for f in fixtures
        .iter()
        .filter(|f| prime(f.order) && f.order % 2 == 1)
    {
        let e = f.curve();
        let available = e
            .rational()
            .into_iter()
            .filter(|p| p.is_some() && canonical(e, *p) == *p)
            .collect::<Vec<_>>();
        for b in [8usize, 12, 16] {
            if available.len() < b {
                continue;
            }
            for random_seed in 0..32 {
                let mut ps = available.clone();
                let mut state = seed(&f.id, &format!("E22-random|{b}|{random_seed}"));
                for i in (1..ps.len()).rev() {
                    let j = (next(&mut state) % (i as u64 + 1)) as usize;
                    ps.swap(i, j);
                }
                ps.truncate(b);
                ps.sort();
                let (en, pair, triple) = energy(e, &ps);
                let label = format!("{}-b{b}-s{random_seed}", f.id);
                let targets = if [59, 211, 251].contains(&f.p) {
                    e.rational()
                } else {
                    (0..32).map(|i| public_target(f, "E22", i)).collect()
                };
                let row = json!({"experiment":"E22","fixture":f.id,"B":b,"random_seed":random_seed,"base":ps,"additive_energy_signed_base":en,"pair_histogram_support":pair.len(),"triple_histogram_support":triple.len(),"target_yields":targets.iter().map(|p|json!({"target":p,"pair":pair.get(p).unwrap_or(&0),"triple":triple.get(p).unwrap_or(&0)})).collect::<Vec<_>>(),"status":"matched-random-base-energy-control","diagnostic_uses_known_logs":false,"performance_claim":false});
                write_json(out.join(format!("random-{label}.json")), &row)?;
                rows.push(json!({"experiment":"E22","fixture":f.id,"B":b,"random_seed":random_seed,"energy":en,"matched_random":true,"status":"energy-diagnostic-recorded"}));
            }
        }
    }
    Ok(rows)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn row_elimination_recovers_a_target_without_full_factor_rank() {
        let mut l = Linear::new(7);
        l.insert(vec![1, 1, 3]);
        l.insert(vec![1, 0, 1]);
        assert_eq!(l.value(1, 2), Some(2));
    }
    #[test]
    fn auxiliary_trace_rows_match_mitm_with_nonzero_twist_parts() {
        let e = Quad::new(7, 1, 1);
        let ps = e.points();
        let mut lifts = BTreeMap::new();
        for p in ps {
            let t = e.trace(p);
            if t.is_some() && e.pa(p, e.pn(e.frob(p))).is_some() {
                let t = canonical(e, t);
                lifts
                    .entry(t)
                    .or_insert(if e.trace(p) == t { p } else { e.pn(p) });
            }
        }
        let base = lifts.keys().copied().take(3).collect::<Vec<_>>();
        let ws = base.iter().map(|p| lifts[p]).collect::<Vec<_>>();
        for m in [2, 3] {
            let (index, _) = auxiliary_index(e, &base, &ws, m);
            for p in e.rational() {
                assert_eq!(
                    index.get(&p).cloned().unwrap_or_default(),
                    direct_rows(e, &base, m, p).0
                );
            }
        }
    }
}
