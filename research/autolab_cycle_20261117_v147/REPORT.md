# C147: static pivot,changed hard diagnostic,and explicit sparse blocks

The static-first-pivot variant applies exact Schur elimination before dynamic5x6decoding. Its unit is bound to C145's complete truth-plane fingerprint;rare rank2/3cells retain all free/compatibility conditions. The unchanged source-product cost ceiling222912is passed by215415clauses,down from C146224700,but still above C138178329. Independent matrix/RHSoracles,all32768d2coefficient assignments including12positive repeated-source normtuples,and32complete actual cofactor controls pass. Every original63selector and the exact raw five-source-product verifier remain charged. This is not original339variable completeness or a runtime comparison.

Actual evidence:

{
  "static": {
    "out": {
      "matrix_rhs_oracles": 128,
      "tiny_original_coefficient_assignments": 32768,
      "tiny_exact_coefficient_tuples": 12,
      "actual": {
        "scope": "fixed conditional k3source-polynomial product,not original full roots",
        "inputs": 61,
        "quotient_inputs": 56,
        "free_slots": 5,
        "unpruned_nodes": 99867,
        "variables": 60207,
        "clauses": 215415,
        "old_C138_clauses": 178329,
        "clause_ratio": 1.2079639318338578,
        "before_decoder_nodes": 10076,
        "after_decoder_nodes": 13586,
        "complete_quotient_cases": 32,
        "free_assignments_checked": 1024,
        "exact_necessary_fiber_tuples": 62,
        "accepted_raw_products": 0,
        "graph_build_seconds": 0.1632818249927368,
        "graph_sha256": "95bc0648577ea0a903d03e89a9fbc4325584b657a72961c6a9562846c89545d1",
        "development_cost_target_met": true,
        "certified_static_first_pivot": true,
        "full_graph_intermediate_retained": false
      },
      "original_complete": false,
      "all_registered_contexts_complete": true,
      "total_seconds": 0.3296447609900497,
      "graph_artifact_sha256": "80e661488d898198e508f7f75e46f466989f13961612eefcf7f1fc19e4acfb30",
      "post_exec_memory": {
        "start": {
          "VmSize": "36616 kB",
          "VmHWM": "10416 kB",
          "VmRSS": "10416 kB"
        },
        "finish": {
          "VmSize": "87972 kB",
          "VmHWM": "137080 kB",
          "VmRSS": "61200 kB"
        }
      }
    },
    "error": null
  },
  "native": {
    "out": {
      "models": [],
      "original_complete": false,
      "original_exact_root_count": null,
      "independent_unsat_certificate": false,
      "verified_original_tuple_lower_bound": 0,
      "scope": "fixed necessary k3family;otherstrata/source/U/ghost/outercoverage remains incomplete",
      "recovery_oracles": 12,
      "original_instance_sha256": "5c9be468c207cc2b013123b4265ee0963e96490c16aad1b1650a3528c0ce0c25",
      "native_gate_oracle_models": 6,
      "inputs": 61,
      "variables": 60207,
      "clauses": 215415,
      "graph_sha256": "95bc0648577ea0a903d03e89a9fbc4325584b657a72961c6a9562846c89545d1",
      "version": "4.15.4",
      "backend_configuration_readback": {
        "threads": "1",
        "cut": "true",
        "cut.xor": "true",
        "cut.aig": "true",
        "cut.delay": "0",
        "anf": "false"
      },
      "cpu_affinity": [
        0
      ],
      "formulation_seconds": 4.079883099999279,
      "engine_check_seconds": 35.98228356600157,
      "last_answer": "unknown",
      "termination": "unknown",
      "reason_unknown": "timeout",
      "statistics": {
        "sat mk clause 2ary": "231507",
        "sat mk clause 3ary": "89287",
        "sat mk clause nary": "459757",
        "sat mk var": "119932",
        "sat gc clause": "169926",
        "sat del clause": "353583",
        "sat conflicts": "202505",
        "sat decisions": "265044",
        "sat propagations 2ary": "34909498",
        "sat propagations nary": "148244317",
        "sat restarts": "14428",
        "sat minimized lits": "3237673",
        "sat subs resolution dyn": "48986",
        "sat units": "1076",
        "sat elim bool vars res": "28528",
        "sat backjumps": "202505",
        "sat elim clauses": "1630",
        "sat elim literals": "2752",
        "sat subsumed": "10849",
        "sat subs resolution": "8954",
        "sat bce": "1",
        "sat ate": "831",
        "sat scc elim vars": "60501",
        "sat scc elim binary": "124",
        "sat tr": "167",
        "sat probing assigned": "165",
        "sat-cut.eqs": "520",
        "sat-cut.cuts": "319714",
        "sat-cut.ands": "24586",
        "sat-cut.xors": "35137",
        "sat-cut.xands": "15718",
        "sat-cut.xites": "16362",
        "sat-cut.xxors": "22855",
        "sat-cut.dc-reduce": "11731",
        "rlimit count": "184110535",
        "max memory": "256.22",
        "memory": "86.61",
        "num allocs": "40198699318.0",
        "time": "35.98"
      },
      "full_diagnostic_seconds": 40.07080042301095,
      "post_exec_memory": {
        "start": {
          "VmSize": "36384 kB",
          "VmHWM": "10044 kB",
          "VmRSS": "10044 kB"
        },
        "finish": {
          "VmSize": "379836 kB",
          "VmHWM": "331900 kB",
          "VmRSS": "330040 kB"
        }
      },
      "complete": false,
      "exact_root_count": null,
      "all_registered_contexts_complete": false,
      "complete_early_census": false,
      "conditional_parameter_upper_bound": null,
      "global_original_root_count": null
    },
    "error": "nativeunknown"
  },
  "block": {
    "out": {
      "matrix_rhs_oracles": 128,
      "tiny_original_coefficient_assignments": 32768,
      "tiny_exact_coefficient_tuples": 12,
      "actual": {
        "scope": "fixed conditional k3source-polynomial product,not original full roots",
        "inputs": 61,
        "quotient_inputs": 56,
        "free_slots": 5,
        "unpruned_nodes": 99920,
        "variables": 60260,
        "clauses": 215615,
        "old_C138_clauses": 178329,
        "clause_ratio": 1.209085454412911,
        "before_decoder_nodes": 9675,
        "after_decoder_nodes": 13185,
        "complete_quotient_cases": 32,
        "free_assignments_checked": 1024,
        "exact_necessary_fiber_tuples": 56,
        "accepted_raw_products": 0,
        "graph_build_seconds": 0.4077224250067957,
        "graph_sha256": "d3cbac54921c93105b63d8bb6479efb167ffebc4be57c8eb3d44b04c783c8752",
        "development_cost_target_met": false,
        "certified_static_first_pivot": true,
        "early_inputs": 24,
        "tail_inputs": 32,
        "matrix_support_q_mask_hex": "0xffffff",
        "rhs_tail_degree_upper_bounds": [
          2,
          2,
          2,
          2,
          2,
          2
        ],
        "static_reference_clauses": 215415,
        "ratio_to_static_reference": 1.0009284404521506,
        "full_graph_intermediate_retained": false
      },
      "original_complete": false,
      "all_registered_contexts_complete": true,
      "total_seconds": 0.5883747030165978,
      "graph_artifact_sha256": "929a910376918e1b738f4e2dc8b1798eccc740b90ffbb0ba19fdff0c1c986784",
      "post_exec_memory": {
        "start": {
          "VmSize": "36872 kB",
          "VmHWM": "10588 kB",
          "VmRSS": "10588 kB"
        },
        "finish": {
          "VmSize": "88152 kB",
          "VmHWM": "137684 kB",
          "VmRSS": "61760 kB"
        }
      }
    },
    "error": null
  },
  "supervisor": {
    "static": {
      "status": "ok",
      "wall_seconds": 0.7407841089880094,
      "peak_process_rss_kib": 962604
    },
    "native": {
      "status": "ok",
      "wall_seconds": 40.22943492300692,
      "peak_process_rss_kib": 331900
    },
    "block": {
      "status": "ok",
      "wall_seconds": 1.182315280981129,
      "peak_process_rss_kib": 137684
    }
  },
  "workers": 3,
  "qualified_speedup": null,
  "goal_complete": false
}

