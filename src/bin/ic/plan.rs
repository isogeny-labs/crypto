//! `ic plan`: the configuration the measured evidence says to run on a curve.
//!
//! Reads the curve's class, size, field and automorphisms and asks the
//! planner (`ic_framework::planner`) for every `ic.pipeline` stage, with the
//! rule, the reason, how far the curve is from what was measured, the files
//! the rule rests on, and where the plan is expected to stand against the
//! matched rho.  Measures nothing; `ic bench --auto` runs the plan, and
//! `--ecbench-arm` prints it as an ecbench spec arm.
use clap::Args;
use crypto_lib::cryptanalysis::ecbench::workload::{CurveSpec, Instance};
use crypto_lib::cryptanalysis::ic_framework::planner::{
    self, CurveClass, CurveProfile, Goal, PlanOptions,
};
use serde_json::{json, Value};

#[derive(Args, Clone, Debug)]
pub struct PlanArgs {
    /// The curve, as an ecbench workload curve (`{"kind":"prime_explicit",…}`,
    /// `{"kind":"koblitz","a":0,"n":41}`, `{"kind":"prime_search","bits":28,"seed":1}`,
    /// `{"kind":"binary_random",…}`); `@path` reads it from a file.
    #[arg(long, conflicts_with_all = ["prime_bits", "koblitz", "binary_degree"])]
    pub curve: Option<String>,
    /// Shorthand for `{"kind":"prime_search","bits":B,"seed":SEED}`.
    #[arg(long)]
    pub prime_bits: Option<u32>,
    /// Shorthand for `{"kind":"koblitz","a":A,"n":N}`, written `A:N`.
    #[arg(long)]
    pub koblitz: Option<String>,
    /// Shorthand for `{"kind":"binary_random","n":N,"seed":SEED,"max_cofactor":8}`.
    #[arg(long)]
    pub binary_degree: Option<u32>,
    /// Seed for the `--prime-bits` and `--binary-degree` searches.
    #[arg(long, default_value_t = 1)]
    pub seed: u64,
    /// `one-target` (stop when the target is pinned; the leaderboard's
    /// comparison) or `log-database` (every base logarithm, full rank).
    #[arg(long, default_value = "one-target")]
    pub goal: Goal,
    /// Memory the pair table may take, in GiB; the base shrinks to fit.
    #[arg(long, default_value_t = 8.0)]
    pub max_table_gib: f64,
    /// Also print the plan as an ecbench spec arm of this name.
    #[arg(long)]
    pub ecbench_arm: Option<String>,
}

impl PlanArgs {
    fn curve_spec(&self) -> Result<CurveSpec, String> {
        let text = if let Some(c) = &self.curve {
            match c.strip_prefix('@') {
                Some(path) => {
                    std::fs::read_to_string(path).map_err(|e| format!("--curve {path}: {e}"))?
                }
                None => c.clone(),
            }
        } else if let Some(bits) = self.prime_bits {
            json!({"kind": "prime_search", "bits": bits, "seed": self.seed}).to_string()
        } else if let Some(k) = &self.koblitz {
            let (a, n) = k
                .split_once(':')
                .ok_or("--koblitz takes A:N, for example 0:41")?;
            let a: u8 = a.parse().map_err(|_| "--koblitz: A is 0 or 1")?;
            let n: u32 = n.parse().map_err(|_| "--koblitz: N is the field degree")?;
            json!({"kind": "koblitz", "a": a, "n": n}).to_string()
        } else if let Some(n) = self.binary_degree {
            json!({"kind": "binary_random", "n": n, "seed": self.seed, "max_cofactor": 8})
                .to_string()
        } else {
            return Err("name a curve: --curve, --prime-bits, --koblitz or --binary-degree".into());
        };
        serde_json::from_str(&text).map_err(|e| format!("curve: {e}"))
    }
}

