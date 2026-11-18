# C134: exact unit-cofactor integration and total graph profile

Hypothesis: consume the certified c18unit direction by embedding62free selectors with pivot1fixed0,compute h=c18(x),and reconstruct all63original selectors theta=x+v*h. Clone the original constraints and root outputs at theta. The universal certificate makes c18(theta)=0,so downstream uses of that correction can be replaced by zero. The final raw-product verifier remains mandatory. PROOF_AND_REVIEW.md gives the inverse bijection,gate-by-gate equivalence,readback contract and scope.

Actual evidence and any worker error:

{
  "out": {
    "tiny_reference_assignments": 32768,
    "tiny_variants": [
      {
        "direction_hex": "0x2",
        "pivot": 1,
        "source_inputs": 15,
        "free_inputs": 14,
        "cofactor_nodes_before_final_prune": 23,
        "unpruned_nodes": 76,
        "pruned_nodes": 76,
        "exhaustive_assignments": 16384,
        "exact_slice_count": 16384,
        "exact_component_tuple_count": 12,
        "component_tuple_sha256": "4b3ea38137c935615dc1a4a283bd2371057a4cf64423e3b44da387f8465f6e59"
      },
      {
        "direction_hex": "0x6",
        "pivot": 1,
        "source_inputs": 15,
        "free_inputs": 14,
        "cofactor_nodes_before_final_prune": 23,
        "unpruned_nodes": 77,
        "pruned_nodes": 77,
        "exhaustive_assignments": 16384,
        "exact_slice_count": 16384,
        "exact_component_tuple_count": 12,
        "component_tuple_sha256": "4b3ea38137c935615dc1a4a283bd2371057a4cf64423e3b44da387f8465f6e59"
      }
    ],
    "actual": {
      "direction_hex": "0x25bc29bd1e36b336",
      "pivot": 1,
      "source_inputs": 63,
      "free_inputs": 62,
      "cofactor_nodes_before_final_prune": 3426,
      "unpruned_nodes": 50810,
      "pruned_nodes": 50810,
      "source_graph_sha256": "256c3b08787e5c1352c6b9518e838c4b9c0791152f94d5c68cc5170d9c5f6768",
      "source_variables": 47430,
      "source_clauses": 169977,
      "new_clauses": 182771,
      "clause_ratio": 1.0752690069832977,
      "profile_admitted_for_changed_diagnostic": true,
      "source_compile_seconds": 0.22987840400310233,
      "transform_seconds": 0.0883410090173129,
      "equivalence_checks": 128,
      "original_theta_q_readback_checks": 128,
      "no_native_solve": true,
      "graph_sha256": "939a9facabf64e2c4efe8e57b81fa3c8475955f372502b624c01ffa4462eb2ab",
      "profile_after_oracles_seconds": 3.6046451659931336
    },
    "original_complete": false,
    "total_seconds": 5.153709091973724
  },
  "error": null
}

The tiny reference exhausts32768assignments;each declared transformed direction exhausts16384free assignments. Complete original-selector/root tuple sets and complete zero-correction slices are compared in memory,with exact counts and tuple hashes retained. The zero assignment explicitly exercises repeated sources. These are complete component sets,not original339-coordinate all-root sets. Actual fixed-seed comparisons exercise all remaining constraints,five root words,selector reconstruction,inverse mapping,and original73norm-plus3quotient readback. Universality rests on the frozen C133certificate and compiler induction,not sampling.

The whole graph and CNF counts include the reconstructed-selector recovery wires. They are not just an isolated correction-cone profile. Original76-bit theta/q decoding is explicitly applied after reconstructing the old63selectors;the old affine map is never applied directly to the62free selectors. The recorded clause ratio is a development admission test only. One fewer search selector does not predict a2xsolve speedup,and graph growth can negate it. No native solve ran.

Supervisor wall seconds and peak process RSS in KiB:

{
  "status": "ok",
  "wall_seconds": 5.334553104999941,
  "peak_process_rss_kib": 771796
}

One fresh experiment ran once with90souter,4GiB,CPU0. Source/input hashes,old/current frozen manifests,raw attempts and negative results remain preserved. Portable replay checks immutable source/raw hashes and reconstructs both complete tiny component tuple sets and the actual graph hash. Berkeley ABC's pinned primary source establishes cached reachable-DAG duplication and structural hashing as prior art;C133 separately pins FLINT Hensel lifting. No new generic compiler technique or broad algorithm novelty is claimed.

Next: Freeze a changed bounded native diagnostic against this exact graph hash,with full62-to63selector and76-bit theta/q readback,final raw product,source-root extraction and original recovery checks. Charge all compile/parse/setup/serialization and RSS. Keep unresolved/timeout evidence and all original outer/ghost branches open. No exact UNSAT without independent certification.

The complete original source/U/ghost/outer system,requested field/curve coverage,matched genuine F4/F5/F6interfaces and20hard-instance release gate remain active and unpassed. Current Boolean-selector integration does not add new prime/odd-extension solver coverage;C133's17-field checks concern the tangent component only. No qualified2xor50xend-to-end or memory gain is claimed.
