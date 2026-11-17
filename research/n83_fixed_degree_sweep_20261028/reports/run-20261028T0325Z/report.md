# N83 index-calculus degree sweep

Observations only; this is a bounded decomposition-system run, not an evidence-review verdict or a full ECDLP result.

All 96 computations were attempted. 4 completed; 0 supplied a finite top-form degree; 63 reached the process time limit and 29 exited with Singular's no-more-memory code 14. Any memory diagnostic is retained separately; classifications follow process exit status.

Fixed ell=27 for every m; per-curve factor-base manifests are unchanged. Candidate x-domain size is 2^ell; actual points must lift to the curve. Every cell has one planted and one natural target, with seed 20261027. Values below are observed exact degrees only when the solver completed. P/N means planted/natural.

| m | ell | Koblitz top P/N | Binary top P/N | Prime top P/N |
|---:|---:|---|---|---|
| 2 | 27 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 3 | 27 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 4 | 27 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 5 | 27 | time limit / memory limit | time limit / memory limit | time limit / memory limit |
| 6 | 27 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 7 | 27 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 8 | 27 | memory limit / time limit | memory limit / time limit | time limit / memory limit |
| 9 | 27 | memory limit / time limit | memory limit / memory limit | time limit / memory limit |

## Affine output

The following values are maximum degrees in a completed final basis. They are not maximum intermediate solving degrees.

| m | Koblitz P/N | Binary P/N | Prime P/N |
|---:|---|---|---|
| 2 | 2 / 0 | 2 / 0 | time limit / memory limit |
| 3 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 4 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 5 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 6 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 7 | time limit / time limit | time limit / time limit | time limit / memory limit |
| 8 | memory limit / time limit | memory limit / time limit | time limit / memory limit |
| 9 | memory limit / memory limit | memory limit / memory limit | time limit / memory limit |

## Reproduction and scope

Frozen design: ../../PROTOCOL.md and ../../protocol.json. Image, argv and limits: ../../request.json. Terminal execution/image identity: ../../terminal.json. Unified export: ../../experiment.export.json. All raw paths and verified content hashes: ../../evidence/storage.json and ../../evidence/raw-manifest.sha256. Eight same-driver controls completed. Five native arithmetic/construction/membership tests passed before freezing. No independent replay certificate is claimed.

The measured quantity is one plus the last nonzero graded degree of the quotient by the highest homogeneous components, where that quotient is zero-dimensional. It is representation dependent and is not the regularity of the saturated homogenized affine ideal. Positive dimension is reported explicitly. No timeout supplies a degree bound. Intermediate solving degree remains unmeasured. CPU user/system time and peak RSS were not recorded; wall times in the raw rows are diagnostic and invalid for performance claims.

Koblitz uses y^2+xy=x^3+1 over GF(2^83), modulus z^83+z^45+z^2+z+1. General binary uses a=1, b=2^82+0x51b. Prime uses y^2=x^3+2x+3 and the Proth-certified prime in ../../evidence/prime-certificate.json. Binary coordinates use a polynomial-basis subspace; prime coordinates use a bit-encoded integer interval. Full-curve synthetic targets were generated; no subgroup attack is established, including for Koblitz. Roots of an S3 chain require point lifting and sign consistency before they become IC relations.

Methods follow [Kosters and Yeo](https://arxiv.org/html/1503.08001) and the [Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf). The backend is Singular 4.3.1 std, degree reverse lexicographic order. EKS shared-bucket permission failure from the prior enumeration required the existing-host fallback. The Singular runtime image is pinned; the new native binary is mounted read-only. This run preserves every requested n83 cell. Unresolved degrees remain outstanding research work.
