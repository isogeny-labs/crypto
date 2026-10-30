//! `ic tensor`: the toy tensor-contraction index calculus of
//! `crates/ecdl-tensor-ic` (Rust port of the Go `ecdltensor` prototype).
//!
//! `scale` prints the shape of the low-degree gadget networks (edges per bit,
//! max degree / b^(3/4)) as the field grows; networks are built, never
//! evaluated. `demo` solves one discrete logarithm on a random toy curve with
//! relations found by exact (exponential-time) contraction. Neither is a
//! performance result: without an [Ope26] solver every counter here costs
//! more than Pollard rho.

use clap::{Args, Subcommand};
use ecdl_tensor_ic::field::rand_below;
use ecdl_tensor_ic::scale::{ScaleRow, DEFAULT_BITS};
use ecdl_tensor_ic::{
    random_prime_order_curve, DFSCounter, NetworkStats, Solver, SparseContractor,
};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};

const SCOPE: &str = "toy prototype: exponential-time counters, no [Ope26] solver, \
gadgets and affine factor-base hash are our own construction; no speedup over Pollard rho";

#[derive(Args, Clone, Debug)]
pub struct TensorArgs {
    #[command(subcommand)]
    pub command: TensorCommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum TensorCommand {
    /// Shape of the mul, complete-add and point gadget networks for
    /// p = 2^(b-1)+1 (structure only; the modulus need not be prime).
    Scale(ScaleArgs),
    /// Solve one discrete logarithm on a random prime-order toy curve with
    /// relations found by tensor contraction.
    Demo(DemoArgs),
}

#[derive(Args, Clone, Debug)]
pub struct ScaleArgs {
    /// Comma-separated field sizes in bits (each at least 4).
    #[arg(long, value_delimiter = ',', default_values_t = DEFAULT_BITS)]
    pub bits: Vec<usize>,
}

#[derive(Args, Clone, Debug)]
pub struct DemoArgs {
    /// Field size in bits (4 to 26; naive point counting).
    #[arg(long, default_value_t = 12)]
    pub bits: usize,
    /// Number of factor bases m (at least 2).
    #[arg(long, default_value_t = 2)]
    pub m: usize,
    /// Seed for the curve, the secret and the solver.
    #[arg(long, default_value_t = 1)]
    pub seed: u64,
    /// Find relations on the low-degree gadget network with the DFS counter
    /// (slow: every gadget is evaluated for every candidate tuple).
    #[arg(long)]
    pub gadgets: bool,
}

pub fn run(args: TensorArgs, json_mode: bool) -> Result<Value, String> {
    match args.command {
        TensorCommand::Scale(a) => scale(a),
        TensorCommand::Demo(a) => demo(a, json_mode),
    }
}

fn stats(st: &NetworkStats) -> Value {
    json!({"edges": st.edges, "vertices": st.vertices, "max_degree": st.max_degree, "is_graph": st.is_graph})
}

fn scale(args: ScaleArgs) -> Result<Value, String> {
    let mut rows = Vec::new();
    for b in args.bits {
        let row = ScaleRow::new(b).ok_or_else(|| format!("field size {b} is below 4 bits"))?;
        rows.push(json!({
            "b": row.b, "t": row.t, "n": row.n, "L": row.l,
            "mul": stats(&row.mul), "add": stats(&row.add), "point": stats(&row.point),
            "max_degree": row.max_degree(), "is_graph": row.is_graph(),
            "degree_over_b_three_quarters": row.degree_ratio(),
            "line": row.line(),
        }));
    }
    let all_graphs = rows.iter().all(|r| r["is_graph"] == true);
    Ok(json!({
        "schema_version": 1, "operation": "tensor-scale",
        "status": if all_graphs { "complete" } else { "error" },
        "header": ScaleRow::header(), "rows": rows, "scope": SCOPE,
    }))
}

fn demo(args: DemoArgs, json_mode: bool) -> Result<Value, String> {
    let mut rng = StdRng::seed_from_u64(args.seed);
    let c = random_prime_order_curve(&mut rng, args.bits).map_err(|e| e.to_string())?;
    let xi = rand_below(&mut rng, c.order);
    let q = c.mul(xi, c.g);
    let mut s = Solver::new(c, args.m, Box::new(SparseContractor::default()), rng);
    if args.gadgets {
        s.gadgets = true;
        s.counter = Box::new(DFSCounter);
    }
    if !json_mode {
        s.logf = Some(Box::new(|msg: &str| eprintln!("{msg}")));
    }
    let (got, st) = s.solve(q).map_err(|e| e.to_string())?;
    let verified = c.mul(got, c.g) == q;
    Ok(json!({
        "schema_version": 1, "operation": "tensor-demo",
        "status": if verified && got == xi { "complete" } else { "error" },
        "seed": args.seed, "m": args.m, "gadgets": args.gadgets,
        "curve": {"p": c.p, "a": c.a, "b": c.b, "order": c.order,
                  "generator": [c.g.x, c.g.y], "target": [q.x, q.y]},
        "network": stats(&st.network),
        "factor_base_size": st.factor_base_size, "relations": st.relations,
        "attempts": st.attempts, "restarts": st.restarts,
        "wall_seconds": st.elapsed.as_secs_f64(),
        "result": {"expected": xi, "recovered": got, "verified": verified},
        "scope": SCOPE,
    }))
}

pub fn display(report: &Value) {
    match report["operation"].as_str() {
        Some("tensor-scale") => {
            println!("{}", report["header"].as_str().unwrap_or(""));
            for row in report["rows"].as_array().into_iter().flatten() {
                println!("{}", row["line"].as_str().unwrap_or(""));
            }
        }
        Some("tensor-demo") => {
            let c = &report["curve"];
            println!(
                "curve  y^2 = x^3 + {} x + {} over F_{}, prime order r = {}",
                c["a"], c["b"], c["p"], c["order"]
            );
            let n = &report["network"];
            println!(
                "network: {} edges, {} vertices, max degree {}, graph={}",
                n["edges"], n["vertices"], n["max_degree"], n["is_graph"]
            );
            println!(
                "N = {}, relations = {}, attempts = {}, restarts = {}, wall {:.3} s",
                report["factor_base_size"],
                report["relations"],
                report["attempts"],
                report["restarts"],
                report["wall_seconds"].as_f64().unwrap_or(0.0)
            );
            let r = &report["result"];
            println!(
                "recovered xi = {} (secret {}) — verified [xi]P == Q: {}",
                r["recovered"], r["expected"], r["verified"]
            );
        }
        _ => {}
    }
    println!("Scope: {}", report["scope"].as_str().unwrap_or(""));
}
