# C141: implicit projected domains

Hypothesis: remove the domain-materialization bottleneck introduced by exact field-line elimination. A prime-interval projection has exact logarithmic-step membership via two Euclidean floor sums; overlapping translated intervals have at most two cyclic segments. An affine extension-field subspace projection is represented by an exact base-field Gaussian basis. This changes domain representation without dropping reconstructed original-domain constraints. Independent coordinate membership does not establish a shared pivot witness across coordinates.

Actual bounded evidence:

{
  "out": {
    "floor_sum_oracles": 20250,
    "prime_witness_oracles": 44401,
    "prime_member_oracles": 6739,
    "prime_root_oracles": 1045,
    "contexts": [
      {
        "name": "GF2",
        "projection_cases": 32,
        "complete_direct_cases": 32,
        "seconds": 0.0003446000046096742
      },
      {
        "name": "GF4",
        "projection_cases": 144,
        "complete_direct_cases": 144,
        "seconds": 0.0024674749874975532
      },
      {
        "name": "GF8",
        "projection_cases": 256,
        "complete_direct_cases": 256,
        "seconds": 0.009060537006007507
      },
      {
        "name": "GF2^83",
        "projection_cases": 144,
        "complete_direct_cases": 0,
        "seconds": 0.044645689020399004
      },
      {
        "name": "3^1",
        "projection_cases": 48,
        "complete_direct_cases": 48,
        "seconds": 0.0006718940276186913
      },
      {
        "name": "5^1",
        "projection_cases": 48,
        "complete_direct_cases": 48,
        "seconds": 0.001353483006823808
      },
      {
        "name": "3^2",
        "projection_cases": 144,
        "complete_direct_cases": 144,
        "seconds": 0.016018017020542175
      },
      {
        "name": "5^2",
        "projection_cases": 144,
        "complete_direct_cases": 144,
        "seconds": 0.05514895200030878
      },
      {
        "name": "3^3",
        "projection_cases": 256,
        "complete_direct_cases": 256,
        "seconds": 0.1600766810006462
      },
      {
        "name": "7^1",
        "projection_cases": 48,
        "complete_direct_cases": 48,
        "seconds": 0.00120506901293993
      },
      {
        "name": "11^1",
        "projection_cases": 48,
        "complete_direct_cases": 48,
        "seconds": 0.0020245460036676377
      },
      {
        "name": "7^2",
        "projection_cases": 144,
        "complete_direct_cases": 144,
        "seconds": 0.15346268800203688
      },
      {
        "name": "P-192",
        "projection_cases": 48,
        "complete_direct_cases": 0,
        "seconds": 0.0020371849823277444
      },
      {
        "name": "P-224",
        "projection_cases": 48,
        "complete_direct_cases": 0,
        "seconds": 0.0019909850088879466
      },
      {
        "name": "P-256",
        "projection_cases": 48,
        "complete_direct_cases": 0,
        "seconds": 0.002035852987319231
      },
      {
        "name": "P-384",
        "projection_cases": 48,
        "complete_direct_cases": 0,
        "seconds": 0.00211251899600029
      },
      {
        "name": "P-521",
        "projection_cases": 48,
        "complete_direct_cases": 0,
        "seconds": 0.0022731310164090246
      }
    ],
    "nist": [
      {
        "name": "P-192",
        "source_bits": 64,
        "segment_count": 2,
        "projection_cardinality": 36893488147419103231,
        "arbitrary_ratio_mode": "membership_only",
        "max_floor_steps": 80,
        "query_seconds": 0.0008542690193280578,
        "large_root_enumeration": false
      },
      {
        "name": "P-224",
        "source_bits": 74,
        "segment_count": 2,
        "projection_cardinality": 37778931862957161709567,
        "arbitrary_ratio_mode": "membership_only",
        "max_floor_steps": 92,
        "query_seconds": 0.0009633340232539922,
        "large_root_enumeration": false
      },
      {
        "name": "P-256",
        "source_bits": 85,
        "segment_count": 2,
        "projection_cardinality": 77371252455336267181195263,
        "arbitrary_ratio_mode": "membership_only",
        "max_floor_steps": 99,
        "query_seconds": 0.000988421990768984,
        "large_root_enumeration": false
      },
      {
        "name": "P-384",
        "source_bits": 128,
        "segment_count": 2,
        "projection_cardinality": 680564733841876926926749214863536422911,
        "arbitrary_ratio_mode": "membership_only",
        "max_floor_steps": 159,
        "query_seconds": 0.0019433429988566786,
        "large_root_enumeration": false
      },
      {
        "name": "P-521",
        "source_bits": 173,
        "segment_count": 2,
        "projection_cardinality": 23945242826029513411849172299223580994042798784118783,
        "arbitrary_ratio_mode": "membership_only",
        "max_floor_steps": 226,
        "query_seconds": 0.002817453001625836,
        "large_root_enumeration": false
      }
    ],
    "binary83": {
      "same_ratio_rank": 18,
      "shift_ratio_rank": 36,
      "cardinalities": [
        262144,
        68719476736
      ],
      "large_enumeration": false
    },
    "refusals": 7,
    "all_registered_contexts_complete": true,
    "original_complete": false,
    "total_seconds": 0.6144437549810391
  },
  "error": null,
  "supervisor": {
    "status": "ok",
    "wall_seconds": 0.6689498969935812,
    "peak_process_rss_kib": 846224
  }
}

