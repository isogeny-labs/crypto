//! Presentation/evidence join in Rust. Existing native statistics are reused.
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let s = PathBuf::from(std::env::args().nth(1).expect("study"));
    let b = Path::new("research/prime_conductor_gap80_20261031/twist-transport-20261119");
    let base = read(b.join("summary.json"));
    let map = read(s.join("results/main-signed-v2/result.json"));
    let dual = read(s.join("results/dual-signed-v2/result.json"));
    assert_eq!(map["source"], base["source"]);
    assert_eq!(map["codomain"], base["target"]);
    assert_eq!(map["ell"], base["prime_isogeny_degree"]);
    let mut calls = Vec::new();
    let mut rows = String::new();
    for (label, title) in [
        ("main", "Rust first build: main +33 images"),
        ("dual", "Rust first build: dual +33 images"),
        ("main-signed-v2", "Rust final build: main +33 images"),
        ("probe-signed-v2", "Rust final build: auxiliary probe"),
        ("dual-signed-v2", "Rust final build: dual +33 images"),
        ("target80", "Original 80bit: first codec refusal"),
        ("target80-signed-v2", "Original 80bit: final Rust attempt"),
    ] {
        let d = s.join("results").join(label);
        let exit = fs::read_to_string(d.join("exit")).unwrap();
        let resource = fs::read_to_string(d.join("resources.txt")).unwrap();
        let wall = resource
            .split_whitespace()
            .find_map(|x| x.strip_prefix("wall_s="))
            .unwrap();
        rows.push_str(&format!("[{title}], [{wall}], [{}],\n", exit.trim()));
        calls.push(
            json!({"label":label,"exit":exit.trim(),"resources":resource,"RSS_observed":false}),
        );
    }
    assert_eq!(
        fs::read_to_string(s.join("results/target80-signed-v2/exit"))
            .unwrap()
            .trim(),
        "124"
    );
    assert!(!s.join("results/target80-signed-v2/result.json").exists());
    for name in ["main-audit.stdout", "dual-audit.stdout"] {
        assert!(fs::read_to_string(s.join("audit-signed-v2").join(name))
            .unwrap()
            .starts_with("PASS33"));
    }
    let archive = read(s.join("archive-info.json"));
    let summary = json!({"schema":"conductor.rust-port-evidence/v1","implementation":"Rust complete evaluator, existing Rust Mont/Jac source, FLINT/GMP primitive FFI; no C++ constructor","final_codec":"signed source a,b normalized modulo p; other numeric encodings remain strict","map":map,"dual":dual,"calls":calls,"qualified_reference_controls":read(s.join("qualification-c/summary.json")),"final_unit_tests":7,"new_port_clippy_passed":true,"unchanged_Mont_dependency_lints_excluded":2,"source_End_conductor":"1","target_End_conductor":"33555419","prime_ratio_integer_bits":26,"ordinary_order_argument":"../twist-transport-20261119/CM_ARGUMENT.md; exact equation identity checked above","independent_final_main_check":fs::read_to_string(s.join("audit-signed-v2/main-audit.stdout")).unwrap(),"independent_final_dual_check":fs::read_to_string(s.join("audit-signed-v2/dual-audit.stdout")).unwrap(),"original_80bit_crossing_complete":false,"full32_target_ECDLP_complete":false,"solver_baseline":base["ECDLP"],"solver_measurements_reexecuted_in_port":false,"scalar_baseline":base["scalar"],"raw_archive":archive,"world_record_claimed":false,"wall_admitted":false,"producer_evidence":"unsigned/not_covered"});
    fs::write(
        s.join("summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    let out = s.join("reports");
    fs::create_dir_all(&out).unwrap();
    let graph = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1050" height="270" viewBox="0 0 1050 270"><defs><marker id="a" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0L6,3L0,6" fill="none" stroke="#38688c"/></marker></defs><rect width="1050" height="270" fill="#f3f7fb"/><g font-family="DejaVu Sans" text-anchor="middle" fill="#244459"><rect x="35" y="80" width="350" height="100" rx="8" fill="#dfede4"/><text x="210" y="118" font-size="20">Rust ecbench isogeny evaluator</text><text x="210" y="145" font-size="15">coverage • jets • log norms • twist map</text><rect x="655" y="20" width="350" height="90" rx="8" fill="#deebf5"/><text x="830" y="57" font-size="19">Existing Rust Montgomery/Jacobian</text><text x="830" y="83" font-size="14">unchanged source; private field/point values</text><rect x="655" y="160" width="350" height="90" rx="8" fill="#deebf5"/><text x="830" y="197" font-size="19">FLINT/GMP polynomial primitives</text><text x="830" y="224" font-size="14">owned opaque contexts; small C allocation ABI</text><text x="518" y="194" font-size="14">SHA256-checked library</text></g><g stroke="#38688c" stroke-width="2" fill="none" marker-end="url(#a)"><path d="M400,110L635,65"/><path d="M400,150L635,205"/></g></svg>"##;
    fs::write(out.join("architecture.svg"), graph).unwrap();
    let text = r##"#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10.5pt)
#set par(leading: 0.6em)
= Rust evaluator: complete retained conductor crossing
19 November 2026. User requested Rust; all isogeny scheduling, jet/log-norm identities, coverage checks and twist transport are now Rust. Field/Jacobian arithmetic reuses the unchanged repository source. The small C ABI exposes existing FLINT/GMP primitives and allocations; it does not implement or call the C++ constructor. [pcc:unverified]

#image("architecture.svg", width: 100%)

Final source is src/bin/ecbench/isogeny_eval.rs with its private flint.rs ownership wrapper. Opaque contexts and polynomials retain Rc ownership of their library/context. Dynamic library SHA256 and ABI are checked. The supported ABI is Linux with 64-bit long; field coordinates stay four limbs and arbitrary public isogeny scalars remain BigUint. Source a,b accept signed residue representatives, matching the original input format; point/field/degree encodings stay strict. [pcc:unverified]

The final build passes seven arithmetic/coverage/parser tests,18 complete small-map comparisons against retained independent PARI references (470 point inputs; two fields; square/nonsquare twists; full/blocked rectangles), and eight rejection controls. Port code passes scoped clippy with warnings denied. Two original indexed-loop lints are excluded only on the unchanged included Montgomery dependency; original diagnostics remain. No full-repository test or formal-proof claim is made. [pcc:unverified]

The complete retained ordinary map has prime degree 33555419 and actual End-conductors 1→33555419, a26-bit prime ratio. The exact source/quotient equations and original prime subgroup 5629830654216971 are unchanged. Final Rust main/probe/dual runs independently pass 33 point/order checks,32 scalar identities and 33 normalized dual compositions. Their numeric output agrees with the retained reference. The standard CM conductor argument applies to these exact equations; see the adjacent CM_ARGUMENT.md and its fresh PARI arithmetic certificates. [pcc:unverified]

Source equation: a=5620419248410339, b=675862186879016. Quotient: a=2539833775737119, b=1737207970637906. Field prime 5629830687772387. Transported original G=(2975092980436849,5333622322807887). This evaluated milestone remains below the original 80-bit minimum. [pcc:unverified]

#pagebreak()
= Actual local calls and remaining goal
#table(columns: (2.7fr,1fr,0.6fr), inset: 4pt,
 [Whole process / actual outcome], [Elapsed seconds], [Exit],
 @ROWS@
)

All elapsed values are individual Bash whole-process L0 observations. They include executable/library hashing and loading, input/primality checks, quotient construction, supplied point evaluations and output. The300second cap for retained cases and 60second cap for the original 80-bit target are budgets; 12 GiB is an address-space limit, not observed RSS. No build/audit/benchmark overlapped a timed call. No separated phase cost, speedup or high-isolation admission is inferred. [pcc:unverified]

The first 80-bit call refused its−140 coefficient before construction. That exact refusal is retained separately. The final signed-input build then executed the unchanged original33-point input and reached its 60second watchdog (124), without quotient, images or durable partial-coverage journal. This is a censored attempt, not an impossibility or completion certificate. The original 80-bit crossing remains open. [pcc:unverified]

Existing matched Rust solver measurements on these identical retained endpoints are preserved as baseline, not re-run or relabelled by this port: four verified rho recoveries (source/image means 7.637/10.318seconds, one target/two rounds), four IC exhaustions at 10000 trials with zero relations, and 672 independently checked scalar batches over 32 bases/model and six measured rounds plus warmup. All eight solver attempts replay exactly. The word-field solver and four-limb scalar clocks are separate from constructor clocks. Full 32-target ECDLP and admitted performance remain unfinished. [pcc:unverified]

Exact sources, failed builds/role wiring/smoke/refusal attempts, pre-call hashes, raw output, independent checks and deterministic complete raw archive are preserved. The archive inventory records every extracted hash and byte-identical repack; it does not claim an S3 upload. Numerical libraries and qualified existing release-library build identities remain explicit. Producer evidence is unsigned/not_covered. [pcc:unverified]

References: existing Rust Montgomery/Jacobian source; native complete-evaluator and prepared-norm derivation; FLINT/PARI API contracts; ordinary-order/CM references linked in the retained CM_ARGUMENT.md. KB unavailable. No novelty, world-record or performance-leaderboard claim. [pcc:unverified]
"##;
    fs::write(out.join("report.typ"), text.replace("@ROWS@", &rows)).unwrap();
    let md=format!("# Rust evaluator implemented and executed\n\nThe actual ecbench command is `isogeny-evaluate`. Scheduling, jets, prepared norms, coverage, coefficient reconstruction and twist transport are Rust; existing Rust Montgomery/Jacobian code and FLINT/GMP primitives are explicit dependencies. It does not call the C++ constructor. [pcc:unverified]\n\nFinal build:7 tests,18 reference controls/470 points,8 rejections and scoped port clippy pass. Full33555419-degree retained map and normalized dual pass all 33 independent point/composition checks on original subgroup 5629830654216971. This is the same26-bit prime End-conductor crossing, not the original 80-bit goal. [pcc:unverified]\n\nThe exact original 80-bit input ran through the final Rust build and hit its 60second cap without quotient/images. The earlier unsigned-codec refusal is retained separately. Existing Rust ECDLP/scalar results are preserved baseline, not re-measured by this port. [pcc:unverified]\n\n[Two-page report](reports/report.pdf), [protocol](PROTOCOL.md), [signed input correction](SIGNED_INPUT_AMENDMENT.md), [native summary](summary.json), [raw archive](raw-results.tar.xz), [inventory](archive-info.json). Whole raw archive: {} bytes, SHA256 {}. Extraction/repack checks pass; no S3 upload/readback is claimed. [pcc:unverified]\n\nBuild argv and exact source/library/binary identities are in build-a through build-e and the pre-call manifests. Failed module-path/build-role/first smoke attempts remain. Current protocol has one post-call evidence label only; exact pre-call text is archived and mapped in protocol-annotation.json. Larger-family40-bit successor remains a separately proposed next experiment. Original80-bit and full 32-target ECDLP remain open. [pcc:unverified]\n",archive["bytes"],archive["sha256"].as_str().unwrap());
    fs::write(s.join("README.md"), md).unwrap();
    println!("native Rust evidence joined and report source rendered; no new timing statistics");
}
