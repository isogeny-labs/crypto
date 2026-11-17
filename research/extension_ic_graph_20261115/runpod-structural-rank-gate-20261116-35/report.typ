#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let display_value(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
= Graph structure before a full rank check
16 November 2026 · Runpod CPU · Restricted weak Fp6 cover family

Source #raw(s.source_revision.slice(0,9)), p=#display_value(s.p),
seed=#display_value(s.seed), scalar order #display_value(s.l).
Both public-input workers recover #display_value(s.actual_scalar).
The generator holds out its planted answer; each worker uses placeholder0.
This is a graph-checkpoint component comparison. [pcc:unverified]

#table(columns:(190pt,130pt,130pt),align:left,inset:5pt,
 [Observation],[Rank stop],[Structural gate],
 [Full checkpoints performed],[#display_value(s.gate_proof.baseline_exact_checks)],[#display_value(s.gate_proof.candidate_exact_checks)],
 [Whole observed primitive subtotal],[#display_value(s.outcomes.at(0).accounting.observed_primitive_subtotal_bitops)],[#display_value(s.outcomes.at(1).accounting.observed_primitive_subtotal_bitops)],
 [Original source rows],[Same],[#display_value(s.gate_proof.same_original_sources)],
 [Descent rows / collection endpoint],[Same],[#display_value(s.gate_proof.same_descents) / #display_value(s.gate_proof.same_collection_endpoint)],
 [All phase primitive changes explained],[Required],[#display_value(s.gate_proof.phase_conservation.all_phase_counts_conserved)],
 [Complete operation cost],[Unknown],[Unknown],
)

#image("linear-algebra.svg",width:100%)

The candidate tracks omitted-point graph components and single-vertex
anchors with union-find. Its intended large-column rank lower bound is
V−C+A, giving an upper bound N−(V−C+A) on independent small-only rows. It
defers an exact checkpoint while that upper bound is below small_base−1.
Both arms advance the same source-stride checkpoint schedule, including
skipped checks. Exact graph elimination, combination replay, rank, affine
target determination and elliptic verification remain mandatory.
[pcc:unverified]

The copied Rust qualifier independently traverses each prefix graph and
computes its actual large-column rank with dense modular elimination. It
checks the bound on every gate prefix, exact source/descent/endpoint and
checkpoint schedule identity, permitted full-checkpoint equality, and all
remaining combination/rank/target proofs. Every primitive phase must equal
baseline minus skipped checkpoint vectors plus the known charged gate
events. [pcc:unverified]

Charged gate events cover state-array initialization, support entries,
parent probes and bound reads. DSU scalar updates/comparisons and other
native bookkeeping remain incomplete; parent-probe counts are source
emissions. This gives no complete pipeline cost, pricing band, ratio, T0
against the best IC pipeline or T1 against rho. [pcc:unverified]

Both use75% small base, full p² B budget and rank stride128. One cold process
per arm, zero warmups/retries;900s worker,1900s whole,2700s runner lifetime.
Dedicated4vCPU/8GiB Runpod CPU, actual pinned CPU #display_value(e.pinned_cpu).
Wall observations are practicality only. Builds finish before timing.
Exact source/binaries/argv/flow/status/raw/qualifier/hashes:
#raw(e.run_path). Artifact collection precedes owned pod termination;
the native export claims no unavailable canonical-schema conformance.
Producer checks carry no independent signed verdict. [pcc:unverified]