The signed floor sums are independently compared with direct summation. All interval pairs/ratios over primes 2,3,5,7 are checked against direct Cartesian products; admitted small root tuples are independently enumerated. All 17 registered coefficient-field contexts receive projection controls, with exhaustive direct products in small fields and exact full/point/basis controls in large fields. GF(2^83) uses the actual prefix18 domain. The five NIST primes use widths 2^64,2^74,2^85,2^128,2^173. These are full-width domain-query controls, not complete hard curve solves. Oversized enumeration returns incomplete with roots/count null; membership speed does not solve candidate indexing or all roots.

Output contract: for a nonzero independently certified field-line slope kappa and v_p!=0, every original root theta maps uniquely to gauge x=theta-v*theta_p/v_p, and theta=x-v*c(x)/kappa. Projected domains contain every such gauge. Reconstructing and rechecking all original domains and constraints makes the bounded enumerator sound and complete over admitted products. Refusal is never an empty-root certificate. The proof and frozen sources cover empty domains and zero-dimensional subspaces; zero slopes require a separate branch.

One new 90s/4GiB/CPU0 component ran once after synchronous manifest validation, normal root-ledger resume and run-ready. Frozen source/input hashes, prior attempts, rejected C140 plans and the separate eight-record C140 child ledger remain preserved. Timing and RSS are full-child observations; different parent-ledger/fresh-ledger launcher contexts prevent attributing RSS differences to algorithms. A dedicated launcher calibration is queued.

Primary-source review: AtCoder Library internal_math.hpp at ffb7aa847fe142c1eb346c5083194d7b8f53c029 implements the same Euclidean floor-sum recurrence with fixed-width unsigned contracts. This implementation uses Python exact integers and signed normalization plus domain-specific count differences. Euclidean floor sums, linear projection and Gaussian bases are established. Novelty of a complete generalized rootfinding algorithm remains unestablished.

Next: C142-shared-pivot-implicit-domain-integration. Integrate these descriptors into a shared-pivot constraint or derive exact implicit candidate indexing, then measure total reconstruction/domain/search costs against the enumerating wrapper on matching complete small original root sets. Large membership-only projections must not be silently enumerated. For the binary hard diagnostic, derive shifted simultaneous tangents and singularity-safe dynamic block elimination rather than repeat the unchanged UNKNOWN controller. Preserve original source/U/ghost/outer branches and all requested fields/curves.

The 20 hard-instance genuine matched F4/F5/F6 gate remains active and unpassed. No qualified 2x,8x or50x end-to-end speedup or memory gain is established.