A separately frozen changed native diagnostic loads the exact graph artifact and checks both graph hashes,clause count,passing profile/outer receipt,and original339variable332equation snapshot hash. Native gate enumeration and12independent GF4source/Ufiber controls pass. Input/oracle/formulation costs are included in40ssoft/90souter,with72soriginal-recovery guard and one-modelcap. The actual answer is UNKNOWN(timeout),with zero models;this is not an empty set or any speed ratio. Original per-model recovery remains unexercised at full width because no model returned;whole original completeness/counts stay false/null. Full backend statistics and frozen seed/configuration remain in raw evidence. No unchanged hard retry is authorized by this result.

The third hypothesis replaces the generic56coordinate quotient with24early-image witnesses plus32tail directions completing K16inside K8. Exact direct-sum ranks and zero early images are checked. The emitted matrix's syntactic support is exactlywithin24early inputs;all six conservative tail-degree bounds are2,as predicted by23<3*8. The final exact source-product graph nevertheless grows to215615clauses,200more than the static reference,refuting the strict cost-improvement target. Structural block separation is proved;runtime benefit is unobserved. All old source/plans/raw are preserved;the three workers have distinct frozen assumptions/inputs and run once under4GiB/CPU0caps.

The next hypothesis is bounded canonical XOR sharing:represent affine XOR forms over opaque nonlinear gate atoms,without distributing ANDor expanding whole ANF. Preserve exact fallback recipes when a term cap is reached. Independently verify every small linear/zero/singular/source-root catalogue case,then charge actual graph/extraction/memory costs against215415clauses before another changed native diagnostic. This differs from the already refuted dense ANF/ZDDapproaches by keeping nonlinear products opaque. The explicit24early/32quadratic-tail/7delayed structure remains available for later algebraic solvers.

Whole-childRSSand separately labeled post-exec VmHWM/VmRSSare retained;fresh-child/parent-ledger launch contexts differ,so no algorithm memory gain is claimed. Normal root resume/run-ready executed the first component;append-only childledger/c147executed native and block components after preregistration/admission. Full root and child validation receipts and portable hash replay are retained. The exact immutable graph artifacts are included in the raw archive;the original instance snapshot remains frozen in C135's preserved evidence with its named hash.

Schur/Gaussian elimination,Hensel correction,prefix extraction and block basis changes are established. Complete generalized-rootfinding novelty and the20hard-instance genuine matchedF4/F5/F6gate remain unestablished. Original source/U/ghost/other-leading/outer coverage,all requested fields/curves/scales,threecold repeats,fastestbaseline>=1s,A/A<=10%,qualified isolation and2xfloor/50xstretch remain active and unpassed. Prime/additive frontends are still queued.
