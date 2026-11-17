//! Native source/protocol report; starts no measurement.
use serde_json::Value;
use std::{fs, path::Path};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: REPORT REPO NEW_OUTPUT".into());
    }
    let root = Path::new(&a[0]);
    let out = Path::new(&a[1]);
    let receipts = root.join("research/extension_ic_graph_20261115/validation/deferred50");
    for (file, needle) in [
        ("worker-tests.log", "test result: ok. 45 passed; 0 failed"),
        (
            "checker-tests-v2.log",
            "test result: ok. 158 passed; 0 failed",
        ),
        ("transport.log", "PASS: eight-arm positive"),
    ] {
        if !fs::read_to_string(receipts.join(file))?.contains(needle) {
            return Err(format!("source receipt {file}").into());
        }
    }
    let plan: Value = serde_json::from_slice(&fs::read(
        root.join("research/extension_ic_graph_20261115/deferred-pair-plan.json"),
    )?)?;
    if plan["schema"] != "cover.deferred-pair.runpod-validation/v1"
        || plan["mode_order"].as_array().map(Vec::len) != Some(8)
        || plan["worker_timeout_seconds"] != 360
        || plan["timeout_seconds"] != 3200
        || plan["max_deferred_edges"] != 262144
    {
        return Err("registered protocol".into());
    }
    let replay: Value =
        serde_json::from_slice(&fs::read(receipts.join("independent-recorded-graph.json"))?)?;
    if replay["sources"] != 108058 || replay["independent_full_incidence_witnesses"] != 865 {
        return Err("recorded-source control".into());
    }
    fs::create_dir_all(out)?;
    let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="210" viewBox="0 0 1000 210"><rect width="1000" height="210" fill="white"/><g fill="#eef5f9" stroke="#275c78" stroke-width="2"><rect x="5" y="30" width="205" height="65" rx="8"/><rect x="250" y="30" width="205" height="65" rx="8"/><rect x="495" y="30" width="235" height="65" rx="8"/><rect x="770" y="30" width="225" height="65" rx="8"/></g><g stroke="#275c78" stroke-width="3" fill="none"><path d="M210 62H250M455 62H495M730 62H770"/><path d="M610 95V140H110V95"/></g><g font-family="DejaVu Sans" text-anchor="middle" font-size="20" fill="#173c50"><text x="107" y="69">Screened sources</text><text x="352" y="59">Unit anchor</text><text x="352" y="83">Queue activation</text><text x="612" y="59">Exact witness</text><text x="612" y="83">+ activation source</text><text x="882" y="69">Sparse rank + scalar</text><text x="500" y="185">Compare: original IC / table / grounded screen / deferred / 4 rho arms</text></g></svg>"##;
    fs::write(out.join("deferred-flow.svg"), svg)?;
    let typ = r##"#set page(paper:"a4",margin:(x:15mm,y:13mm))
#set text(font:"DejaVu Sans",size:8.5pt)
#set par(leading:0.4em)
= Deferred signed graph IC: registered Runpod comparison
*Rust producer / complete independent replay / protocol50 · 17 November 2026*

The new separate collector retains checked two-class edges until a unit anchor activates them. It combines the established discriminant-screened relation finder with bounded queue elimination, explicit activation-source records, sparse homogeneous rank and unchanged full-only Nagao/affine elliptic recovery. Natural-yield and elapsed results for this entrypoint are pending. PCC187567/1bb597are UNVERIFIED. [pcc:unverified]

#image("deferred-flow.svg",width:100%)

== Why measure this successor
Recorded49source projection yields865full rows versus522from the grounded-discard graph. The516-row threshold moves from complete batch185344to155648, with actual rank508in the recorded prefix. This motivates testing whether earlier rank outweighs extra graph state, queue work and longer witnesses. It is not a new natural-yield, sparse-solve or performance measurement. Source/component98fe3e3b and PCCe9bd73/e9a8cepreserve that result. [pcc:unverified]

Run49screened pairs recovered the scalar with observed incomplete subtotal6235988188572, compared with6441136952022unscreened. Original IC4525556132414and table5230830441710were stronger in that unit. The frozen timer-equality failure and separate successful replay remain unchanged in their result packet. [pcc:unverified]

== Source controls and independent completeness
#table(columns:(1fr,1fr,1fr),inset:6pt,stroke:0.4pt,
 [*Worker controls*],[*Qualifier controls*],[*Transport*],
 [45pass; serialized],[158pass; serialized],[Eight-arm simulation passes])

Both native optimized builds and scoped strict Clippy pass. A flat ordered incidence index independently reconstructs every queue event, unlike the producer's per-node lists with stale ids. The checker requires all exact integer anchors/full witnesses and explicit activation-source prefixes. A balanced-cycle control demonstrates that the newest anchor can cancel from a final older witness. Independent replay of all108058hash-bound recorded49sources matches865full incidences and all event order/statistics. [pcc:unverified]

Full live qualification checks every original norm/principal/Cantor relation, bounded split, discriminant, candidate/refusal/cohort/RNG stream, exact graph projection, dense actual rank, sparse certificate and full affine/elliptic endpoint. Common complete prefixes must have equal original certificates and split/screen histories; graph/rank records are checked separately. Normalized kernel/descent/affine data and postcollection primitive phases must agree. Independent elapsed times are retained without equality requirements. [pcc:unverified]

== Frozen comparison and bounded execution
Eight fixed-order cold processes: original IC; quadratic table; grounded screened pairs; deferred screened pairs; rho plain/negation with batch1; rho plain/negation with batch16. Same exposed p1009/seed389public target, binary/host/first allowed CPU, zero warmups/retries/exclusions. Rho uses the existing r256/dp16registered portfolio, with every configuration executed and checked. This is the available four-configuration reference, not a globally optimal rho claim. [pcc:unverified]

Both pair arms:524288slow histories,1000000early refusals,262144sources,256witness support,8split attempts. Grounded graph65536vertices; deferred graph262144vertices and262144lifetime deferred edges. B budget1018081;1024B batches;8192descent residuals/32accepted. Sparse surplus8/stride16/8checkpoints/3preconditioners. Explicit resource/failed/capped outcomes stay in the record. [pcc:unverified]

Runpod4CPU/8GiB≤0.40USD/hour;7200second absolute lifetime; live detached guardian checked before sole dispatch. Fixture60seconds; eight workers360each; wrapper3200; qualifier1800. Require6400seconds remaining before dispatch. Publish/freeze source/binaries/protocol before allocation; collect/hash/private-scan/delete before interpretation. [pcc:unverified]

Complete native totals/ratios remain null; fresh/scaling and signed independent admission remain open. Native controls use the declared immutable Root41library; unchanged base dependency errors prevent a fresh whole-root build. No scoreboard promotion. Primary graph source: Gaudry–Thériault–Thomé corr2004-29§3.1–3.3; Swan1962Corollary1/§4supports the necessary discriminant test. KB unavailable; no novelty/asymptotic claim. [pcc:unverified]
"##;
    fs::write(out.join("source-report.typ"), typ)?;
    fs::write(out.join("figure.typ"),"#set page(width:750pt,height:157.5pt,margin:0pt)\n#image(\"deferred-flow.svg\",width:750pt,height:157.5pt)\n")?;
    Ok(())
}
