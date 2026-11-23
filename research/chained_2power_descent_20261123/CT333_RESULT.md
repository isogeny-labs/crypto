# CT333 — all Frobenius types: rational Richelot-kernel sieve

**Date:** 2026-11-23 UTC. **Standing:** bounded producer-authored source-only finite enumeration, **[pcc:unverified]**. This is an additive continuation of [CT331](CT331_IRREDUCIBLE_TORSION_GATE.md) and [CT332](CT332_RESULT.md), not a native/genus-three solver experiment.

## Executed result

There are exactly 15 maximal isotropic Richelot kernels over an algebraic closure of odd characteristic, represented by unordered perfect matchings on the six Weierstrass points. For each of the 720 permutations in S6, the new Python classifier computed all 15 kernel orbits. A separately coded 4-dimensional F2 symplectic-isotropic-plane representation matched **every** fixed kernel and its full orbit multiset; eleven conjugacy-class populations also matched the elementary S6 class-size formula. All three malformed negative controls rejected. `python3` and `python3 -O` emitted byte-identical records. **These are producer self-checks, not independent or PCC-signed proof.** [pcc:unverified]

| Frobenius cycle type | Number of k-rational maximal isotropic subgroups | Kernels with all 3 nonzero pair classes k-rational | Minimum extension degree admitting a kernel |
|---|---:|---:|---:|
| 1+1+1+1+1+1 | 15 | 15 | 1 |
| 2+1+1+1+1 | 3 | 3 | 1 |
| 2+2+1+1 | 3 | 1 | 1 |
| 3+1+1+1 | **0** | 0 | **3** |
| 2+2+2 | 7 | 1 | 1 |
| 3+2+1 | **0** | 0 | **3** |
| 4+1+1 | 1 | 0 | 1 |
| 3+3 | 3 | 0 | 1 |
| 4+2 | 1 | 0 | 1 |
| 5+1 | **0** | 0 | **5** |
| 6 | 1 | 0 | 1 |

The additional **3+2+1** classification gives no rational one-step Richelot kernel even though the mod-two module is reducible; it joins the previously noted 3+1+1+1 and 5+1 classes. The **3+3** class illustrates the opposite subtlety: three rational *subgroups* but no rational perfect matching whose three individual pair classes are all fixed. A Frobenius-permuted triple is still a rational kernel.

The extension degree is the minimum degree over which at least **one** of the fifteen geometric maximal isotropic subgroup schemes becomes Frobenius stable; it is **not** a computed cost of constructing the isogeny and **not** the extension degree needed for an entire multi-hop chain. These finite combinatorial counts use standard Richelot matching correspondence and are not claimed as novel literature.

### Exact custody and commands

- Preregistered [CT333_PROTOCOL.md](CT333_PROTOCOL.md) in Git commit `fa520d6c4fddfa17f9748acf706c0c773f51b5cf` **before** final source-bound execution. An earlier exploratory scratch census was excluded.
- Python script [ct333_rational_kernel_sieve.py](ct333_rational_kernel_sieve.py): exact blob `744efe7bff8cfa8a524fe5189d8dc6a7dc21b83f`, SHA-256 `611555114ff1c2db5a1f90a94a5fcbc61cfa93a0a06e5154d1ac1ecf618278cc`, source commit `2937e71fce2039ed1573eeef09fb0fc6272d2526`; local `git hash-object` confirmed exact Git blob.
- Full [CT333_RESULT.json](CT333_RESULT.json) exact Git blob `f627174d022de627777680c251597f196cbfd8ca`, SHA-256 `e82b9e7bc9197918eb410d6266f86f087ccbde4c55b359f435aa030f5ea320ce`; Python normal/-O output byte-identical and local Git blob equals remote readback. No timed solver run. [pcc:unverified]

```sh
cd research/chained_2power_descent_20261123
python3 ct333_rational_kernel_sieve.py > CT333_RESULT.json
python3 -O ct333_rational_kernel_sieve.py > /tmp/ct333_opt.json
cmp CT333_RESULT.json /tmp/ct333_opt.json
sha256sum ct333_rational_kernel_sieve.py CT333_RESULT.json
```

The script must run at the stated **original source blob**. The first two output hashes above specify exact replay identities; the `/tmp` derivative is not a new frozen source. The inspected one-page [PDF report](CT333_FROBENIUS_KERNEL_GATE.pdf) and [SVG explanatory flow diagram](CT333_KERNEL_SIEVE_FLOW.svg) are now in the PR at commit `246889ebd3b022a5f9247ebe857e7275f163cc52` (PDF Git blob `65a6d1f1a94a9b2df929577ba35448a5bcb2fb3f`, SHA-256 `e5a1efb8510fcb67357408314944bd89a888e5c70da025dce0f7b9d92eafe857`); PDF was rasterized and visually inspected after a layout correction. [pcc:unverified]

## The mathematical distinction that matters

Magma's [Richelot documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1615) identifies these fifteen subgroups with partitions of six branch points into three pairs; their kernels are Galois-stable precisely when the *unordered triple* of pairs is Frobenius-stable. Thus a zero in the rational-kernel column rules out **one base-field-defined (2,2) isogeny with that source kernel**, but not every multi-hop descent of *nonrational* steps. Parent [#4992](https://github.com/isogeny-labs/crypto/pull/4992) has an explicit 3+1+1+1 mixed-kernel counterexample for descended double steps; do not promote this sieve to an all-length obstruction.

Conversely a nonzero entry is a **candidate rational kernel**, not a guarantee that an ordinary Richelot formula produces a smooth Jacobian codomain rather than a decomposable PPAV, nor a prime-field DLP transfer with lower full cost. CT331's all-length irreducible-2-torsion gate remains specific to 5+1 and properly requires descent of the *composite morphism*, not an equation for the target alone.

## Ownership, exclusions and next decision

One coordinator performed the protocol, Python source, local tests, raw-record replay, readback, report and visuals. **Zero native child workers** or independently authenticated validators participated. No native Rust/Cantor build, Lean proof, EKS/Spot job, paid spend, same-host timing, core 695/701 fixture acquisition, PCC signature, merge or main-branch mutation occurred. No main, parent #4990/#4992, genus-three RREF or norm-point reuse owner paths were modified. Research KB and Conductor were unavailable; fallback repo notes and Magma, Castryck–Decru–Smith (2020), Bruin–Doerksen (2009) and the November 2026 Dang abstracts were reviewed. [pcc:unverified]

Use the zero-kernel types as *one-step* fast rejects and the minimum-degree column to bound the splitting-field overhead of candidate rational-quotient models. Next substantive research must attach **genuine curves and descended maps**, then price full transfer, Cantor arithmetic, relation processing, linear algebra, recovery and memory against the strongest matched optimized rho/BSGS/IC on approved EKS. Authenticated 695/701-relation custody, C169/C13a/C13b/N2_actual/D physical pricing and independent PCC remain with their existing owner. This record changes a structural screening table, **not** a complete ECDLP speedup claim.
