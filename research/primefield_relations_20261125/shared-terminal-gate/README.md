# Shared-terminal membership and public-label correctness gate

**Attempt 001 passed: two Rust regression tests and 47 assertion entries covering 31 distinct checks.** The executable performs a deterministic replay of an archived M31 relation plus exact finite support checks. It makes no new relation-discovery, yield, speed, scaling, LFD, or exponent claim.

All new files are confined to this directory. No shared checkout or GitHub state was modified. The native arithmetic and factor-base constructor are copied from immutable source; no live, concurrently edited source is compiled.

## What the result establishes

On the canonical curve

\[
E/\mathbb F_p:\ y^2=x^3+847514846x+2087444724,
\quad p=2147483647,\quad r=2147446831,
\]

with `G=(625037715,2100627251)` and public input `Q=(1477799919,824532935)`, the program checks that p and r are prime, the curve is nonsingular, and the finite nonzero points G and Q are annihilated by r. A separate integer Hasse bound establishes `#E(F_p)=r`.

For the shared chain

\[
u_0=x,\qquad u_{j+1}=u_j^2-2,\qquad j=0,1,2,
\]

the terminal polynomial is

\[
D_8(x)=x^8-8x^6+20x^4-16x^2+2.
\]

The fixed terminal set is

```
2, 2207, 1166356352, 1519334736, 1780636042, 2147483645
```

Three values are derived from the already-archived relation points, one is the pinned native Base constructor's depth-3 fiber, and two are the exceptional values ±2. The input also repeats the Base terminal to test duplicate handling. These choices are **conditioned on the archived relation**: this is a positive correctness fixture, not an unbiased relation-yield trial.

Let

\[
F(X)=\prod_{c\in C}(D_8(X)-c),
\qquad S=\bigcup_{c\in C}\{x\in\mathbb F_p:D_8(x)=c\}.
\]

Repeated exact square roots enumerate S. A separate polynomial calculation then verifies [pcc:unverified]

\[
\operatorname{monic}\gcd(F(X),X^p-X)
=\prod_{x\in S}(X-x).
\]

The program obtains the gcd by polynomial modular exponentiation and Euclidean division; it does not enumerate p field elements. Since `X^p-X` has each field element as a simple root, this equality certifies the **entire F_p support of this fixed terminal union**, including exceptional fibers and multiplicities. The complete coefficients and roots are retained in raw JSON for replay by another implementation.

| Set or polynomial | Observed result |
|---|---:|
| Distinct terminals | 6 |
| Degree of F | 48 |
| Distinct F_p selector roots; degree of the gcd | 29 |
| Roots with a rational curve lift | 17 |
| Signed rational curve points above those roots | 34 |
| Selector roots without a curve lift | 12 |
| Frozen original canonical columns carrying checked public tags | 3 |
| Rational abscissae outside that frozen three-column roster | 14 |

“Outside the roster” is a concrete identity/metadata condition in this gate. It is not a proof that those points cannot be labeled by some other construction. Membership in a selector fiber is insufficient to insert a point into the frozen labeled relation matrix.

## Archived useful relation replay

