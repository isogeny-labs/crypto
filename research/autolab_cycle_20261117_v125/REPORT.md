# C125: preserve XOR sharing, then clip impossible Hensel coefficients

The first hypothesis preserves the first XOR/NOT recipe for each canonical nonlinear-value mask, emitting shared binary recipes rather than flattening masks. Induction preserves all observable constraint and root values; each emitted operation is charged to a source operation. The proposed node and weighted-CNF non-growth bound passed 401 checks. Independent exhaustive tests covered 80 circuits, 400 configurations, 12,000 assignments and 288,000 value comparisons, with 1,143 exact cuts, 64 deep-path assignments, source immutability and a deliberate work-cap rejection.

The fixed C123 verifier has 53,794 variables and 192,485 clauses (source graph SHA-256 1f77cc2df83d9ec0d4d7cb42f6ad0b795b75da40f061cfe4cf78c6a84b6b8733). Recipe limits 32/64/128 yielded respectively 53,316/190,667; 53,291/190,570; 53,280/190,526 variables/clauses. Each passed 32 full observable-vector controls. The preregistered training rule selects 128, graph beea3463cb922635b0f69bf33934fe3f5466fcf9db5007961584f44fb3310145. This is only a 1.02% clause reduction and is worse than C124's four-term 184,526-clause calibration. A new hard timeout on this spelling would add little information, so the third worker tests a different algebraic assumption.

The fresh hypothesis is degree-clipped Hensel generation followed by an exact raw root-product certificate. The lift is triangular in precision. On every supported k3 quintic, the true normalized factors have coefficient bounds deg(A_j)<=(2-j)(d-1), deg(B_j)<=(3-j)(d-1). Their leading forms Z^2 and (Z+1)^3 are coprime. Unique formal lifting and induction show that coefficients beyond these bounds are zero; omit their corrections before later recurrences. For unsupported inputs arbitrary generated root words may change. This is safe because acceptance still requires the exact raw quintic to equal the product of all five generated domain roots. There is no modular/hash relaxation, and all quotient bits remain. Repetitions within a leading cluster are retained.

Third-worker evidence (raw, not a release claim):

{
  "out": {
    "actual_controls": 32,
    "formulation": {
      "inputs": 63,
      "variables": 47663,
      "clauses": 170851,
      "gate_kinds": {
        "false": 1,
        "true": 1,
        "input": 63,
        "not": 184,
        "xor": 27981,
        "and": 19435
      },
      "graph_sha256": "dbadc6bc3cbfa5d304262f20143ea00cc5c8a2dedd70173ebf1bef13d56865d1",
      "source_variables": 53794,
      "source_clauses": 192485
    },
    "exhaustive_assignments": 32768,
    "planted_assignments": 200,
    "random_assignments": 128,
    "accepted": 212,
    "q_alias_checks": 1,
    "original_complete": false,
    "cold_compile_seconds": 0.3473343680088874,
    "total_seconds": 2.288491987012094
  },
  "error": null
}

Process measurements (wall seconds and peak process RSS KiB):

{
  "xor-recipe-oracles-20261117-v125b": {
    "status": "ok",
    "wall_seconds": 0.4091572120087221,
    "peak_process_rss_kib": 608496
  },
  "xor-recipe-profile-20261117-v125b": {
    "status": "ok",
    "wall_seconds": 6.6156737669953145,
    "peak_process_rss_kib": 605472
  },
  "degree-clipped-oracles-profile-20261117-v125c": {
    "status": "ok",
    "wall_seconds": 2.36803239499568,
    "peak_process_rss_kib": 614200
  }
}

Decision: degree clipping passed; freeze a changed cold native diagnostic. No identical failed execution was repeated. Three workers ran with 90-second outer deadlines, 4-GiB limits, CPU0, and 78/84-second reporting boundaries. Measurements are development observations, with inherited RSS effects; no isolated matched runtime or memory ratio is inferred. The recipe profile took 6.615674 seconds and peaked at 605,472 KiB. Source compilation, checks and serialization are retained in raw receipts.

The output contract is conditional exact quintic support and its five roots with multiplicities for one fixed norm family and leading chart. It is not the complete original coupled source/U/ghost/outer-family solution. Other leading strata, prime/odd domains, all requested curves/fields, original root sets and genuine matched F4/F5/F6 baselines remain independent active work. The 20-hard-instance release gate is unchanged and unpassed; neither 2x nor the 50x stretch goal is established.

Portable replay rebuilds all recipe graphs and, when the worker completes it, the clipped graph, verifying hashes, counts and raw evidence. All source, inputs, plans, attempts, failures and local derived graphs are preserved.
