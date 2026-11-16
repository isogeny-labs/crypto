//! Common m3 output chart for RR and F. No solver or root-completeness claim:
//! callers supply rational solver assignments, which are checked before use.
use super::*;
use std::collections::BTreeSet;

struct Query {
    cv: Curve,
    kind: Kind,
    t: usize,
    scale: u64,
    terminal: u64,
    target: (u64, u64),
    domain: Vec<u64>, // complete tower root set, not planted support
}

impl Query {
    fn from_summary(s: &Value) -> Result<Self, String> {
        let i = &s["instance"];
        let get = |key: &str| {
            i[key]
                .as_u64()
                .ok_or_else(|| format!("missing instance {key}"))
        };
        if s["schema"] != "p1tool.emit/v1" || get("m")? != 3 {
            return Err("extract requires p1tool m3 metadata".into());
        }
        let p = get("p")?;
        if p <= 3 || p >= 1u64 << 32 || !is_prime(p) {
            return Err("invalid prime".into());
        }
        let f = F::new(p);
        let t = get("t")? as usize;
        if !(2..=10).contains(&t) {
            return Err("extract supports 2 <= t <= 10".into());
        }
        let kind = match i["kind"].as_str() {
            Some("kummer") => Kind::Kummer,
            Some("dickson") => Kind::Dickson,
            _ => return Err("invalid tower kind".into()),
        };
        let a = get("a")?;
        let b = get("b")?;
        let scale = get("scale_g")?;
        let terminal = get("terminal_c")?;
        let r = i["R"]
            .as_array()
            .filter(|r| r.len() == 2)
            .ok_or("invalid target")?;
        let target = (
            r[0].as_u64().ok_or("invalid target x")?,
            r[1].as_u64().ok_or("invalid target y")?,
        );
        if [a, b, scale, terminal, target.0, target.1]
            .iter()
            .any(|&x| x >= p)
            || scale == 0
        {
            return Err("noncanonical field value".into());
        }
        let cv = Curve { f, a, b };
        if target.1 == 0
            || !cv.on(Some(target))
            || f.add(f.mul(4, f.pow(a, 3)), f.mul(27, f.pow(b, 2))) == 0
        {
            return Err("singular curve or unsupported finite target".into());
        }
        let mut domain = vec![];
        match kind {
            Kind::Kummer => {
                if (p - 1) % (1 << t) != 0 || terminal != 1 {
                    return Err("invalid Kummer tower".into());
                }
                let h = (2..p).find(|&h| !f.is_sq(h)).ok_or("no nonsquare")?;
                let zeta = f.pow(h, (p - 1) >> t);
                let mut w = 1;
                for _ in 0..1 << t {
                    domain.push(w);
                    w = f.mul(w, zeta);
                }
            }
            Kind::Dickson => {
                if p % 4 != 3
                    || (p + 1) % (1 << t) != 0
                    || scale != 1
                    || terminal == 2
                    || terminal == p - 2
                {
                    return Err("invalid Dickson torus".into());
                }
                let tower = &i["info"]["tower"];
                let pair = |key: &str| -> Result<Fp2, String> {
                    let v = tower[key]
                        .as_array()
                        .filter(|v| v.len() == 2)
                        .ok_or("missing torus generator")?;
                    let x = v[0]
                        .as_u64()
                        .filter(|&x| x < p)
                        .ok_or("invalid generator")?;
                    let y = v[1]
                        .as_u64()
                        .filter(|&x| x < p)
                        .ok_or("invalid generator")?;
                    Ok((x, y))
                };
                let u = pair("u")?;
                let zeta = pair("zeta")?;
                if f2pow(f, u, p + 1) != (1, 0)
                    || f2pow(f, zeta, 1 << t) != (1, 0)
                    || f2pow(f, zeta, 1 << (t - 1)) != (p - 1, 0)
                {
                    return Err("invalid torus generator order".into());
                }
                let mut w = u;
                for _ in 0..1 << t {
                    let inv = f2inv(f, w);
                    if f.add(w.1, inv.1) != 0 {
                        return Err("nonrational torus trace".into());
                    }
                    domain.push(f.add(w.0, inv.0));
                    w = f2mul(f, w, zeta);
                }
            }
        }
        domain.sort_unstable();
        domain.dedup();
        if domain.len() != 1 << t
            || domain
                .iter()
                .any(|&w| tower_value(kind, f, w, t) != terminal)
        {
            return Err("tower root set is not complete/distinct".into());
        }
        Ok(Self {
            cv,
            kind,
            t,
            scale,
            terminal,
            target,
            domain,
        })
    }

