# C75: test of Vieta rationality cut and actual join preflight

Hypothesis: the scalar Vieta trace condition may reject candidates of the affine relaxation before full norms/root reconstruction. Its scope is necessary rational-point parity, not general source support. Zero roots are removed before applying reciprocal Vieta, and multiplicities remain. A full primitive degree5 norm satisfies the trace identity automatically; the proposed cut can only help before those identities have been imposed.

Independent small oracles compare the coefficient formula with literal point rationality on root multisets, including zero and repeated roots. Even numbers of nonrational roots are an explicit false-positive control. Full norm identities are tested separately. Actual exposed affine samples and arbitrary support controls are distinguished: rejecting a support-valid polynomial outside the actual rational-root contract is not a solver improvement.

Recorded outcome (unmodified raw details remain):

```json
{
  "qualified_speedup": null,
  "small": {
    "cases": 34616,
    "all_rational": 2532,
    "mixed_even_false_positive": {
      "n": 3,
      "A": 0,
      "roots": [
        0,
        2,
        2
      ],
      "H": [
        0,
        4,
        0,
        1
      ]
    },
    "mixed_odd_rejection": {
      "n": 3,
      "A": 0,
      "roots": [
        0,
        0,
        2
      ],
      "H": [
        0,
        0,
        2,
        1
      ]
    },
    "zero_root_cases": 8846,
    "norm_identity_cases": 616
  },
  "trace_mask_hex": "0x200000000000000000001",
  "batch_inverse_seconds_development": 0.010843221010873094,
  "batch_trace_seconds_development": 0.010295964995748363,
  "individual_trace_seconds_development": 0.48623417000635527,
  "fresh_rejected": 0,
  "old_relaxed_rejected": 0,
  "supported_controls_rejected": 15,
  "join_preflight": {
    "nonzero_rational_x": 130723,
    "nonrational_x": 131420,
    "torsion_x": 1,
    "source_points": 261447,
    "finite_unordered_pair_records": 34177266904,
    "record_bytes": 16,
    "flat_pair_bytes": 546836270464,
    "source_index_bits": 18,
    "trace_mask_hex": "200000000000000000001",
    "cap_bytes": 4294967296,
    "refuse_flat_materialization": true,
    "complete_domain_profile": true,
    "seconds": 0.0206039,
    "peak_rss_kib": 12288
  }
}
```

Worker error: None. A passed diagnostic establishes neither an all-roots solve nor a qualified speedup. Single development phase timings are not cold performance ratios. The old/fresh rejection counts must be assessed separately from the proved redundancy on complete norms.

The native profile counts actual GF83 domain point types without enumerating source pairs. It computes the exact footprint of the specified 16-byte, one-record-per-finite-pair representation. Any refusal applies to that format, not to every compact algorithm. No oversized pair table is allocated, and no image-collision estimate is reported as a measurement.

Next: if parity is redundant on the affine family too, retire it and test higher Vieta bits of the rationality-label image, which can detect two/four anti-rational roots. First derive a compact construction with positive controls and account for its cost; do not repeat an unchanged filter. Replace the refused pair-fiber format with a genuinely compressed image representation or a new work-bounded search mechanism. Existing historical three-source emptiness certificates were discovered during design and must be reused by hash rather than rerun as new evidence.

One frozen 90-second, 4-GiB, one-core diagnostic ran through resume/run-ready. The scientific 50x goal remains active; no generic novelty, full hard-root list or genuine F4/F5/F6 superiority is claimed. Session and publication duration are recorded separately in budget receipts.
