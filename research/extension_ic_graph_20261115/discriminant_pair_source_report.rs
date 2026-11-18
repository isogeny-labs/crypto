//! Native source/protocol report only. No DLP or new natural-yield measurement.
use std::{fs, path::Path};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: REPORT REPO NEW_REPORT".into());
    }
    let root = Path::new(&a[0]);
    let out = Path::new(&a[1]);
    let receipts = root.join("research/extension_ic_graph_20261115/validation/discriminant49");
    for (name, needle) in [
        (
            "worker-tests-v2.log",
            "test result: ok. 37 passed; 0 failed",
        ),
        (
            "checker-tests-v2.log",
            "test result: ok. 149 passed; 0 failed",
        ),
        ("transport.log", "PASS: four-arm positive"),
    ] {
        if !fs::read_to_string(receipts.join(name))?.contains(needle) {
            return Err(format!("source receipt {name}").into());
        }
    }
    fs::create_dir_all(out)?;
    let diagram = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="210" viewBox="0 0 1000 210"><defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0 0L8 4L0 8" fill="none" stroke="#275c78"/></marker></defs><rect width="1000" height="210" fill="white"/><g fill="none" stroke="#275c78" stroke-width="2"><path d="M175 70H210M380 70H415M585 70H620M790 70H825" marker-end="url(#arrow)"/><path d="M295 100V145" marker-end="url(#arrow)"/></g><g fill="#eef5f9" stroke="#275c78" stroke-width="2"><rect x="5" y="40" width="170" height="60" rx="9"/><rect x="210" y="40" width="170" height="60" rx="9"/><rect x="415" y="40" width="170" height="60" rx="9"/><rect x="620" y="40" width="170" height="60" rx="9"/><rect x="825" y="40" width="170" height="60" rx="9"/><rect x="145" y="145" width="300" height="45" rx="9"/></g><g font-family="DejaVu Sans" font-size="19" text-anchor="middle" fill="#173c50"><text x="90" y="76">Quartic tail</text><text x="295" y="76">Resultant + χ</text><text x="500" y="76">Bounded split</text><text x="705" y="76">Signed graph</text><text x="910" y="76">Rank + scalar</text><text x="295" y="174">χ = 0 or −1: retain refusal</text><text x="397" y="30">χ = +1</text></g></svg>"##;
    fs::write(out.join("screen-flow.svg"), diagram)?;
    let typ = r##"#set page(paper: "a4", margin: (x: 15mm, y: 13mm))
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.50em)
= A scalar discriminant screen before graph relations
*Rust source / Runpod49 protocol checkpoint · 17 November 2026*

Run48's two-quadratic collector stopped at its quartic-history cap before rank admission. The new separate Rust entrypoint rejects impossible quadratic pairs by discriminant character, then retains the existing bounded splitter, principal certificates, exact graph witnesses, sparse rank and full-only Nagao/affine scalar recovery. New49 natural-yield/performance results are pending. PCC-20261117-3c2fcf/06f517 are UNVERIFIED. [pcc:unverified]

#image("screen-flow.svg", width: 100%)

== Necessary algebraic condition
Swan1962Corollary1 (crediting Stickelberger) states that a degree-n squarefree polynomial over an odd finite field has r irreducible factors with r ≡ n mod2 exactly when its discriminant is a square. Two distinct irreducible quadratics therefore require nonzero square discriminant. Positive character alone admits nothing. [pcc:unverified]

For a quartic f with leading coefficient a, a·Res(f,f′) = a²disc(f) represents the same square class and avoids a normalization inverse. An immutable typed ring context validates its prime once; counted Euclidean remainders compute the resultant. A separate u128 7×7 Sylvester determinant reconstructs it independently. Written derivation and full source citation are in Amendment44; no novelty or Lean-checked theorem is claimed. [pcc:unverified]

== Finite and retained-data controls
#table(columns: (1fr,1fr,1fr), inset: 6pt, stroke: 0.4pt,
 [*Worker controls*],[*Qualifier controls*],[*Transport*],
 [37 pass, serialized],[149 pass, serialized],[Four-arm simulation passes])

Both optimized builds and scoped strict Clippy pass. Four scalar-screen test groups include16906 exhaustive nonmonic quartics over p5/p7, malformed domains, mutation refusals and counted hook ownership. Every one of262144 retained48tails matches the independent determinant:174615 would skip the old splitter and87529 remain. All86093 previously admitted factorizations have positive character; every skipped old call used zero random polynomials. This is mathematical replay, with no new local DLP worker or speed measurement. [pcc:unverified]

== Registered complete-pipeline comparison
Four fixed-order cold processes: original IC, current quadratic-table collector, unscreened two-quadratic collector, screened two-quadratic collector. Same exposed p1009/seed389 public point, binary/host/first allowed CPU; zero warmups, retries or exclusions. Both pair arms have524288 slow histories,262144 unique sources,65536 classes, witness support256 and eight split attempts. Screen has at most1000000 compact early-refusal records. B budget1018081;1024B batches;8192 descent residuals/32 accepted. [pcc:unverified]

The qualifier reconstructs every early decision, original norm/Cantor relation, graph witness, candidate/refusal/cohort/RNG flow, actual rank, sparse trace and full affine/elliptic endpoint. It requires equal pair sources/graph/rank geometry, exact retained split histories, zero-draw-only removals and equal post-collection kernel/descent/affine data. Old48 failure and component replay remain preserved at their frozen source. [pcc:unverified]

Runpod:4CPU/8GiB, at most0.40USD/hour, absolute7200second lifetime. Fixture60seconds; each worker900; wrapper4000; qualifier1800. Require a live detached guardian and at least6400seconds remaining before sole dispatch. Publish/hash-freeze source/binaries/protocol before allocation; collect/hash/private-scan/delete before interpretation. [pcc:unverified]

Complete native costs and ratios remain null. Existing integer/control/container/PRNG gaps, exposed target, missing fresh/scaling comparison and signed strongest-reference admission prevent a primary rho-parity claim. Source controls do not supply an independent signed verdict. [pcc:unverified]

*Primary source:* Richard G. Swan, Pacific J. Math.12(3),1099–1106 (1962), Corollary1 and §4, https://msp.org/pjm/1962/12-3/pjm-v12-n3-p27-s.pdf. SHA256db8a73496c10f2a32549b31b673f3039d2b6ba1533a90b2d3fac3e81a2cfbdc2. KB unavailable (wrapper127); existing bounded splitting follows retained Shoup §20.2. No extension-class distribution or asymptotic assumption is transferred. [pcc:unverified]
"##;
    fs::write(out.join("source-report.typ"), typ)?;
    Ok(())
}
