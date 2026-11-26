//! Read-only Rust replay of every case, function, composite edge and graph.
pub use crypto_lib::cryptanalysis;
#[path = "../../../../src/cryptanalysis/jv_forced_divisor.rs"]
pub mod jv_forced_divisor;
#[path = "../../../../src/cryptanalysis/jv_forced_graph.rs"]
pub mod jv_forced_graph;
use cryptanalysis::{
    f4_fp,
    jv_cover::{BaseEl, Div, Fld, Fq, E2},
    jv_sieve::FpRing,
};
use jv_forced_divisor::Family;
use jv_forced_graph as graph;
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn ensure(b: bool, s: &str) -> Result<()> {
    if b {
        Ok(())
    } else {
        Err(s.into())
    }
}
fn n(v: &Value) -> Result<u64> {
    v.as_u64().ok_or_else(|| "integer counter".into())
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn native(v: &Value) -> Result<Vec<E2>> {
    Ok(serde_json::from_value::<Vec<[u64; 2]>>(v.clone())?
        .into_iter()
        .map(E2)
        .collect())
}
fn encode(v: &[E2]) -> Value {
    json!(v.iter().map(|e| e.0).collect::<Vec<_>>())
}
fn family(f: &Fq, h: &[E2], class: &Value, sign: &Value) -> Result<Family> {
    let k: [u64; 6] = serde_json::from_value(class.clone())?;
    let s = sign.as_i64().ok_or("signed class")?;
    ensure([-1, 1].contains(&s), "unit orientation")?;
    let mut v = vec![E2([k[2], k[3]]), E2([k[4], k[5]])];
    if s == -1 {
        for x in &mut v {
            *x = f.neg(x);
        }
    }
    Ok(Family::new(
        f,
        h,
        &[f.from_fp(k[0]), f.from_fp(k[1]), E2::ONE],
        &v,
    )?)
}
fn base(f: &Fq, h: &[E2]) -> Vec<BaseEl> {
    let mut out = vec![];
    let mut rng = StdRng::seed_from_u64(13);
    for x in 0..f.p {
        let hx = h
            .iter()
            .rev()
            .fold(E2::ZERO, |v, c| f.add(&f.scale(&v, x), c));
        let Some(y) = f.sqrt(&hx, &mut rng) else {
            continue;
        };
        if y == E2::ZERO {
            continue;
        }
        let neg = f.neg(&y);
        let y = if y.0 <= neg.0 { y } else { neg };
        out.push(BaseEl {
            x,
            y,
            d: Div {
                u: vec![f.neg(&f.from_fp(x)), E2::ONE],
                v: vec![y],
            },
        });
    }
    out
}
fn principal(
    f: &Fq,
    h: &[E2],
    domain: &graph::Domain<'_>,
    wire: &Value,
) -> Result<graph::PrincipalProof> {
    let family = family(
        f,
        h,
        &wire["forced"]["class"],
        &wire["forced"]["coefficient"],
    )?;
    let point = family.recover_parameters(f, &native(&wire["a"])?, &native(&wire["c"])?)?;
    let function = family.instantiate(f, &point)?;
    ensure(
        encode(&function.norm) == wire["norm"],
        "original proof norm",
    )?;
    let roots: Vec<u64> = serde_json::from_value(wire["roots"].clone())?;
    let factors: Vec<Vec<u64>> = serde_json::from_value(wire["factors"].clone())?;
    let proof = graph::PrincipalProof::check(domain, &family, &function, &roots, &factors)?;
    ensure(
        serde_json::to_value(&proof)? == *wire,
        "complete original principal proof",
    )?;
    Ok(proof)
}
fn composite(
    f: &Fq,
    h: &[E2],
    domain: &graph::Domain<'_>,
    wire: &Value,
) -> Result<graph::CompositeEdge> {
    let edge = graph::CompositeEdge::cancel(
        domain,
        principal(f, h, domain, &wire["candidate"])?,
        principal(f, h, domain, &wire["anchor"])?,
    )?;
    ensure(
        serde_json::to_value(&edge)? == *wire,
        "complete composite incidence/provenance",
    )?;
    Ok(edge)
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    ensure(a.len() == 3, "RAW_ROOT NEW_ANALYSIS")?;
    let root = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    ensure(!out.exists(), "exclusive output")?;
    for name in ["native.status", "posthash.status", "launcher.status"] {
        ensure(
            fs::read_to_string(root.join(name))?.trim() == "0",
            "native/hash/launcher status",
        )?;
    }
    ensure(
        fs::metadata(root.join("worker.stderr"))?.len() == 0,
        "empty stderr",
    )?;
    let fixture_bytes = fs::read(root.join("fixture.json"))?;
    ensure(
        sha(&fixture_bytes) == "d09c03c2734e32b36c229f1fc52997c05e9dfc1b12ba7f8c57c330364d6c26e9",
        "public fixture hash",
    )?;
    let fixture: Value = serde_json::from_slice(&fixture_bytes)?;
    let raw: Value = serde_json::from_slice(&fs::read(root.join("output/summary.json"))?)?;
    let conditions: Value = serde_json::from_slice(&fs::read(root.join("conditions.jsonl"))?)?;
    ensure(
        raw["planned_calls"] == 258
            && raw["attempted_calls"] == 258
            && raw["not_run_calls"] == 0
            && raw["stopped"].is_null()
            && raw["complete_reference_comparison"] == true,
        "complete panel",
    )?;
    let samples = raw["samples"].as_array().ok_or("samples")?;
    ensure(samples.len() == 258, "denominator")?;
    let events: Vec<Value> = fs::read_to_string(root.join("output/events.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    ensure(events.len() == 516, "complete event stream")?;
    let parameters: Vec<_> = std::iter::once(369u64).chain(0..128).collect();
    ensure(raw["parameters"] == json!(parameters), "fixed values")?;
    let f = Fq::new(1009);
    let h = native(&fixture["h"])?;
    let base = base(&f, &h);
    ensure(base.len() == 509, "base")?;
    let domain = graph::Domain::new(&f, &h, &base)?;
    let forced = &fixture["remainders"][0];
    let family = family(&f, &h, &forced["key"], &forced["coefficient"])?;
    let positive =
        family.recover_parameters(&f, &native(&fixture["a"])?, &native(&fixture["c"])?)?;
    let positive_function = family.instantiate(&f, &positive)?;
    let positive_roots: Vec<_> = fixture["small"]
        .as_array()
        .ok_or("small")?
        .iter()
        .map(|s| n(&s["x"]))
        .collect::<Result<_>>()?;
    let anchor =
        graph::PrincipalProof::check(&domain, &family, &positive_function, &positive_roots, &[])?;
    let ring = FpRing::new(1009);
    let mut rows = vec![];
    let mut cases = vec![];
    let mut retained_edge = None;
    for (arm, mode) in ["full-reference", "traced"].iter().enumerate() {
        let mut graphs = [
            graph::Graph::new(graph::Options::default())?,
            graph::Graph::new(graph::Options::default())?,
        ];
        let mut edges: [Vec<graph::CompositeEdge>; 2] = [vec![], vec![]];
        let (
            mut solver,
            mut development_solver,
            mut roots,
            mut dev_roots,
            mut replays,
            mut quartics,
            mut split_refusals,
            mut extra,
            mut common,
        ) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
        let mut policy = [0u64; 2];
        let mut dmin = u64::MAX;
        let mut dmax = 0;
        for (index, &parameter) in parameters.iter().enumerate() {
            let pos = arm * 129 + index;
            let s = &samples[pos];
            let r = &samples[index];
            ensure(
                s["mode"] == *mode && s["case"] == index && s["parameter"] == parameter,
                "fixed order",
            )?;
            ensure(
                events[2 * pos]["event"] == "start"
                    && events[2 * pos]["mode"] == *mode
                    && events[2 * pos]["case"] == index
                    && events[2 * pos + 1] == *s,
                "event/result equality",
            )?;
            ensure(
                s["resolved"] == true && s["invalid_original_replay"] == false,
                "resolved original replay",
            )?;
            if arm == 1 {
                ensure(
                    s["matches_full_reference"] == true
                        && s["roots"] == r["roots"]
                        && s["functions"] == r["functions"]
                        && s["old_graph"] == r["old_graph"]
                        && s["expanded_graph"] == r["expanded_graph"],
                    "complete output equality",
                )?;
            }
            for name in ["timed_out", "incomplete", "inconsistent", "nonseparating"] {
                ensure(
                    s["root_stats"][name] == r["root_stats"][name],
                    "resolution flags",
                )?;
            }
            let st = &s["root_stats"];
            let cost = n(&st["f4_muls"])? + n(&st["lin_muls"])?;
            solver += cost;
            if index > 0 {
                development_solver += cost;
            }
            replays += u64::from(st["replayed"] == true);
            let d = n(&st["delta"])?;
            dmin = dmin.min(d);
            dmax = dmax.max(d);
            let common_case = n(&s["common_original_legacy_fq_products"])?;
            common += common_case;
            let pc = [
                n(&s["policy_legacy_fq_products"][0])?,
                n(&s["policy_legacy_fq_products"][1])?,
            ];
            ensure(
                common_case + pc[0] + pc[1] == n(&s["post_legacy_fq_products"])?,
                "disjoint post components",
            )?;
            for i in 0..2 {
                policy[i] += pc[i];
            }
            let original_roots: Vec<Vec<u64>> = serde_json::from_value(s["roots"].clone())?;
            let functions = s["functions"].as_array().ok_or("functions")?;
            ensure(
                original_roots.len() == functions.len(),
                "root/function denominator",
            )?;
            roots += original_roots.len() as u64;
            if index > 0 {
                dev_roots += original_roots.len() as u64;
            }
            let input = family.specialize(0, parameter)?;
            for (z, row) in original_roots.iter().zip(functions) {
                ensure(
                    z.len() == 7
                        && z.iter().all(|&x| x < 1009)
                        && input.iter().all(|q| f4_fp::eval(q, z, 1009) == 0),
                    "original specialized root",
                )?;
                let mut point = z.clone();
                point.insert(0, parameter);
                ensure(row["point"] == json!(point), "root lifting")?;
                let function = family.instantiate(&f, &point)?;
                ensure(
                    encode(&function.a) == row["normalized_a"]
                        && encode(&function.c) == row["normalized_c"]
                        && encode(&function.norm) == row["norm"],
                    "original function/norm replay",
                )?;
                let polynomial: Vec<_> = function.quotient.iter().map(|e| e.0[0]).collect();
                let small = ring.roots_by_table(&polynomial);
                ensure(row["small_roots"] == json!(small), "complete small roots")?;
                if small.len() == 3 {
                    quartics += 1;
                    let report = &row["split_report"];
                    ensure(
                        report["p"] == 1009 && report["max_attempts"] == 8,
                        "bounded split report",
                    )?;
                    ensure(
                        report["attempts"].as_array().ok_or("split attempts")?.len() <= 8,
                        "all bounded attempts",
                    )?;
                    if report["factors"].is_null() {
                        split_refusals += 1;
                        ensure(
                            row["expanded"]["admitted"] == false && !report["refusal"].is_null(),
                            "retained split refusal",
                        )?;
                    }
                }
                for (i, name) in ["old", "expanded"].iter().enumerate() {
                    let outcome = &row[*name];
                    if outcome["trivial"] == true {
                        let proof =
                            graph::PrincipalProof::check(&domain, &family, &function, &small, &[])?;
                        ensure(
                            graph::CompositeEdge::cancel(&domain, proof, anchor.clone())?
                                .is_trivial(),
                            "actual trivial cancellation",
                        )?;
                    }
                    if outcome["admitted"] == true {
                        let edge = composite(&f, &h, &domain, &outcome["proof"])?;
                        let id = edges[i].len();
                        ensure(
                            outcome["source"] == id && !edge.is_trivial(),
                            "composite source stream",
                        )?;
                        let produced = edge.insert(&domain, &mut graphs[i], id)?;
                        ensure(json!(produced) == outcome["events"], "actual graph events")?;
                        edges[i].push(edge);
                        ensure(
                            outcome["projections"] == json!([]),
                            "observed no full projections",
                        )?;
                        if i == 1 && small.len() == 3 && index > 0 {
                            extra += 1;
                            retained_edge = Some(outcome["proof"].clone());
                            ensure(
                                parameter == 10 && index == 11,
                                "observed extra edge location",
                            )?;
                        }
                    }
                }
            }
            ensure(
                json!(graphs[0].stats()) == s["old_graph"]
                    && json!(graphs[1].stats()) == s["expanded_graph"],
                "reconstructed cumulative graphs",
            )?;
            cases.push(json!({"mode":mode,"case":index,"parameter":parameter,"solver_products":cost,"roots":original_roots.len(),"delta":d}));
        }
        ensure(
            extra == 1 && edges[0].is_empty() && edges[1].len() == 1,
            "observed edge scope",
        )?;
        for i in 0..2 {
            ensure(
                serde_json::to_value(&edges[i])?
                    == raw["graphs"][arm][if i == 0 { "old" } else { "expanded" }]["edges"],
                "final proof stream",
            )?;
        }
        rows.push(json!({"mode":mode,"calls":129,"solver_products":solver,"development_solver_products":development_solver,"root_occurrences":roots,"development_norm_functions":dev_roots,"three_small_requests":quartics,"split_refusals":split_refusals,"extra_development_edges":extra,"full_projected_rows":0,"trace_replays":replays,"common_legacy_fq_products":common,"old_policy_legacy_fq_products":policy[0],"expanded_policy_legacy_fq_products":policy[1],"delta_min":dmin,"delta_max":dmax,"old_graph":graphs[0].stats(),"expanded_graph":graphs[1].stats()}));
    }
    let result = json!({"schema":"graph-ic.anchored-policy-analysis/v1","source_revision":"9c7d9982e1914e08588cc9c8cf0a86d9b3e96990","rows":rows,"cases":cases,"setups":raw["setups"],"conditions":conditions,"cost_per_full_relation":null,"independent_rank":null,"full_native_cost_available":false,"wall_comparison":false,"rho_comparison":null,"signed_verification":false});
    fs::create_dir(out)?;
    fs::write(
        out.join("derived.json"),
        serde_json::to_vec_pretty(&result)?,
    )?;
    fs::write(
        out.join("extra-edge.json"),
        serde_json::to_vec_pretty(&retained_edge.ok_or("edge proof")?)?,
    )?;
    let maximum = n(&rows[0]["solver_products"])? as f64;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"950\" height=\"240\"><rect width=\"950\" height=\"240\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172b4d\"><text x=\"24\" y=\"30\" font-size=\"20\">Anchored support expansion: one extra edge, zero full relations</text><text x=\"24\" y=\"54\" font-size=\"14\">129 fibers per arm; each policy pays the whole shared finder cost; solver products only</text>");
    for (i, row) in rows.iter().enumerate() {
        let value = n(&row["solver_products"])?;
        let width = value as f64 / maximum * 520.0;
        let y = 80 + i * 52;
        svg.push_str(&format!("<text x=\"24\" y=\"{}\" font-size=\"15\">{}</text><rect x=\"180\" y=\"{y}\" width=\"{width}\" height=\"28\" fill=\"{}\"/><text x=\"{}\" y=\"{}\" font-size=\"14\">{value}</text>",y+20,row["mode"].as_str().unwrap(),["#8799aa","#007f73"][i],width+192.0,y+20));
    }
    svg.push_str("<text x=\"24\" y=\"215\" font-size=\"13\">Old policy: 0 edges. Expanded: 1 deferred edge. No rank/full-native/wall/parity claim; producer-unsigned.</text></g></svg>");
    fs::write(out.join("solver-products.svg"), svg)?;
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}
