//! Native source/protocol report. No science worker or natural-yield run.
use std::{fs, path::Path};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: REPORT REPOSITORY NEW_REPORT_DIR".into());
    }
    let root = Path::new(&args[0]);
    let out = Path::new(&args[1]);
    let proof = root.join("research/extension_ic_graph_20261115/validation/pair48");
    for (name, needle) in [
        (
            "worker-tests-final.log",
            "test result: ok. 36 passed; 0 failed",
        ),
        (
            "checker-tests-v5.log",
            "test result: ok. 143 passed; 0 failed",
        ),
        ("worker-clippy-v2.status", "worker-clippy exit=0"),
        ("checker-clippy-v2.status", "checker-clippy exit=0"),
        (
            "transport.log",
            "PASS: three-arm positive/first-failure continuation",
        ),
    ] {
        if !fs::read_to_string(proof.join(name))?.contains(needle) {
            return Err(format!("source receipt {name}").into());
        }
    }
    fs::create_dir_all(out)?;
    let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="230" viewBox="0 0 1000 230">
<defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0 0L8 4L0 8" fill="none" stroke="#234b68"/></marker></defs>
<rect width="1000" height="230" fill="#f3f7fa"/>
<g fill="none" stroke="#234b68" stroke-width="3"><path d="M185 100H390" marker-end="url(#arrow)"/><path d="M570 100H750" marker-end="url(#arrow)"/></g>
<g fill="white" stroke="#234b68" stroke-width="2"><rect x="20" y="65" width="165" height="75" rx="14"/><rect x="390" y="65" width="180" height="75" rx="14"/><rect x="750" y="65" width="225" height="75" rx="14"/></g>
<g font-family="DejaVu Sans" text-anchor="middle" fill="#17384f"><text x="102" y="95" font-size="20">Ground x</text><text x="102" y="123" font-size="18">Aₓ = R₀</text><text x="285" y="62" font-size="18">R₁ = Dₓ + Dᵧ</text><text x="480" y="95" font-size="20">Ground y</text><text x="480" y="123" font-size="18">Aᵧ = R₁ − R₀</text><text x="655" y="62" font-size="18">R₂ = Dₓ − Dᵧ</text><text x="862" y="95" font-size="20">Cancel exactly</text><text x="862" y="123" font-size="18">W = −2R₀ + R₁ + R₂</text><text x="500" y="190" font-size="19">Large-class incidence is zero over the integers; scalar-field rank is checked separately.</text></g></svg>"##;
    fs::write(out.join("grounded-witness.svg"), svg)?;
    let typ = r##"#set page(paper: "a4", margin: (x: 15mm, y: 13mm))
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: 0.58em)
#set heading(numbering: none)
= From principal norms to a grounded graph
*Rust source and Runpod protocol checkpoint · 17 November 2026*

The new production entrypoint `run_extension_quadratic_pair_dlp_on` admits two quadratic cofactor classes and passes exact signed witnesses into the existing sparse-rank and scalar-recovery pipeline. This report records implementation and finite controls; no new natural-yield or performance run has occurred. PCC-20261117-92f18d is UNVERIFIED producer evidence. [pcc:unverified]

== Algebraic admission
For a genus-three cover over Fp², construct the degree-nine norm A² − C²h. Nine, seven or five distinct small points leave zero, one or two quadratic factors. The bounded quartic splitter checks squarefreeness, absence of Fp-linear factors and the degree-two Frobenius condition, then attempts at most eight recorded equal-degree splits. Each admitted quadratic is irreducible, has denominator coprime to C and a valid ordinate v = −A/C modulo U. The complete factorization and both signed canonical classes are checked before graph admission. [pcc:unverified]

#image("grounded-witness.svg", width: 100%)

A source with one known endpoint extends a grounded witness; two known endpoints give an exact cancellation. Ungrounded edges are explicitly discarded. Expanding source witnesses as integers preserves coefficient 2. Sparse kernel certificates establish actual scalar-field rank. The existing full-only Nagao descent, protected affine solve and elliptic scalar check are retained initially. [pcc:unverified]

== Controls and independent replay
#table(columns: (1fr,1fr,1fr), inset: 6pt, stroke: 0.4pt,
  [*Worker controls*],[*Qualifier controls*],[*Transport*],
  [36 pass, serialized],[143 pass, serialized],[Three-arm simulation passes])

Both optimized native builds and scoped strict Clippy pass. Independent coefficient arithmetic checks line constraints, norms, small support, class signs and ordinates; direct Cantor addition checks each original relation. Independent graph replay checks stored/emitted integer incidence, including −2R₀ + R₁ + R₂ and corruption refusals. Complete candidate/split-RNG/cohort replay, dense rank, sparse traces and full affine checks are wired into the separate qualifier. Original build/lint and parallel global-accounting test failures are retained. These finite checks do not supply a signed execution verdict. [pcc:unverified]

== Registered measurement
Three fixed-order cold processes: original IC, current quadratic sparse/table collector, grounded two-quadratic collector. Same exposed p1009/seed389 public G/Q, binary, host and one allowed CPU; zero warmups, retries or exclusions. Bounds: 262144 unique sources and quartic histories, 65536 grounded classes, witness support 256, eight split attempts; 1018081 B values in batches of 1024; 8192 descent residuals and 32 accepted descents. [pcc:unverified]

Runpod allocation: 4 CPU / 8 GiB, at most 0.40 USD/hour, absolute 6000-second owned-runner lifetime. Fixture: 60 seconds; each worker: 900; execution wrapper: 3000; separate qualifier: 1200. Publish/hash-freeze source, protocol and binaries before allocation. Verify a detached deletion guardian before the sole dispatch; collect, hash, scan and delete before interpretation. [pcc:unverified]

Native counts remain incomplete, so complete costs and ratios remain null. This exposed, single fixed-order trial cannot admit primary rho parity; fresh unknown-log/scaling comparisons, strongest matched rho and signed independent admission remain separate obligations. [pcc:unverified]

*Sources checked:* Gaudry–Thériault–Thomé, three-author February 2005 preprint §§2.3/3.1/3.3 (author copy); Cavallar filtering; Shoup, author text §20.2; retained46/47 implementations and43 zero-anchor descent result. KB unavailable; original AMS Cantor–Zassenhaus PDF returned403. No novelty, uniform cofactor-class distribution or transferred asymptotic bound is claimed. Full URLs and SHA-256 identifiers are in Amendment43. [pcc:unverified]
"##;
    fs::write(out.join("source-report.typ"), typ)?;
    Ok(())
}
