//! Recorded mathematics and retained instruction-profile report, generated in Rust.
use serde_json::{json, Value};
use std::{fmt::Write as _, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: TRACE_REPORT STUDY_ROOT NEW_OUTPUT_DIR".into());
    }
    let root = Path::new(&args[0]);
    let out = Path::new(&args[1]);
    let five = read(&root.join("validation/unit-trace-recorded-proof.json"))?;
    let six = read(&root.join("validation/six-trace-recorded-proof.json"))?;
    for p in [&five, &six] {
        if p["systems"] != 3415
            || p["roots"] != 3419
            || p["failures"] != 0
            || p["new_natural_yield_calls"] != 0
            || p["elapsed_claim"] != false
        {
            return Err("recorded proof contract".into());
        }
    }
    let arms = [
        ("six", "Six coordinates"),
        ("unit", "Original unit"),
        ("fast", "Normalized unit"),
        ("trace", "Unit with trace"),
    ];
    let mut rows = Vec::new();
    let mut table = String::new();
    let mut chart = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="940" height="252" viewBox="0 0 940 252"><rect width="940" height="252" fill="white"/><g font-family="DejaVu Sans" fill="#17334a"><text x="15" y="24" font-size="18">Fixed first 32 captured systems: reported solver multiplications</text>"##,
    );
    for (i, (id, label)) in arms.iter().enumerate() {
        let c = read(&root.join(format!(
            "coordinate-profile-20261123/compact-control-{id}.json"
        )))?;
        if c["systems"] != 32
            || c["roots"] != 34
            || c["coordinate_sha256"]
                != "7c09ce05178a16e3b27c3c3c07115c6a4c021ef9c7e15c12ddefb1aae5c05e4f"
        {
            return Err("compact output binding".into());
        }
        let f: u64 = c["solver_reported_f4_muls"]
            .as_str()
            .ok_or("F4 count")?
            .parse()?;
        let l: u64 = c["solver_reported_linear_muls"]
            .as_str()
            .ok_or("linear count")?
            .parse()?;
        let y = 48 + i * 40;
        let fw = f as f64 / 30_000_000.0 * 570.0;
        let lw = l as f64 / 30_000_000.0 * 570.0;
        write!(
            chart,
            r##"<text x="170" y="{}" text-anchor="end" font-size="15">{label}</text><rect x="185" y="{y}" width="{fw:.2}" height="22" fill="#367da5"/><rect x="{:.2}" y="{y}" width="{lw:.2}" height="22" fill="#d98e39"/><text x="{:.2}" y="{}" font-size="14">{} + {}</text>"##,
            y + 17,
            185.0 + fw,
            193.0 + fw + lw,
            y + 17,
            f,
            l
        )?;
        writeln!(table, "[{label}],[{f}],[{l}],")?;
        rows.push(json!({"arm":id,"f4_muls":f,"linear_muls":l,"systems":32,"roots":34,"scope":"solver-reported subtotal; no reduction/collection/whole-cost claim"}));
    }
    chart.push_str(r##"<rect x="185" y="218" width="16" height="13" fill="#367da5"/><text x="209" y="230" font-size="14">F4</text><rect x="275" y="218" width="16" height="13" fill="#d98e39"/><text x="299" y="230" font-size="14">linear stage</text><text x="15" y="248" font-size="12">Recorded correctness controls only. No new timing, natural-yield measurement or complete-cost ratio.</text></g></svg>"##);
    let diagram = r##"<svg xmlns="http://www.w3.org/2000/svg" width="940" height="210" viewBox="0 0 940 210"><defs><marker id="a" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto"><path d="M0 0L10 5L0 10" fill="#526879"/></marker></defs><rect width="940" height="210" fill="white"/><g font-family="DejaVu Sans" fill="#17334a" font-size="15" text-anchor="middle"><text x="470" y="22" font-size="18">Existing graph collection and opt-in target descent</text><g fill="#ecf3f7" stroke="#8299a8"><rect x="15" y="40" width="210" height="48" rx="6"/><rect x="250" y="40" width="210" height="48" rx="6"/><rect x="485" y="40" width="210" height="48" rx="6"/><rect x="720" y="40" width="205" height="48" rx="6"/></g><text x="120" y="69">Extension-field norm sieve</text><text x="355" y="69">Quadratic-factor screen</text><text x="590" y="69">Graph elimination</text><text x="822" y="69">Sparse kernel</text><g fill="#fff2df" stroke="#c79c61"><rect x="15" y="132" width="210" height="48" rx="6"/><rect x="250" y="132" width="210" height="48" rx="6"/><rect x="485" y="132" width="210" height="48" rx="6"/><rect x="720" y="132" width="205" height="48" rx="6"/></g><text x="120" y="161">Target divisor</text><text x="355" y="161">Exact unit reduction 6 to 5</text><text x="590" y="161">Checked F4 trace reuse</text><text x="822" y="161">Affine recovery + replay</text><g stroke="#526879" fill="none" marker-end="url(#a)" stroke-width="2"><path d="M225 64H247M460 64H482M695 64H717M225 156H247M460 156H482M695 156H717M822 88V129"/></g><text x="470" y="203" font-size="12">The new coordinate backend changes target descent; graph collection and sparse elimination remain the matched controls.</text></g></svg>"##;
    let md=format!("# Exact unit descent and checked trace reuse\n\nNative Rust integration provides opt-in normalized-unit, traced-unit and traced-six backends. Original entry points keep their original behavior. Workers explicitly clear trace state and separate arities. Every trace mismatch takes the existing checked fallback; refusals are retained.\n\nBoth full recorded controls match all 3419 roots on the same 3415 saved Run51 systems: five-variable replay/fallback {}/{}, six-variable {}/{}. These are recorded correctness observations, not new natural-yield or timing trials. Unsigned PCC-20261123-ecef76. [pcc:unverified]\n\n![Integration](flow.svg)\n\n![Partial coordinate counts](coordinate-counts.svg)\n\nThe four compact controls use the same first32 captured systems and34 roots, with one exact coordinate fingerprint. Counts come from the solver and exclude reduction, graph collection, other native control, hashing and serialization. No complete-cost, runtime or parity gain is inferred.\n\nThe earlier full-record Callgrind profiles include JSON/hash startup: original and normalized unit totals are 29290711138 and29289230219 simulated instructions. They are instruction attribution only. Original raw profiles and automated PDFs remain hash-bound LOCAL; long symbol tables in those PDFs overflow. The compact profiling schedule has not executed.\n\nThe full current-root library build fails on unchanged base syntax in binary_hyperelliptic_index_calculus.rs. Scoped binaries explicitly use the pinned Root41 library; scoped checks do not substitute for that gate. New Runpod measurement is blocked by the account balance and missing checkpoint upload route. No successor pod was created.\n\nSources: [five replay](../../validation/unit-trace-recorded-proof.json), [six replay](../../validation/six-trace-recorded-proof.json), [compact controls](../../coordinate-profile-20261123/), [native worker](../../../../examples/jv_sieve_fast_unit_deferred_compare.rs), [native per-arm checker](../../../../examples/jv_sieve_fast_unit_arm_check.rs), [Run51 observations](../../runpod-unit-deferred-20261117-51/report/REPORT.md). Joux–Vitse, §§3.1–3.3, [author PDF](https://www.lix.polytechnique.fr/~joux/notes/CoverDecomp.pdf); existing unit-elimination source e4af92fbc62ac21b6ae53d45d961de50ed60275f. KB unavailable; no novelty claim.\n",five["replayed"],five["shape_fallbacks"],six["replayed"],six["shape_fallbacks"]);
    let typ = format!(
        r###"#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
= Exact unit descent with checked trace reuse
Rust extension-field graph IC · recorded mathematics · 23 November 2026

#image("flow.svg", width: 100%)

The graph collector and sparse kernel stay fixed. Opt-in target descent uses a constant-unit substitution to reduce six coordinates to five, then the existing F4 learning/replay mechanism. Every shape mismatch takes the checked full fallback. Dedicated worker calls clear the cache and separate arities before work.

== Full captured-cohort correctness
#table(columns:(2fr,1fr,1fr,1fr),align:(left,right,right,right),
 [Backend],[Systems / roots],[Replayed],[Fallbacks],
 [Five coordinates],[3415 / 3419],[{}],[{}],
 [Six-coordinate control],[3415 / 3419],[{}],[{}],)
Both match every captured root set and every original equation, with zero failures. No new relation search or elapsed improvement follows. Producer evidence is *UNVERIFIED*, PCC-20261123-ecef76. [pcc:unverified]

== Fixed first 32 systems: partial solver counters
#image("coordinate-counts.svg", width: 100%)
#table(columns:(2fr,1fr,1fr),align:(left,right,right),[Backend],[F4 multiplies],[Linear multiplies],{table})
All four controls match the same 34 roots and coordinate fingerprint. These counters exclude reduction, collection, native control and serialization; they are not complete pipeline costs.

== Profile and execution limits
Full-record Callgrind totals are 29290711138 / 29289230219 simulated instructions (old / normalized unit), dominated by unrelated JSON/hash startup. The compact profile schedule remains unexecuted. Instruction profiles are not timing measurements.

Current-root library tests fail to compile unchanged base syntax in binary_hyperelliptic_index_calculus.rs. Scoped binaries use the explicitly pinned Root41 library. Runpod balance and unavailable checkpoint upload block a new paid comparison; no successor pod exists.

Sources: #link("https://github.com/isogeny-labs/crypto/pull/4986")[PR 4986], retained replay JSON and compact controls beside this report; native source trace_integration_report.rs. Prior art: Joux–Vitse §§3.1–3.3 and the existing exact unit/F4 trace implementations. Original Run51 qualifier137 and separate replay0 remain unchanged. No complete-cost ratio or parity claim.
"###,
        five["replayed"], five["shape_fallbacks"], six["replayed"], six["shape_fallbacks"]
    );
    fs::create_dir(out)?;
    for (file, text) in [
        ("REPORT.md", md),
        ("report.typ", typ),
        ("flow.svg", diagram.into()),
        ("coordinate-counts.svg", chart),
    ] {
        fs::write(out.join(file), text)?;
    }
    fs::write(
        out.join("plot-data.json"),
        serde_json::to_vec_pretty(&rows)?,
    )?;
    Ok(())
}