The [historical receipt](https://github.com/isogeny-labs/crypto/blob/b60f801660eb423901ddea0c03c629089a1b5bfc/research/prime31_ic_20261115/native/APS_ONE_RELATION_PRIME31_RUST_CI_20261116.json) references [Actions run 43786504359](https://github.com/isogeny-labs/crypto/actions/runs/43786504359) at source commit `90a0314f5be63d5ca56465a884091f4be0f114fd`. Its exact point-source function reconstructs three lists of 1536 points using the historical seed `0x4150535f50523331`, with zero identity or duplicate-x rejections. This gate performs **zero relation-search or pair-table calls** and takes the already-published relative indices `[437,1247,879]` and signs `[-1,+1,-1]` directly from that evidence.

The historical seed is inherited replay provenance. It is not a newly preregistered random seed. Q remains the public input to all source generation; the receipt's known output scalar is used only as an expected final result.

Each record is canonicalized by choosing `min(y,p-y)` and negating both scalar labels modulo r whenever the point is negated. The three resulting records are:

| Original column ID | Canonical point | Public a | Public b | Canonical relation sign |
|---|---|---:|---:|---:|
| 437 | `(2094212670,136182498)` | 859702901 | 1311605331 | +1 |
| 2783 | `(52167908,1024913436)` | 1062506079 | 696601830 | −1 |
| 3951 | `(90062827,494460869)` | 1347373701 | 1396869982 | +1 |

Every point is checked as `[a]G+[b]Q`. The signed point sum is the identity, and summing the same signed labels gives

\[
\alpha=1144570523,\qquad\beta=2011873483\ne0.
\]

The executable derives `d=-alpha/beta mod r=1038579171` and checks `[d]G=Q`. This reproduces the existing receipt; it is not a newly found useful relation.

## Separate complete labeled fiber on E

The degree-two curve map

\[
h_A(T)=x(T-H),\qquad H=[2^{-1}\bmod r]A
\]

has the two-point fiber `{R,A-R}` away from its fixed point and pole. Taking A and R to be the first two canonical archived points provides a deterministic control. Both H and A−R receive public tags by scalar multiplication and subtraction of the original tags.

The gate reconstructs both lifts above `h_A(R)` and translates them by H, obtaining exactly `{R,A-R}`. For this fixture:

```
H                   = (1561903295,1163870545)
h_A(R)              = 1098002574
A-R                 = (90062827,1653022778)
public tag of H     = (1503574866,1729526081)
public tag of A-R   = (1944643653,615003501)
```

Because A and R were selected from the archived row, A−R geometrically equals the negative of the third canonical point. Its cheaply derived tag differs from the archived point's original tag. The program preserves that distinction. The constructed row `R+(A-R)-A` using the derived tags has `alpha=beta=0` and is rejected as useless. Its point identity alone supplies no new target equation.

This is a map **on E**, using translated points. It is separate from an original-x univariate Dickson selector. The control demonstrates complete public labeling for one curve-map fiber and explicitly checks the associated trivial-relation hazard.

## Rejection and regression gates

The frozen gate rejects a missing-root certificate, mutated chain edge, noncanonical field value, removed terminal, rational sibling with no original column record, changed public scalar tag, wrong relation sign, wrong canonical point orientation, wrong curve, wrong public Q, invalid sign, swapped original column, and zero aggregate Q coefficient. It also verifies empty and repeated terminal handling. [pcc:unverified]

Seventeen assertions check the individual rational curve lifts; the remaining assertions give 31 distinct check names in total. Both Rust tests passed. The first test independently exercises repeated-root removal by `gcd((X-1)^2,X^p-X)=X-1`; the second reruns the full fixed correctness gate. The test harness's incidental duration is not benchmark evidence.

The first build and run passed without a source revision. No failing attempt was discarded. The wrapper preserves failed attempts if a future run fails and refuses to overwrite an existing attempt directory.

## Reproduction and evidence map

The exact command run from this directory was:

```bash
bash run.sh attempt-001
```

To repeat after attempt 001 already exists, use a new attempt name:

```bash
bash run.sh attempt-002
```

On another Linux workspace, provide an absolute compiler path:

```bash
RUSTC=/absolute/path/to/rustc bash run.sh attempt-002
```

The publication wrapper defaults to `rustc` on PATH and resolves a supplied
`RUSTC` name before hashing its executable. Prefer `RUSTC="$(rustup which rustc)"`
when using rustup so the hash identifies the selected compiler rather than its
proxy. Compiler-probe failures after the attempt starts are captured by the
completion trap. These wrapper changes postdate attempt 001; its frozen wrapper
and all executed research source remain unchanged. The wrapper requires Linux,
Bash, GNU `timeout`, `sha256sum`, `jq`, and Rust.

The original attempt used Rust 1.100.0 (`0ef1bc386`, 2026-11-09) from:

```
/workspace/scratch/998688543c6d/toolchain/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
```

It first copies source and this gate's `QUALIFIER.md`, records SHA-256 hashes, and makes those copies read-only. It writes a build manifest before compilation and a separate pre-execution manifest before either test or gate call. The binaries are compiled from those frozen copies with `--edition=2021 -O -C codegen-units=1`; tests add `--test` and run with `--test-threads=1`. Each compiler, test, and gate call has a 45-second limit with a 2-second kill grace. Child processes inherit a 2-GiB virtual-memory cap. These are resource guards, not performance measurements.

| Artifact | Purpose |
|---|---|
| `src/main.rs` | Shared-terminal support, public-tag, relation, mutation, and reflection gates |
| `src/reused_native.rs` | Pinned native arithmetic, curve fixture and Base subset |
| `src/archive_source.rs` | Exact historical PRNG/source replay subset |
| `QUALIFIER.md` | Input selection, scope, expected gates, source pins and resource limits frozen before execution |
| `results/attempt-001/frozen/` | Immutable source/qualifier snapshot actually compiled |
| `results/attempt-001/frozen/source-manifest.sha256` | Source/qualifier SHA-256 inventory |
| `results/attempt-001/build-manifest.json` | Pre-build manifest |
| `results/attempt-001/pre-execution-manifest.json` | Distinct pre-test/pre-gate manifest |
| `results/attempt-001/post-execution-manifest.json` | Completion/exit status; wrapper exit 0 |
| `results/attempt-001/result.json` | Raw coefficients, support, points, tags, relation and assertion results |
| `results/attempt-001/tests.stdout` | Two passing Rust regression tests |
| `results/attempt-001/compile-*.stdout`, `compile-*.stderr`, `tests.stderr`, `gate.stderr` | Raw compiler and executable streams; all stderr streams empty |
| `results/attempt-001/binaries.sha256` | Binary hashes |
| `results/attempt-001/retained-output-hashes.sha256` | Manifest and raw output hashes |

Raw result SHA-256:

```
1538f5e862f998d5f3a125db39d2b14182a8740391177d7b47b6a11a140dd2d3
```

The exact copied source bodies were compared before building. Native source lines 20–244 match the immutable blob after removal of `Serialize` derive tokens; historical source lines 71–100 match after changing `source` visibility to `pub`. Full originals, Git blob hashes, commit pins and SHA-256 hashes are preserved in `sources/` and the qualifier.

## Limits

The support result is an exact finite computation on the stated selector and field. Group arithmetic is intentionally reused from the native lab; this is not an independent arithmetic implementation, a proof-kernel artifact, or a signed PCC. The producer standing of the historical relation is retained.

The gate does not supply a new low-cost mechanism for obtaining useful relations, labels for every selector root, a generic-DLP improvement, a relation-yield measurement, or evidence about LFD. Its reusable increment is a strict boundary between terminal membership, rational lifting, original-column/public-tag admission, and a genuinely nonzero target equation.
 [pcc:unverified]