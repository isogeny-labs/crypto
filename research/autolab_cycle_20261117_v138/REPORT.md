# C138: guarded delayed-block reconstruction

Hypothesis:compile the C137joint tangent system into an exact rank-aware substitution,retaining every singular compatibility/free-coordinate branch. State-dependent particular operators,kernel bases,left-null constraints and active-slot flags are emitted as complete small Boolean functions. Unused free slots are forced0to prevent duplicate models. Coefficient-level fusion reconstructs the original family and shares the prefix before17. The final raw-product verifier and every unconsumed correction remain mandatory.

Compact actual evidence and any error:

{
  "out": {
    "matrix_rhs_oracles": 4096,
    "generic_catalog_state_checks": 512,
    "original_complete": false,
    "total_seconds": 4.860532925988082,
    "tiny": {
      "inputs": 14,
      "quotient_inputs": 12,
      "free_slots": 2,
      "variables": 158,
      "unpruned_variables": 181,
      "cofactor_nodes_before_final_prune": 86,
      "early_bits": 5,
      "states": 32,
      "function_count": 16,
      "distinct_functions": 10,
      "monomial_count": 12,
      "rank_distribution": {
        "1": 8,
        "2": 24
      },
      "reference_assignments": 32768,
      "compiled_assignments": 16384,
      "exact_component_tuples": 12,
      "exact_consumed_zero_slice": 8192,
      "tuple_sha256": "4b3ea38137c935615dc1a4a283bd2371057a4cf64423e3b44da387f8465f6e59"
    },
    "actual": {
      "inputs": 62,
      "quotient_inputs": 60,
      "free_slots": 2,
      "variables": 49587,
      "unpruned_variables": 75382,
      "cofactor_nodes_before_final_prune": 10040,
      "early_bits": 8,
      "states": 256,
      "function_count": 23,
      "distinct_functions": 20,
      "monomial_count": 229,
      "rank_distribution": {
        "1": 16,
        "2": 144,
        "3": 96
      },
      "clauses": 178329,
      "baseline_clauses": 170149,
      "clause_ratio": 1.0480755102880417,
      "profile_admitted_for_changed_diagnostic": true,
      "coefficient_checks": 128,
      "root_unconsumed_checks": 128,
      "guard_pass_samples": 31,
      "inverse_readback_checks": 128,
      "catalog_state_checks": 256,
      "graph_sha256": "b86a060c8c61a9655ddbd0320f225ddd82c659e4a211c219dfcf7ad31bc467cf",
      "no_native_solve": true,
      "profile_after_oracles_seconds": 3.203158125979826
    }
  },
  "error": null
}

The actual input layout is60quotient coordinates plus at most2free delayed slots;there is no uniform63-to60map. Rank3states use neither slot,rank2states use one,rank1states use both. Compatibility constraints retain the precise right-hand-side condition rather than discarding singular states. The proof gives a bijection on accepted inputs. On incompatible inputs,the consumed residuals may differ because the guards reject them;unconditional coefficient/root equivalence and guarded consumed-equation equivalence are the relevant contracts.

Generic controls exhaust all512binary3x3matrices,all8rhs and all8slot assignments against brute force,including rank0and inconsistent systems. Every generic512-state emitted catalog and actual256-state catalog is checked against its numeric records. The full tiny source graph is exhausted,with exact consumed-zero slices and complete original-selector/root tuple sets compared to the guarded graph. Repetitions,canonical-slot uniqueness,coefficient identity and inverse readback are explicit. Actual128checks covercoefficients,root/unconsumed constraints,guarded consumed corrections,state indexing,basis inversion and all76original norm/qbits. Full details,bases and raw process records remain in the hashed archive.

Supervisor wall seconds and peak process RSS in KiB:

{
  "status": "ok",
  "wall_seconds": 5.00193891802337,
  "peak_process_rss_kib": 807988
}

The whole graph/CNF profile includes cofactor construction,lookup/ANF functions,compatibility/canonical-slot guards,root-product and all recovered wires. The prerecorded development limit is1.10times the170149-clause one-row fused baseline. A passed component or a smaller graph does not establish faster native solving. C137's conditional7*2^58parameter bound is not an original root count or speedup. This worker runs no native solve and adds no new prime/odd-extension solver coverage.

One fresh90souter/4GiB/CPU0experiment ran once,with max200kpre-prune nodes and earlyrank<=8. All old code,inputs,plans,raw evidence and unsuccessful runs remain preserved. Portable replay checks hashes,complete small accepted sets and the actual guarded graph without native re-execution. Pinned M4RIprimary source establishes rank-aware binary solves with inconsistency checking as prior art;the exact guarded-family integration has no completed novelty or qualified performance claim.

Next: Freeze this exact guarded graph and native identity,40ssoft90souter4GiBCPU0budget,and full quotient/free-slot-to63selector-to76norm-qreadback. Preserve canonical-slot and compatibility guards,root-product verification and original332-equation per-model recovery. UNKNOWN is unresolved;no timeout ratios. Independently preregister prime-field symbolic tangent oracles as a separate stream.

Complete original source/U/ghost/outer enumeration,requested field/curve coverage,independent complete root sets and20hard genuine matchedF4/F5/F6instances with3coldrepetitions,RSS and isolation remain active and unpassed. Neither2xnor50xend-to-end superiority is established.