    fn roots(&self, coefficients: &[u64]) -> Result<Vec<u64>, String> {
        if coefficients.len() != 4 || coefficients[3] != 1 {
            return Err("support must be monic cubic".into());
        }
        let f = self.cv.f;
        let mut poly = coefficients.to_vec();
        let mut roots = vec![];
        for &w in &self.domain {
            if poly.len() > 1 && ueval(f, &poly, w) == 0 {
                let (quot, rem) = udivrem(f, &poly, &vec![f.neg(w), 1]);
                if !rem.is_empty() {
                    return Err("nonzero division remainder".into());
                }
                poly = quot;
                roots.push(f.mul(self.scale, w));
                if poly.len() > 1 && ueval(f, &poly, w) == 0 {
                    return Err("outside chart: repeated support root".into());
                }
            }
        }
        roots.sort_unstable();
        if poly != vec![1] || roots.len() != 3 {
            return Err("support does not split over declared tower domain".into());
        }
        if roots.contains(&self.target.0) || roots.iter().any(|&x| self.cv.rhs(x) == 0) {
            return Err("outside chart: target abscissa or zero-y support".into());
        }
        Ok(roots)
    }
}

/// Recover the monic RR chart from a verified signed decomposition. The norm
/// polynomial is checked as an identity, not inferred from interpolation alone.
fn chart(q: &Query, points: &[(u64, u64)]) -> Option<[u64; 3]> {
    let f = q.cv.f;
    let rows: Vec<Vec<u64>> = points
        .iter()
        .copied()
        .chain(std::iter::once((q.target.0, f.neg(q.target.1))))
        .map(|(x, y)| vec![1, x, y, f.neg(f.mul(x, x))])
        .collect();
    let sol = solve_linear(f, rows, 3)?;
    if sol[2] == 0 {
        return None;
    }
    let ap = vec![sol[0], sol[1], 1];
    let mut norm = umul(f, &ap, &ap);
    let fp = [q.cv.b, q.cv.a, 0, 1];
    for (k, &v) in fp.iter().enumerate() {
        norm[k] = f.sub(norm[k], f.mul(v, f.mul(sol[2], sol[2])));
    }
    let mut rhs = vec![f.neg(q.target.0), 1];
    for &(x, _) in points {
        rhs = umul(f, &rhs, &vec![f.neg(x), 1]);
    }
    (utrim(norm) == utrim(rhs)).then_some([sol[0], sol[1], sol[2]])
}

fn normalise(
    q: &Query,
    arm: &str,
    names: &[String],
    values: &[u64],
) -> Result<Vec<Vec<(u64, u64)>>, String> {
    if names.len() != values.len() || values.iter().any(|&x| x >= q.cv.f.p) {
        return Err("invalid assignment dimensions/residues".into());
    }
    let get = |s: &str| {
        names
            .iter()
            .position(|n| n == s)
            .map(|k| values[k])
            .ok_or_else(|| format!("missing {s}"))
    };
    let f = q.cv.f;
    let u = match arm {
        "rr" => vec![get("u0")?, get("u1")?, get("u2")?, 1],
        "f" => vec![f.neg(get("e3")?), get("e2")?, f.neg(get("e1")?), 1],
        _ => return Err("extract supports RR and F only".into()),
    };
    let xs = q.roots(&u)?;
    let mut result = BTreeSet::new();
    if arm == "rr" {
        let a0 = get("a0")?;
        let a1 = get("a1")?;
        let c = get("c0")?;
        if c == 0 {
            return Err("outside chart: C=0".into());
        }
        let points: Vec<_> = xs
            .iter()
            .map(|&x| {
                (
                    x,
                    f.mul(f.neg(f.add(f.add(f.mul(x, x), f.mul(a1, x)), a0)), f.inv(c)),
                )
            })
            .collect();
        if points.iter().any(|&p| !q.cv.on(Some(p)))
            || points.iter().fold(None, |acc, &p| q.cv.add(acc, Some(p))) != Some(q.target)
            || chart(q, &points) != Some([a0, a1, c])
        {
            return Err("RR extraction fails curve, sum or norm check".into());
        }
        result.insert(points);
    } else {
        let ys: Vec<u64> = xs
            .iter()
            .map(|&x| {
                q.cv.f
                    .sqrt(q.cv.rhs(x))
                    .ok_or("support includes twist point".to_string())
            })
            .collect::<Result<_, _>>()?;
        for mask in 0..8 {
            let points: Vec<_> = xs
                .iter()
                .enumerate()
                .map(|(k, &x)| {
                    (
                        x,
                        if mask >> k & 1 == 1 {
                            f.neg(ys[k])
                        } else {
                            ys[k]
                        },
                    )
                })
                .collect();
            if points.iter().fold(None, |acc, &p| q.cv.add(acc, Some(p))) == Some(q.target)
                && chart(q, &points).is_some()
            {
                result.insert(points);
            }
        }
    }
    Ok(result.into_iter().collect())
}

