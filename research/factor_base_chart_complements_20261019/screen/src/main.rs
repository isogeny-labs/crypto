//! Finite structural census, not a decomposition solver or timing benchmark.
mod arithmetic;
use arithmetic::{hex, prime, Curve, Point, O};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Pair = (usize, usize);
type Witnesses = BTreeMap<Point, Vec<Pair>>;
fn read(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn number(v: &Value, k: &str) -> Result<u64> {
    v[k].as_u64()
        .ok_or_else(|| format!("missing number {k}").into())
}
fn string<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v[k].as_str()
        .ok_or_else(|| format!("missing string {k}").into())
}
fn neg(p: Point) -> Point {
    if p.infinity {
        p
    } else {
        Point { y: p.x ^ p.y, ..p }
    }
}
fn point(v: &Value) -> Result<Point> {
    Ok(Point {
        x: hex(string(v, "x")?)?,
        y: hex(string(v, "y")?)?,
        infinity: false,
    })
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut b = 1;
    while n > 0 {
        if n & 1 != 0 {
            b = b * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    b
}
#[derive(Default)]
struct Rank {
    rows: BTreeMap<usize, Vec<u64>>,
}
impl Rank {
    fn insert(&mut self, mut v: Vec<u64>, p: u64) {
        for (&pivot, row) in &self.rows {
            let a = v[pivot];
            if a != 0 {
                for (x, y) in v.iter_mut().zip(row) {
                    *x = (*x + p - a * y % p) % p;
                }
            }
        }
        if let Some(pivot) = v.iter().position(|x| *x != 0) {
            let inverse = pow(v[pivot], p - 2, p);
            for x in &mut v {
                *x = *x * inverse % p;
            }
            self.rows.insert(pivot, v);
        }
    }
}
#[derive(Clone)]
struct Base {
    slug: String,
    labels: Vec<String>,
    pts: Vec<Point>,
    raw: Value,
}
struct Census {
    c: Curve,
    r: u64,
    h: u64,
    cache: BTreeMap<(Point, Point), (Point, bool)>,
    images: BTreeMap<Point, Point>,
    scalar_checks: u64,
}
impl Census {
    fn sum(&mut self, p: Point, q: Point) -> Result<(Point, bool)> {
        let key = if p <= q { (p, q) } else { (q, p) };
        if let Some(s) = self.cache.get(&key) {
            return Ok(*s);
        }
        for a in [p, q] {
            if !self.images.contains_key(&a) {
                self.images.insert(a, self.c.scale(a, self.r)?);
            }
        }
        let sum = self.c.add(p, q)?;
        let compatible = self.c.add(self.images[&p], self.images[&q])? == O;
        // Every enumerated pair is checked directly, including rejected and identity pairs.
        if compatible != (self.c.scale(sum, self.r)? == O) {
            return Err("cofactor-image criterion disagrees with direct scalar check".into());
        }
        self.scalar_checks += 1;
        self.cache.insert(key, (sum, compatible));
        Ok((sum, compatible))
    }
    fn measure(&mut self, pts: &[Point]) -> Result<(Value, BTreeSet<Point>)> {
        let mut witnesses: Witnesses = BTreeMap::new();
        let mut identity = 0usize;
        let mut compatible = 0usize;
        for i in 0..pts.len() {
            for j in i..pts.len() {
                let (s, ok) = self.sum(pts[i], pts[j])?;
                if s == O {
                    identity += 1;
                } else if ok {
                    compatible += 1;
                    witnesses.entry(s).or_default().push((i, j));
                }
            }
        }
        let cols: BTreeSet<_> = pts.iter().map(|p| (*p).min(neg(*p))).collect();
        let cols: Vec<_> = cols.into_iter().collect();
        let zero_projection: Vec<_> = cols
            .iter()
            .map(|p| self.c.scale(*p, self.h).map(|q| q == O))
            .collect::<Result<_>>()?;
        let map: Vec<_> = pts
            .iter()
            .map(|p| {
                let canonical = (*p).min(neg(*p));
                (
                    cols.binary_search(&canonical).expect("column present"),
                    if *p == canonical { 1 } else { self.r - 1 },
                )
            })
            .collect();
        let vector = |pair: Pair| {
            let mut v = vec![0; cols.len()];
            for i in [pair.0, pair.1] {
                let (k, sign) = map[i];
                v[k] = (v[k] + sign) % self.r;
            }
            v
        };
        let mut rank = Rank::default();
        let mut useful_rank = Rank::default();
        let mut active = BTreeSet::new();
        let mut parent: Vec<usize> = (0..cols.len()).collect();
        fn root(parent: &[usize], mut x: usize) -> usize {
            while parent[x] != x {
                x = parent[x];
            }
            x
        }
        let mut relation_rows = 0;
        let mut targets = Vec::new();
        for (s, ws) in &witnesses {
            let baseline = vector(ws[0]);
            for &pair in ws {
                let (a, b) = (map[pair.0].0, map[pair.1].0);
                active.insert(a);
                active.insert(b);
                let ra = root(&parent, a);
                let rb = root(&parent, b);
                parent[rb] = ra;
                let v = vector(pair);
                if pair != ws[0] {
                    let difference: Vec<_> = v
                        .iter()
                        .zip(&baseline)
                        .map(|(x, y)| (x + self.r - y) % self.r)
                        .collect();
                    useful_rank.insert(
                        difference
                            .iter()
                            .zip(&zero_projection)
                            .filter(|(_, zero)| !**zero)
                            .map(|(x, _)| *x)
                            .collect(),
                        self.r,
                    );
                    rank.insert(difference, self.r);
                    relation_rows += 1;
                }
            }
            targets.push(json!({"target":s,"witnesses":ws}));
        }
        let components: BTreeSet<_> = active.iter().map(|x| root(&parent, *x)).collect();
        let mut fibres: BTreeMap<Point, usize> = BTreeMap::new();
        for p in pts {
            *fibres.entry(self.images[p]).or_default() += 1;
        }
        Ok((
            json!({"columns":cols.len(),"signed_points":pts.len(),"unordered_pairs_with_repetition":pts.len()*(pts.len()+1)/2,
            "identity_pairs":identity,"nonidentity_compatible_pairs":compatible,"support":witnesses.len(),
            "cofactor_fibres":fibres.iter().map(|(p,n)|json!({"image":p,"points":n})).collect::<Vec<_>>(),
            "active_columns":active.len(),"active_pair_graph_components":components.len(),"unused_columns":cols.len()-active.len(),
            "same_target_difference_rows":relation_rows,"same_target_difference_rank_mod_r":rank.rows.len(),
            "zero_prime_projection_columns":zero_projection.iter().filter(|x|**x).count(),
            "same_target_difference_rank_nonzero_projection_columns":useful_rank.rows.len(),
            "points":pts,"column_representatives":cols,"target_witnesses":targets}),
            witnesses.keys().copied().collect(),
        ))
    }
}
fn screen(root: &Path) -> Result<Value> {
    let index = read(&root.join("inputs/index.json"))?;
    let models = read(&root.join("inputs/models.json"))?;
    let expected = read(&root.join("inputs/previous-coverage.json"))?;
    let mut bases: Vec<Base> = Vec::new();
    let mut input_rows = Vec::new();
    for case in index["cases"].as_array().ok_or("cases")? {
        let raw = read(&root.join(string(case, "path")?))?;
        let slug = string(&raw["curve"], "slug")?.to_owned();
        let mut pts: Vec<_> = raw["points"]
            .as_array()
            .ok_or("points")?
            .iter()
            .map(point)
            .collect::<Result<_>>()?;
        pts.sort();
        pts.dedup();
        if pts.is_empty() {
            input_rows.push(
                json!({"label":case["label"],"status":"empty-inventory-retained","base":null}),
            );
            continue;
        }
        let id = if let Some(id) = bases.iter().position(|b| b.slug == slug && b.pts == pts) {
            bases[id].labels.push(string(case, "label")?.to_owned());
            id
        } else {
            let id = bases.len();
            bases.push(Base {
                slug,
                labels: vec![string(case, "label")?.to_owned()],
                pts,
                raw,
            });
            id
        };
        input_rows.push(json!({"label":case["label"],"base":id,"status":"included"}));
    }
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    let mut curve_audits = Vec::new();
    for model in models["curves"].as_array().ok_or("models")? {
        let slug = string(model, "slug")?;
        let ids: Vec<_> = bases
            .iter()
            .enumerate()
            .filter(|(_, b)| b.slug == slug)
            .map(|(i, _)| i)
            .collect();
        if ids.is_empty() {
            continue;
        }
        let n = number(&model["params"], "n")?;
        let a = number(&model["params"], "a")?;
        if !(2..=31).contains(&n) || a > 1 {
            return Err("unsupported model".into());
        }
        let r = number(&bases[ids[0]].raw["curve"], "r")?;
        if !prime(r) {
            return Err("nonprime subgroup".into());
        }
        let mut census = Census {
            c: Curve {
                n: n as u32,
                a,
                modulus: hex(string(&model["params"], "modulus")?)?,
            },
            r,
            h: number(&bases[ids[0]].raw["curve"], "cofactor")?,
            cache: BTreeMap::new(),
            images: BTreeMap::new(),
            scalar_checks: 0,
        };
        for &i in &ids {
            let b = &bases[i];
            let curve = &b.raw["curve"];
            let group = number(curve, "group_order")?;
            if number(curve, "r")? != r
                || group != r * number(curve, "cofactor")?
                || group.is_multiple_of(r * r)
            {
                return Err("inconsistent subgroup metadata".into());
            }
            let gen = &curve["generator"];
            let g = Point {
                x: hex(gen[0].as_str().ok_or("gx")?)?,
                y: hex(gen[1].as_str().ok_or("gy")?)?,
                infinity: false,
            };
            if !census.c.on_curve(g) || census.c.scale(g, r)? != O {
                return Err("generator order".into());
            }
            for &p in &b.pts {
                if !census.c.on_curve(p)
                    || census.c.scale(p, group)? != O
                    || b.pts.binary_search(&neg(p)).is_err()
                {
                    return Err("invalid or unsigned inventory".into());
                }
            }
        }
        let mut supports: BTreeMap<usize, BTreeSet<Point>> = BTreeMap::new();
        for &i in &ids {
            let (metrics, support) = census.measure(&bases[i].pts)?;
            for label in &bases[i].labels {
                let old = expected["rows"]
                    .as_array()
                    .ok_or("coverage rows")?
                    .iter()
                    .find(|x| x["label"] == *label)
                    .ok_or("missing previous coverage label")?;
                if old["prime_target_coverage"].as_u64() != Some(support.len() as u64) {
                    return Err(
                        format!("previous independent coverage mismatch for {label}").into(),
                    );
                }
            }
            supports.insert(i, support);
            rows.push(json!({"id":format!("B{i:03}"),"curve":slug,"r":r,"kind":"single","bases":[i],"labels":bases[i].labels,"metrics":metrics}));
        }
        for (position, &i) in ids.iter().enumerate() {
            for &j in &ids[position + 1..] {
                let pts: Vec<_> = bases[i]
                    .pts
                    .iter()
                    .chain(&bases[j].pts)
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                if pts.len() > 256 {
                    skipped.push(json!({"bases":[i,j],"reason":"256-signed-point-cap","signed_points":pts.len()}));
                    continue;
                }
                let baseline: BTreeSet<_> = supports[&i].union(&supports[&j]).copied().collect();
                let (metrics, support) = census.measure(&pts)?;
                if !baseline.is_subset(&support) {
                    return Err("union lost single support".into());
                }
                let cross_only: Vec<_> = support.difference(&baseline).copied().collect();
                rows.push(json!({"id":format!("B{i:03}+B{j:03}"),"curve":slug,"r":r,"kind":"union","bases":[i,j],"labels":[bases[i].labels,bases[j].labels],"single_support_union":baseline.len(),"new_cross_targets":cross_only.len(),"cross_only_targets":cross_only,"metrics":metrics}));
            }
        }
        curve_audits.push(json!({"curve":slug,"r":r,"unique_pair_direct_scalar_checks":census.scalar_checks,"bases":ids.len()}));
    }
    Ok(
        json!({"schema":"factor-base-chart-screen/v1","source_archive":index["source_archive_sha256"],"input_cases":input_rows,"distinct_nonempty_bases":bases.len(),"rows":rows,"skipped":skipped,"curve_audits":curve_audits,
        "certified_regularity":null,"successful_f4_work":null,"native_speedup":null,"full_pipeline_cost":null,"scope":"finite exhaustive structural census, same-host arithmetic; no held-out confirmation"}),
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 || args[1] != "screen" {
        return Err("usage: factor-base-chart-screen screen STUDY_ROOT OUTPUT_JSON".into());
    }
    let out = Path::new(&args[3]);
    if out.exists() {
        return Err("output already exists".into());
    }
    let result = screen(Path::new(&args[2]))?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out)?;
    use std::io::Write;
    file.write_all(serde_json::to_string_pretty(&result)?.as_bytes())?;
    file.write_all(b"\n")?;
    println!(
        "structural census complete: {} bases, {} cases",
        result["distinct_nonempty_bases"],
        result["rows"].as_array().ok_or("rows")?.len()
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn modular_rank_preserves_dependencies_and_detects_new_pivots() {
        let mut r = Rank::default();
        r.insert(vec![1, 2, 3], 71);
        r.insert(vec![2, 4, 6], 71);
        assert_eq!(r.rows.len(), 1);
        r.insert(vec![0, 1, 1], 71);
        assert_eq!(r.rows.len(), 2);
        r.insert(vec![0, 0, 1], 71);
        assert_eq!(r.rows.len(), 3);
    }
    #[test]
    fn whole_small_curve_checks_fibres_against_direct_subgroup_membership() -> Result<()> {
        let mut c = Census {
            c: Curve {
                n: 3,
                modulus: 0xb,
                a: 1,
            },
            r: 7,
            h: 2,
            cache: BTreeMap::new(),
            images: BTreeMap::new(),
            scalar_checks: 0,
        };
        let mut pts = vec![O];
        for x in 0..8 {
            for y in 0..8 {
                let p = Point {
                    x,
                    y,
                    infinity: false,
                };
                if c.c.on_curve(p) {
                    pts.push(p);
                }
            }
        }
        assert_eq!(pts.len(), 14);
        for &p in &pts {
            for &q in &pts {
                let (s, ok) = c.sum(p, q)?;
                assert_eq!(ok, c.c.scale(s, 7)? == O);
            }
        }
        assert_eq!(c.scalar_checks, 105);
        Ok(())
    }
}
