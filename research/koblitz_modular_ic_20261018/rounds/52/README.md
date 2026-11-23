# R52: exact nu closure, line norm and larger source field

The complete nu adjunction reduces the source index from 376 to 180 and
stabilizes after three extensions. Its least recovery denominator is
z^5(z+1)^9, with a full 32x32 product and exact local minimality. All local
invariant factors are retained. Their complete type matches a candidate
(7,11) cusp model; identifying the actual completed local rings is next.

The generic degree-3 isogeny now has explicit image-line coordinates
(beta/alpha,gamma/alpha), a full norm identity and all three sign products.
The larger marked-23 source has a degree-112 function field and full F2
constant field by the exact two-fiber factorization argument in CM.md.
Every one of the 3,810 finite specialization certificates and all of their
irreducible factors are checked again in a fresh Singular process.

There are 17 terminal attempts: 12 accepted, 5 rejected. Nine Lean declarations
compile and pass the axiom audit with zero failing or trusted declarations.
Read PROOF_MAP.md for their exact hypotheses and the remaining native/DVR/
geometric inputs. CI and byte custody are separate from scientific replay.

- [Report](report.pdf): three-page evidence summary and native data plots.
- [Mathematical account](CM.md): complete certificates and field argument.
- [Results](results.json) and [receipt checks](receipt-validation.json).
- [Formal scope](PROOF_MAP.md) and [Lean receipts](lean-validation.json).
- [Custody](ARTIFACTS.md): immutable raw object and fresh download/repack.
- [New leads](NEW-LEADS52.md) and [continuation](NEXT.md).

Sources and each affected protocol were committed before mathematical data.
The raw archive retains all failed inputs/logs/partials, full matrix products,
all finite factorizations and the accepted Lean objects. Do not regenerate
accepted producers for publication. Use a fresh attempt name for a deliberate
checker replay after restoring raw data with ARTIFACTS.md.

```bash
./REPLAY_CAS.sh fresh-recheck FRESH52.sing FRESH52_ALL_CHECKS_PASS
```

Lean requires the pinned 4.34.1/mathlib d13f23b cache listed in the retained
R51 proof-cache-roots file. FibreIrred52 imports the accepted Closure52.olean;
restore that exact object from runs/closure-lean52-02/Closure52.olean to
/var/tmp/koblitz-r52-closure-lean52-02/Closure52.olean if its cache is absent.
Check its SHA against lean-validation.json; preserve any existing different
cache and reconcile it before proceeding. This restores a checked dependency,
not a new producer. Every proof source and compiled object is in raw custody.

Unchanged limits: native Rust/Singular only, one job at a time, 4 GiB virtual,
1200 seconds, one Singular/FLINT thread, seed 52. Lean -j1 -M4096, 1200 seconds
separately for compile/audit, no virtual cap. No new EKS, Diem/freezer restart,
Conductor claim, delegation, merge or auto-merge change. Further science is R53.

The two large donor arrays are addressed by source-data/.artifacts.json and
retained inside the raw archive as well. input-custody52.json verifies fresh
downloads against their original pre-data Git bytes; history is preserved.