pub(super) fn validate_query(metadata: &Value) -> Result<(), String> {
    Query::from_summary(&metadata["summary"]).map(|_| ())
}

pub(super) fn checked_extract(
    metadata: &Value,
    text: &str,
    values: &[u64],
) -> Result<Value, String> {
    let summary = &metadata["summary"];
    let q = Query::from_summary(summary)?;
    if summary["sha256"] != sha256_hex(text.as_bytes()) {
        return Err("system file digest mismatch".into());
    }
    let mut lines = text.lines();
    let names: Vec<String> = lines
        .next()
        .unwrap_or("")
        .split(',')
        .map(String::from)
        .collect();
    if lines.next().and_then(|s| s.parse::<u64>().ok()) != Some(q.cv.f.p) {
        return Err("input modulus mismatch".into());
    }
    let body = lines.collect::<Vec<_>>().join("\n");
    let eqs = parse_gb(&format!("[{body}]"), &names, q.cv.f);
    if values.len() != names.len()
        || values.iter().any(|&x| x >= q.cv.f.p)
        || eqs.is_empty()
        || eqs.iter().any(|p| p.eval(q.cv.f, &values) != 0)
    {
        return Err("assignment fails supplied emitted system".into());
    }
    let (names, values) = if metadata.get("reduction").is_some() {
        let cert = &metadata["reduction"];
        if cert["reduced_ms_sha256"] != summary["sha256"] {
            return Err("reduction digest mismatch".into());
        }
        let values = reduction::lift(cert, values)?;
        let names: Vec<String> =
            serde_json::from_value(cert["original_names"].clone()).map_err(|e| e.to_string())?;
        (names, values)
    } else {
        (names, values.to_vec())
    };
    let arm = summary["system"].as_str().ok_or("missing arm")?;
    let points = normalise(&q, arm, &names, &values)?;
    Ok(
        json!({"schema":"p1tool.extract/v1","system":arm,"input_sha256":summary["sha256"],
        "chart":"m3 distinct abscissae; nonzero y,C; support excludes x_R; monic A degree 2",
        "target":q.target,"decompositions":points,"scope":"one supplied rational assignment; completeness of solver enumeration not asserted",
        "standing":"producer-unverified"}),
    )
}