/// The planner's profile of a built ecbench curve.
pub fn profile_of(inst: &Instance) -> CurveProfile {
    match inst {
        Instance::Prime(p) => CurveProfile::of_prime(p),
        Instance::Binary(b) => CurveProfile::of_binary(b),
        // A Koblitz curve past one word: the planner has no recipe for it,
        // but names the Frobenius rho.
        Instance::Wide(w) => CurveProfile {
            class: CurveClass::Koblitz,
            p: None,
            n: Some(w.kc.n),
            group_order: w.kc.group_order as f64,
            r: w.kc.r as f64,
            cofactor: w.kc.cofactor as u64,
        },
        // These research-only families do not have an IC planner profile.
        Instance::Fp4Torsion(_)
        | Instance::F2m4(_)
        | Instance::Fp4(_)
        | Instance::ThreeLeads(_)
        | Instance::Ext(_) => CurveProfile::other(0.0, 0.0, 0),
    }
}

pub fn run(args: PlanArgs, _json: bool) -> Result<Value, String> {
    let spec = args.curve_spec()?;
    let inst = spec.build()?;
    let profile = profile_of(&inst);
    let opts = PlanOptions {
        goal: args.goal,
        max_table_bytes: args.max_table_gib * (1u64 << 30) as f64,
    };
    let plan = planner::plan(&profile, &opts);
    let mut report = json!({
        "schema_version": 1,
        "operation": "plan",
        "status": "complete",
        "curve_spec": spec,
        "plan": plan,
        "ic_pipeline_params": plan.pipeline_params(),
    });
    if let Some(name) = &args.ecbench_arm {
        report["ecbench_arm"] = plan.ecbench_arm(name, "candidate").unwrap_or(Value::Null);
    }
    Ok(report)
}

/// The human-readable form of a plan report.
pub fn display(report: &Value) {
    let plan = &report["plan"];
    let pr = &plan["profile"];
    println!(
        "Curve: {}  class {}  log2 r {:.2}  cofactor {}",
        report["curve_spec"],
        pr["class"].as_str().unwrap_or("?"),
        pr["r"].as_f64().map_or(f64::NAN, f64::log2),
        pr["cofactor"]
    );
    println!(
        "Goal: {}   planner {}",
        plan["goal"].as_str().unwrap_or("?"),
        plan["planner"].as_str().unwrap_or("?")
    );
    for c in plan["choices"].as_array().into_iter().flatten() {
        println!(
            "\n  {:<12} {}\n               rule {} ({})\n               {}",
            c["stage"].as_str().unwrap_or("?"),
            c["value"].as_str().unwrap_or("?"),
            c["rule"].as_str().unwrap_or("?"),
            c["support"].as_str().unwrap_or("?"),
            c["reason"].as_str().unwrap_or("?")
        );
        for e in c["evidence"].as_array().into_iter().flatten() {
            println!("               · {}", e["path"].as_str().unwrap_or("?"));
        }
    }
    let rho = &plan["rho"];
    println!(
        "\nAgainst `{}`: {}  ({}; {})",
        rho["reference"].as_str().unwrap_or("?"),
        rho["ic_over_rho"]
            .as_f64()
            .map_or("unmeasured".to_string(), |x| format!(
                "about {x:.1}× rho's S"
            )),
        rho["support"].as_str().unwrap_or("?"),
        rho["basis"].as_str().unwrap_or("?")
    );
    for w in plan["warnings"].as_array().into_iter().flatten() {
        println!("Warning: {}", w.as_str().unwrap_or("?"));
    }
    println!("\nVerdict: {}", plan["verdict"].as_str().unwrap_or("?"));
    if let Some(p) = report["ic_pipeline_params"].as_object() {
        let line: Vec<String> = p
            .iter()
            .map(|(k, v)| format!("--{} {}", k.replace('_', "-"), v.as_str().unwrap_or("?")))
            .collect();
        println!("Run it: ic bench --auto … (or ic bench {})", line.join(" "));
    }
    if !report["ecbench_arm"].is_null() && report.get("ecbench_arm").is_some() {
        println!(
            "\necbench arm:\n{}",
            serde_json::to_string_pretty(&report["ecbench_arm"]).unwrap()
        );
    }
}
