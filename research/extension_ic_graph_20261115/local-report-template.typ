#let render(summary: none, plan: none, figure: none, title: "", run: "", duration: "", findings: none) = {
  set page(paper: "a4", margin: 15mm)
  set text(font: "DejaVu Sans", size: 9pt)
  set par(justify: true)
  heading(level: 1, title)
  [15 November 2026 · Public known-answer cubic curves · Local producer observations]
  parbreak()
  [All #summary.planned_execution_count planned executions completed and recovered:
  #summary.phase_counts.warmup retained warmups and #summary.phase_counts.measurement
  measured executions. Source #raw(summary.source_revision.slice(0, 9)), sampler
  #raw(summary.sampler), five paired timing rounds per stratum. Outer process wall:
  #duration seconds; exit zero. [pcc:unverified]]
  figure
  [Cold wall time divided by the same-instance existing cubic r-adding rho;
  geometric means and deterministic round-bootstrap 95% intervals. The intervals
  cover repeated timings on fixed curves, not independent curve samples.]
  parbreak()
  let fmt(x) = str(calc.round(x, digits: 2))
  table(columns: (1fr, 1.5fr, 0.8fr, 1.3fr, 0.9fr), inset: 3pt,
    [p / seed], [Graph oracle], [Wall / rho], [95% interval], [Local A/A],
    ..summary.strata.map(s => (
      [#s.p / #s.seed],
      [#if s.oracle == "SymmetricPairOnly" { [Symmetric pair] } else { [Symmetric triple] }],
      [#fmt(s.diagnostic_graph_wall_over_rho.geometric_mean)],
      [#fmt(s.diagnostic_graph_wall_over_rho.bootstrap_95.at(0))–#fmt(s.diagnostic_graph_wall_over_rho.bootstrap_95.at(1))],
      [#if s.eligible_local_controlled_comparison { [Pass] } else { [Rejected] }],
    )).flatten()
  )
  parbreak()
  findings
  parbreak()
  [Host: AMD EPYC 9V74, one pinned allowed CPU (CPU0), 4-CPU cgroup quota,
  8-GiB virtual-memory ceiling, 900-second process limit. No standard EKS/L2
  admission or strongest-external comparison is asserted. All base preparation,
  jump/start construction, restarts, collection, elimination and scalar replay
  are inside the per-arm cold wall interval. The legacy operation model remains
  incomplete. No independent signed verdict is available. [pcc:unverified]]
  parbreak()
  [Evidence: #raw(run). Its #raw("summary.json") comes from the native Rust
  analyzer; #raw("runtime-plan.json") pins source/binary and frozen inputs;
  #raw("local-evidence.json") records flow and the exact local raw path/hash.
  Raw JSONL/checkpoint/copied binaries/provenance remain under
  #raw("/workspace/ic-local-runs/" + run), with remote upload pending. There
  was one measurement attempt and zero retries. Canonical export validation
  is unavailable because its accessible source/schema are encrypted VLT1;
  this local record does not claim canonical validation. Plot label regeneration
  leaves all numerical strata unchanged. [pcc:unverified]]
}
