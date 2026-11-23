# C136: coefficient-level unit parity fusion

Hypothesis:replace selector-level substitution by exact affine coefficient fusion to expose cancellations before nonlinear gates. For each coefficient bit,b+a*theta becomes b+a*x+parity(a*v)*h under the certified reconstruction theta=x+v*h. Emit the parity once. Since the actual D_v vanishes below precision17,early cofactor and root-generator work can share identical coefficient wires. This changes the formulation rather than retrying C135's unresolved native run.

The proof preserves the C133unit certificate,C134bijection,allq lifts,root multiplicities,remaining correction constraints and the final raw-product verifier. Only the certified consumed row is replaced by zero. The old affine76-bit norm/q map is applied to reconstructed63selectors,never to62free coordinates. New coefficient-readback groups are recovery-only and are not asserted zero.

Actual evidence and any error:

{
  "out": {
    "tiny_reference_assignments": 32768,
    "tiny_variants": [
      {
        "free_inputs": 14,
        "old_selectors": 15,
        "cofactor_nodes_before_final_prune": 24,
        "unpruned_variables": 103,
        "variables": 76,
        "direction_polynomials_hex": [
          "0x4",
          "0x0",
          "0x0",
          "0x0",
          "0x0",
          "0x0"
        ],
        "direction_hex": "0x2",
        "exhaustive_assignments": 16384,
        "exact_component_tuples": 12,
        "exact_slice_count": 16384,
        "tuple_sha256": "4b3ea38137c935615dc1a4a283bd2371057a4cf64423e3b44da387f8465f6e59"
      },
      {
        "free_inputs": 14,
        "old_selectors": 15,
        "cofactor_nodes_before_final_prune": 24,
        "unpruned_variables": 104,
        "variables": 77,
        "direction_polynomials_hex": [
          "0xc",
          "0x0",
          "0x0",
          "0x0",
          "0x0",
          "0x0"
        ],
        "direction_hex": "0x6",
        "exhaustive_assignments": 16384,
        "exact_component_tuples": 12,
        "exact_slice_count": 16384,
        "tuple_sha256": "4b3ea38137c935615dc1a4a283bd2371057a4cf64423e3b44da387f8465f6e59"
      }
    ],
    "actual": {
      "free_inputs": 62,
      "old_selectors": 63,
      "cofactor_nodes_before_final_prune": 7867,
      "unpruned_variables": 73189,
      "variables": 47473,
      "direction_polynomials_hex": [
        "0x1dade2a03afccdc9e20000",
        "0xb657c37e891ce0000",
        "0x613525e400000",
        "0x39d3c0000",
        "0x0",
        "0x0"
      ],
      "clauses": 170149,
      "prior_unit_clauses": 182771,
      "prior_unit_variables": 50810,
      "clause_ratio_to_prior_unit": 0.930940904191584,
      "clauses_removed": 12622,
      "profile_admitted_for_changed_diagnostic": true,
      "source_compile_seconds": 0.2901757200015709,
      "fusion_seconds": 0.09130886098137125,
      "coefficient_equivalence_checks": 128,
      "constraint_root_equivalence_checks": 128,
      "original_theta_q_readback_checks": 128,
      "graph_sha256": "31e996958ea5a59d3a1fa083bcd4653e3a679effaa808f2d6f0a9b12638b75ee",
      "no_native_solve": true,
      "profile_after_oracles_seconds": 3.9101399189967196
    },
    "original_complete": false,
    "total_seconds": 5.88567860701005
  },
  "error": null
}

Supervisor wall seconds and peak process RSS in KiB:

{
  "status": "ok",
  "wall_seconds": 6.009758210013388,
  "peak_process_rss_kib": 796280
}

Small oracles exhaust32768reference assignments and16384transformed assignments per direction,comparing complete zero-correction slices,complete original-selector/root component tuples,every normalized coefficient,all constraints,and inverse readback. Repeated sources must survive. Actual128fixed-seed checks compare scalar affine coefficients,the original graph's constraints/five roots,and all original theta/q bits. These are implementation diagnostics;universal equivalence follows from coefficient parity and the frozen unit certificate. Complete component tuples are not complete original339-coordinate root sets.

Whole graph/CNF counts include cofactor construction and selector/coefficient recovery wires. A smaller circuit,or a component runtime difference,is not a qualified end-to-end gain. This worker ran no native search and added no new prime/odd-extension solver coverage. C135's UNKNOWN40.276s/794844KiBRSS/zero models remains preserved without being converted into a speed ratio.

Novelty review:affine normalization,functional cofactoring and structural hashing are established. The prior pinned Berkeley ABC and Z3primary-source receipts distinguish those implementations from the specific guarded-family adaptation. No general algorithm novelty or complexity improvement is established by this compiler experiment.

Next: Before another native run,prove and test joint affine tangent rows18through20 on the3-dimensional delayed kernel. Admit complete early rank<=8;enumerate all256early states,report3x3rank/pivot distribution and retain every singular branch. Then choose a complete block compiler or the admitted fused graph diagnostic;see BLOCK_ELIMINATION_NEXT.md and NEXT_DECISION.json.

Independent field stream:the C135prime-field tangent note proves affine early-state row interpolation at n=ell+1<2ell. Implement17-field independent oracles before applying that unexecuted theory to prime/odd-extension systems. All requested original source/U/ghost/outer branches,full field/curve coverage,20hard genuine matchedF4/F5/F6completed instances,independent root sets,threecold repetitions,peak memory and qualified isolation remain active and unpassed. No qualified2xor50xspeedup or memory gain is claimed.