pub(super) fn cmd_extract(args: &HashMap<String, String>) {
    let read = |key: &str| {
        std::fs::read_to_string(
            args.get(key)
                .unwrap_or_else(|| die(&format!("missing --{key}"))),
        )
        .unwrap_or_else(|e| die(&e.to_string()))
    };
    let metadata: Value =
        serde_json::from_str(&read("metadata")).unwrap_or_else(|e| die(&e.to_string()));
    let values: Vec<u64> =
        serde_json::from_str(&read("values")).unwrap_or_else(|e| die(&e.to_string()));
    println!(
        "{}",
        checked_extract(&metadata, &read("system-file"), &values).unwrap_or_else(|e| die(&e))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Inst, Query) {
        let i = sample_instance(1151, 294, 772, Kind::Dickson, 3, 4, None, 1);
        let summary = json!({"schema":"p1tool.emit/v1","instance":inst_json(&i)});
        let q = Query::from_summary(&summary).unwrap();
        (i, q)
    }

    #[test]
    fn input_digest_equations_and_reduced_map_are_checked_before_extraction() {
        let (i, _) = fixture();
        for arm in ["rr", "f"] {
            let original = if arm == "rr" {
                build_rr(&i)
            } else {
                build_f(&i)
            };
            let text = ms_text(&original, i.p);
            let metadata = json!({"summary":{"schema":"p1tool.emit/v1","system":arm,
                "sha256":sha256_hex(text.as_bytes()),"instance":inst_json(&i)}});
            let expected = checked_extract(&metadata, &text, &original.plant).unwrap();
            assert!(!expected["decompositions"].as_array().unwrap().is_empty());
            assert!(checked_extract(&metadata, &format!("{text}\n"), &original.plant).is_err());
            let mut bad = original.plant.clone();
            bad[0] = (bad[0] + 1) % i.p;
            assert!(checked_extract(&metadata, &text, &bad).is_err());
            let mut bad_metadata = metadata.clone();
            bad_metadata["summary"]["instance"]["p"] = json!(1279);
            assert!(checked_extract(&bad_metadata, &text, &original.plant).is_err());
            if arm == "rr" {
                let (reduced, map) = reduction::reduce(build_rr(&i), F::new(i.p), "quadratic");
                let reduced_text = ms_text(&reduced, i.p);
                let mut reduced_metadata = metadata.clone();
                reduced_metadata["summary"]["sha256"] = json!(sha256_hex(reduced_text.as_bytes()));
                reduced_metadata["reduction"] = map;
                let output =
                    checked_extract(&reduced_metadata, &reduced_text, &reduced.plant).unwrap();
                assert_eq!(expected["decompositions"], output["decompositions"]);
                reduced_metadata["reduction"]["reduced_ms_sha256"] = json!("wrong");
                assert!(checked_extract(&reduced_metadata, &reduced_text, &reduced.plant).is_err());
            }
        }
    }
    #[test]
    fn signed_rr_f_and_reduced_rr_have_identical_outputs() {
        let (i, q) = fixture();
        let rr = build_rr(&i);
        let ff = build_f(&i);
        let expected = normalise(&q, "rr", &rr.names, &rr.plant).unwrap();
        assert_eq!(normalise(&q, "f", &ff.names, &ff.plant).unwrap(), expected);
        for mode in ["affine", "quadratic"] {
            let (reduced, map) = reduction::reduce(build_rr(&i), q.cv.f, mode);
            let lifted = reduction::lift(&map, &reduced.plant).unwrap();
            assert_eq!(normalise(&q, "rr", &rr.names, &lifted).unwrap(), expected);
        }
        let mut opposite = q;
        opposite.target.1 = opposite.cv.f.neg(opposite.target.1);
        let negated: Vec<_> = expected[0]
            .iter()
            .map(|&(x, y)| (x, opposite.cv.f.neg(y)))
            .collect();
        assert_eq!(
            normalise(&opposite, "f", &ff.names, &ff.plant).unwrap(),
            vec![negated]
        );
        assert!(normalise(&opposite, "rr", &rr.names, &rr.plant).is_err());
    }

    // Separately written affine addition for the bounded reference. Field
    // primitives are shared; this is not an independent signed PCC audit.
    fn ref_add(cv: &Curve, p: Pt, r: Pt) -> Pt {
        let (Some((x, y)), Some((u, v))) = (p, r) else {
            return p.or(r);
        };
        let f = cv.f;
        if x == u && f.add(y, v) == 0 {
            return None;
        }
        let (num, den) = if x == u {
            (f.add(f.mul(3, f.mul(x, x)), cv.a), f.add(y, y))
        } else {
            (f.sub(v, y), f.sub(u, x))
        };
        let slope = f.mul(num, f.pow(den, f.p - 2));
        let intercept = f.sub(y, f.mul(slope, x));
        let out_x = f.sub(f.sub(f.mul(slope, slope), x), u);
        Some((out_x, f.neg(f.add(f.mul(slope, out_x), intercept))))
    }

    #[test]
    fn exhaustive_bounded_signed_reference_including_zero_and_multiple_targets() {
        let (template, q) = fixture();
        let f = q.cv.f;
        // Enumerate y by its defining equation rather than the extractor's sqrt.
        let points: Vec<_> = q
            .domain
            .iter()
            .filter_map(|&w| {
                let x = f.mul(q.scale, w);
                (1..f.p)
                    .find(|&y| f.mul(y, y) == q.cv.rhs(x))
                    .map(|y| (w, x, y))
            })
            .collect();
        let mut references: BTreeMap<(u64, u64), BTreeSet<Vec<(u64, u64)>>> = BTreeMap::new();
        for a in 0..points.len() {
            for b in a + 1..points.len() {
                for c in b + 1..points.len() {
                    for mask in 0..8 {
                        let selected = [points[a], points[b], points[c]];
                        let mut ps: Vec<_> = selected
                            .iter()
                            .enumerate()
                            .map(|(k, &(_, x, y))| {
                                (x, if mask >> k & 1 == 1 { f.neg(y) } else { y })
                            })
                            .collect();
                        ps.sort_unstable();
                        if let Some(r) =
                            ref_add(&q.cv, ref_add(&q.cv, Some(ps[0]), Some(ps[1])), Some(ps[2]))
                        {
                            if r.1 != 0 && !ps.iter().any(|p| p.0 == r.0) {
                                references.entry(r).or_default().insert(ps);
                            }
                        }
                    }
                }
            }
        }
        assert!(
            references.values().any(|v| v.len() > 1),
            "fixture needs multiple-output target"
        );
        let mut zero_target = None;
        for x in 0..f.p {
            if let Some(y) = f.sqrt(q.cv.rhs(x)) {
                if y != 0 && !references.contains_key(&(x, y)) && !points.iter().any(|p| p.1 == x) {
                    zero_target = Some((x, y));
                    break;
                }
            }
        }
        let zero_query = Query {
            cv: Curve {
                f,
                a: template.a,
                b: template.b,
            },
            kind: template.kind,
            t: template.t,
            scale: template.scale,
            terminal: template.c,
            target: zero_target.expect("zero-output target"),
            domain: q.domain.clone(),
        };
        let names: Vec<String> = ["e1", "e2", "e3"].iter().map(|s| s.to_string()).collect();
        for a in 0..points.len() {
            for b in a + 1..points.len() {
                for c in b + 1..points.len() {
                    let mut u = vec![1];
                    for k in [a, b, c] {
                        u = umul(f, &u, &vec![f.neg(points[k].0), 1]);
                    }
                    let values = [f.neg(u[2]), u[1], f.neg(u[0])];
                    assert!(normalise(&zero_query, "f", &names, &values)
                        .unwrap()
                        .is_empty());
                }
            }
        }
        for (&target, reference) in &references {
            let q = Query {
                cv: Curve {
                    f,
                    a: template.a,
                    b: template.b,
                },
                kind: template.kind,
                t: template.t,
                scale: template.scale,
                terminal: template.c,
                target,
                domain: q.domain.clone(),
            };
            let mut rr_outputs = BTreeSet::new();
            let mut f_outputs = BTreeSet::new();
            for ps in reference {
                assert!(
                    chart(&q, ps).is_some(),
                    "reference relation is outside declared monic chart"
                );
                let mut i = template.clone();
                i.xr = target.0;
                i.yr = target.1;
                i.x = ps.iter().map(|p| p.0).collect();
                i.y = ps.iter().map(|p| p.1).collect();
                i.w = i.x.iter().map(|&x| f.mul(x, f.inv(i.scale))).collect();
                let rr = build_rr(&i);
                let ff = build_f(&i);
                assert!(rr.eqs.iter().all(|p| p.eval(f, &rr.plant) == 0));
                assert!(ff.eqs.iter().all(|p| p.eval(f, &ff.plant) == 0));
                rr_outputs.extend(normalise(&q, "rr", &rr.names, &rr.plant).unwrap());
                f_outputs.extend(normalise(&q, "f", &ff.names, &ff.plant).unwrap());
            }
            assert_eq!(&rr_outputs, reference);
            assert_eq!(&f_outputs, reference);
        }
    }

    #[test]
    fn kummer_scaled_control_has_the_same_signed_output() {
        let i = sample_instance(786433, 658649, 132295, Kind::Kummer, 3, 4, None, 1);
        let summary = json!({"schema":"p1tool.emit/v1","instance":inst_json(&i)});
        let q = Query::from_summary(&summary).unwrap();
        assert_ne!(q.scale, 1);
        let rr = build_rr(&i);
        let ff = build_f(&i);
        assert_eq!(
            normalise(&q, "rr", &rr.names, &rr.plant).unwrap(),
            normalise(&q, "f", &ff.names, &ff.plant).unwrap()
        );
    }

    #[test]
    fn rejects_repeated_roots_tampered_domain_and_wrong_rr_signs() {
        let (i, q) = fixture();
        let f = q.cv.f;
        let w = q.domain[0];
        let cubic = umul(
            f,
            &umul(f, &vec![f.neg(w), 1], &vec![f.neg(w), 1]),
            &vec![f.neg(q.domain[1]), 1],
        );
        assert!(q.roots(&cubic).is_err());
        let mut s = json!({"schema":"p1tool.emit/v1","instance":inst_json(&i)});
        s["instance"]["terminal_c"] = json!(2);
        assert!(Query::from_summary(&s).is_err());
        let mut rr = build_rr(&i);
        rr.plant[names_index(&rr.names, "c0")] = 0;
        assert!(normalise(&q, "rr", &rr.names, &rr.plant).is_err());
    }
}
