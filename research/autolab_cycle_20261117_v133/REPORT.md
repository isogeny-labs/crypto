# C133: delayed-perturbation tangent certificates

Hypothesis: a perturbation beginning at ell can affect the precision-n correction only linearly when n<2ell. Weighted homogeneity of the constant-Sylvester,degree-clipped recurrence proves this: two perturbation inputs would cost at least2ell. A single perturbation leaves base-coefficient weight at most n-ell. Therefore all base assignments can be covered by their exact early-coefficient image. The strict inequality is essential.

The fresh jet implementation differentiates the clipped recurrence using DA*B+A*DB. Independent finite-difference oracles use the earlier general Sylvester numeric lift. Small binary families exercise early truncation,17coefficient fields exercise the tangent identity,including odd extensions,characteristic3,GF(2^83),and all five NIST primes. The GF4 boundary control at n=2ell demonstrates that formal differentiation can miss a finite quadratic contribution;no result at that boundary is accepted.

Actual evidence,including any error and full certificate rows:

{
  "out": {
    "binary_families": 24,
    "binary_difference_checks": 192,
    "early_truncation_checks": 192,
    "field_contexts": 17,
    "field_difference_checks": 51,
    "boundary_checks": 1,
    "actual": {
      "ell": 17,
      "precision": 18,
      "kernel_dimension": 3,
      "kernel_hex": [
        "0x203fc8e0758104c",
        "0x10003f9d8cf98562",
        "0x25bc29bd1e36b336"
      ],
      "early_rank": 1,
      "early_representatives_hex": [
        "0x0",
        "0x1"
      ],
      "tangent_rows": [
        4,
        6
      ],
      "unit_combinations": [
        4,
        5
      ],
      "unit_directions_hex": [
        "0x25bc29bd1e36b336",
        "0x27bfd533196ea37a"
      ],
      "finite_difference_checks": 14,
      "circuit_checks": 128,
      "selector_reconstruction_checks": 128,
      "graph_sha256": "256c3b08787e5c1352c6b9518e838c4b9c0791152f94d5c68cc5170d9c5f6768"
    },
    "original_complete": false,
    "total_seconds": 5.624891835002927
  },
  "error": null
}

The complete delayed-direction space contains 2 universal unit directions for the first necessary correction. These are alternative directions; their number is not the number of independently eliminated bits. No substitution has been integrated yet.

For this actual chart ell17,n18 gives a3-dimensional kernel preserving all coefficients below17 and an early image of rank1. Enumerating both reachable early states and all7 nonzero kernel combinations completely decides this restricted unit-direction question. The universal certificate follows from the graded proof and exhaustive image coverage. Random checks against the unchanged complete correction circuit are implementation checks,not universal proof by sampling. The original affine base,q lifts and frozen graph hash remain bound.

Supervisor measurements(seconds,KiB RSS):

{
  "status": "ok",
  "wall_seconds": 5.9398754829890095,
  "peak_process_rss_kib": 769572
}

These are one-run diagnostic costs,not an all-roots timing,memory gain or qualified ratio. A certified direction yields a bijection:fix one pivot bit0 and reconstruct x*=x+v*c18(x). Every other constraint must use x*,including original theta/q recovery and the exact raw product. This checkpoint does not execute that integration. It solves no new complete original source/U/ghost/outer instance.

Primary review inspected FLINT's established residual/inverse Hensel update;the pinned URL and source hash are in PRIMARY_SOURCE_RECEIPT.json. Hensel lifting and differentiation are prior art. Novelty of a productive conditional-search adaptation remains unestablished. The strongest specialized competitor includes exact conditional polynomial factorization,not only basis algorithms.

One fresh scientific experiment was registered and executed once with90souter,4GiB,CPU0. Source/input hashes,frozen manifests,previous attempts and raw process evidence remain preserved. The portable replay checks immutable hashes and re-derives the actual tangent rows. Qualified original-all-root F4/F5/F6 comparisons,20preregistered hard completed matching instances,threecold repeats,isolation,independent root sets and full requested field/curve coverage remain open. Release gate unchanged and unpassed;neither2xnor50xis claimed.

Next: Select a certified direction deterministically; build an invertible63-to62 selector chart; reconstruct theta=x+v*c18(x); evaluate all eight corrections, root product and original theta/q on reconstructed selectors. Exhaust full tiny zero-correction and product-root sets, including repeated roots; then profile the actual changed complete graph before any newly frozen hard diagnostic.
