# Matched S4 specialization experiment: prepared, not run

Date: 2026-11-24 (America/Los_Angeles). Draft crypto PR #5165. Proposal #24 in the [100-item ledger](IDEAS.md) concerns sparse resultant splitting for elliptic curve point relations over `F_p`. This report and the [native PARI/GP source](S4_MATCHED_GP.gp) prepare the frozen comparison in [workstream 1–34](WORKSTREAM_001_034.md). The planned raw output path is `research/prime_field_100_ideas_20261123/S4_MATCHED_RAW.txt`; it will be created only by an admitted run. No outcome is claimed here.

## Source and hypothesis

[Semaev, *Summation Polynomials and the Discrete Logarithm Problem on Elliptic Curves*](https://eprint.iacr.org/2004/031.pdf), Theorem 1, gives the `S3` polynomial and `S4` resultant recurrence. The prior [exact support diagnostic](S4_SUPPORT_GP.txt) found 88/125 nonzero coefficients for target abscissa `z=0` on `E:y²=x³+x+14` over `F_1009`, compared with 125/125 for ordinary specializations. The question is whether that reduction changes exact point-decomposition work or independent relation rank when both targets are on the same prime-order curve. The prior check did not measure either quantity. `[pcc:unverified]`

## Frozen inputs and controls

| Field | Frozen value |
|---|---|
| Curve | `p=1009`, `a=1`, `b=14`; script guards `#E=1013` prime |
| Targets | Valid abscissae `z=0` and `z=320`; each target uses the deterministic GP square root as its affine point |
| Factor base | First 16 nonzero liftable abscissae in ascending order, identical for both targets; deterministic GP square-root point choice |
| Search | All 16³ ordered triples per target; all 8 sign patterns per triple |
| Algebraic arm | Exact `Res_T(S3(X,Y,T),S3(T,W,z))` over `F_1009`; count every grid zero |
| Reference arm | Direct signed elliptic-curve addition to the chosen affine target; emit every signed witness and reject any mismatch with the algebraic zero set |
| Relation matrix | One exact row `[base coefficients | −1 target]` per signed witness; rank over `F_1013`, reported separately for the two targets |
| Work proxy | Pair-specialized univariate coefficient counts and Horner multiplication/addition counts for 16 candidate `W` values per pair; symbolic resultant and specialization costs excluded; signed group additions reported separately |
| Cap | Two targets, 4096 ordered triples each, no randomness, no external target, no cloud spend |

The matrix rank is a property of each target's bounded relation set. It is not a complete collection or target-descent result. The Horner counts are deterministic arithmetic proxies, not wall time or a full-cost comparison. A valid finding requires the script's exact algebraic-versus-group agreement and no GP error line. Any mismatch stops the run and is retained.

## Launch and admission

The previous `scripts/checkpoint.sh gate` failed because both configured S3 credential IDs were rejected (`InvalidAccessKeyId`), as recorded in [workstream 1–34](WORKSTREAM_001_034.md). The script must remain unrun until a repaired checkpoint gate passes and the parent agent confirms admission. The gate and experiment are sequential:

```sh
scripts/checkpoint.sh gate
gp -q < research/prime_field_100_ideas_20261123/S4_MATCHED_GP.gp > research/prime_field_100_ideas_20261123/S4_MATCHED_RAW.txt 2>&1
```

After an admitted run, record the actual GP version, source and raw-output SHA-256, exit code, all `target`, `witness`, and `summary` lines, any error, and the checkpoint location here. Independently check a sample of signed rows and the rank before promoting a result; preserve failed and zero-yield rows. No command in this section was executed while preparing this report.

## Admission amendment — 2026-11-24 (America/Los_Angeles)

After being told that the required S3 checkpoint gate returned HTTP 403 with `InvalidAccessKeyId`, the user explicitly authorized: **“Run locally with bypass.”** This authorization applies to the bounded public EC comparison in this report. The pre-run gate remains failed and is not presented as satisfied. The earlier launch restriction in this report is superseded for this one local run only; no cloud target or external workload is added. No input, target, factor base, stop condition, or comparison rule is changed.

Frozen source: `S4_MATCHED_GP.gp`, SHA-256 `91219c532a79ef98c5b28cff080b27b7b4fe2b58a2efd03add6255b6e028110b`. PARI/GP 2.17.3 on arm64 macOS. Exact one-shot command from repository root:

```sh
gp -q < research/prime_field_100_ideas_20261123/S4_MATCHED_GP.gp > research/prime_field_100_ideas_20261123/S4_MATCHED_RAW.txt 2> research/prime_field_100_ideas_20261123/S4_MATCHED_STDERR.txt
```

Record its shell exit status in `S4_MATCHED_EXIT.txt` immediately afterward. Preserve both streams even on parse error. Stop after this one attempt; no silent retry or protocol rewrite. The frozen cap remains two on-curve targets (`z=0,320`) and 16³ ordered triples × 8 signs per target, with exact algebraic/group agreement and modular rank reported separately from the Horner work proxy. All results are producer observations `[pcc:unverified]` pending independent review.

## Admitted attempt outcome — 2026-11-24 (America/Los_Angeles)

The command above was invoked **once**, locally, under the explicit user override. [Stdout](S4_MATCHED_RAW.txt), [stderr](S4_MATCHED_STDERR.txt), and the [shell exit code](S4_MATCHED_EXIT.txt) are preserved separately. The shell reported exit code 0, but GP emitted 28 error lines on stderr. The first errors were `syntax error, unexpected end of file` at the multiline top-level `while(nfb<B&&xx<p,` and nested `if(issquare(q),` statements. The factor-base guard then failed; stdout shows `factor_base_x=[1,0,0,...]`. Subsequent `elladd` calls failed on integer placeholders. GP continued processing later top-level expressions and printed `complete matched exact comparison`, but that sentinel is **invalid** because the preceding errors and guards failed. `[pcc:unverified]`

| Artifact | SHA-256 |
|---|---|
| Frozen source `S4_MATCHED_GP.gp` | `91219c532a79ef98c5b28cff080b27b7b4fe2b58a2efd03add6255b6e028110b` |
| Stdout `S4_MATCHED_RAW.txt` | `f82749d00111a3a8abbfea76b6c2929e05c238be671e7bd28eb19289f1bf86d0` |
| Stderr `S4_MATCHED_STDERR.txt` | `b79410caa7d03e9f83754d78ae0d9c99a4dac9e475f63aaca76fc1d8bb20e68b` |
| Exit record `S4_MATCHED_EXIT.txt` | `bde294368bfed77c2cddf8cec271d398aee9cdbab3b26e1059281bd33adb0120` |

The stdout support counts (`88/125` at `z=0`, `125/125` at `z=320`, multidegree `(4,4,4)`) reproduce the already recorded structural diagnostic; they were computed independently of the factor-base loop that failed to parse. They are **not** admitted as a matched decomposition outcome. There are zero emitted witness rows and zero `summary` lines. Exact signed-point agreement, rank, and arithmetic cost proxy are unavailable. The frozen run is stopped. A corrected script needs a separately frozen amendment and explicit new admission; this attempt and its errors must remain intact.

## Second-attempt amendment — 2026-11-24 (America/Los_Angeles)

After inspecting the preserved V1 stderr and source, the parent agent explicitly authorized one **same-input parser repair** and one new local attempt under the user's previously stated checkpoint bypass. V1 source and its stdout, stderr, exit record, and failure analysis remain unchanged. [V2 source](S4_MATCHED_GP_V2.gp) wraps only the factor-base construction loop in a GP function; its diff against V1 changes no curve, target, factor-base selection rule, cap, relation check, rank computation, or work proxy. GP 2.17.3 is unchanged. V2 source SHA-256 is `2c6c948e86b2225e4c341117582ff984bd4e74ee52c8cec9d3cfe4d5cb627657`.

The single authorized V2 invocation from repository root is:

```sh
gp -q < research/prime_field_100_ideas_20261123/S4_MATCHED_GP_V2.gp > research/prime_field_100_ideas_20261123/S4_MATCHED_V2_STDOUT.txt 2> research/prime_field_100_ideas_20261123/S4_MATCHED_V2_STDERR.txt
```

Record the shell status immediately in `S4_MATCHED_V2_EXIT.txt`. Stop on any GP stderr error, factor-base guard failure, or algebraic/group mismatch. Preserve this V2 attempt even if it fails; no further retry or source edit is admitted by this amendment. The same public curve `E/F_1009: y²=x³+x+14`, target abscissae `0,320`, first 16 nonzero liftable abscissae, 4096 ordered triples per target, and 8 signs per triple remain frozen. The bypass is explicit user authorization, not a successful S3 gate.

## V2 attempt outcome — 2026-11-24 (America/Los_Angeles)

The V2 command was invoked **once**. Shell exit was 0, [stderr](S4_MATCHED_V2_STDERR.txt) was empty, and [stdout](S4_MATCHED_V2_STDOUT.txt) contains 80 lines: the complete 16-point factor base, both target points, every signed witness row, both `summary` lines, and the final sentinel. The [exit record](S4_MATCHED_V2_EXIT.txt) was written immediately. The script's exact equality check between `S4=0` on each factor-base triple and existence of a signed point sum to the chosen affine target passed across all 4096 triples per target; no `mismatch` line was emitted. This is a producer self-check `[pcc:unverified]`, pending independent validation.

| Quantity | Special `z=0` | Ordinary on-curve `z=320` |
|---|---:|---:|
| `S4` nonzero coefficients / 125 | 88 | 125 |
| Box corners / 8 | 4 | 8 |
| Multidegree `(X,Y,W)` | `(4,4,4)` | `(4,4,4)` |
| Exact polynomial evaluations | 4096 | 4096 |
| Algebraic-zero triples | 24 | 33 |
| Signed-point triples | 24 | 33 |
| Emitted signed witness rows | 24 | 33 |
| Modular row rank / 17 columns | 4 | 6 |
| Pair-specialized quartic nonzero coefficient sum | 1239 | 1276 |
| Horner multiplication proxy | 16320 | 16352 |
| Horner addition proxy | 15728 | 16320 |
| Signed group additions in reference | 65536 | 65536 |

The exact factor base is `x=[1,2,4,5,10,11,13,16,17,23,25,29,30,34,37,40]`; stdout gives each chosen `y` and target `y`. The 88/125 versus 125/125 support difference corresponds to 29.6% fewer symbolic nonzero coefficients. After pair specialization, the counted Horner proxy differs by 32/16352, or about 0.20%, fewer multiplications and 592/16320, or about 3.63%, fewer additions at `z=0`. These proxies exclude constructing and specializing the resultant, memory traffic, and any complete relation collection. The two target identities also have different exact witness counts and ranks; those differences cannot be assigned to coefficient sparsity from this two-target panel. `[pcc:unverified]`

| Preserved V2 artifact | SHA-256 |
|---|---|
| `S4_MATCHED_GP_V2.gp` | `2c6c948e86b2225e4c341117582ff984bd4e74ee52c8cec9d3cfe4d5cb627657` |
| `S4_MATCHED_V2_STDOUT.txt` | `3accec218c07a2acd2fb433270fa8724db6aee57fef85d0e29e3eb3de16c1f7f` |
| `S4_MATCHED_V2_STDERR.txt` (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `S4_MATCHED_V2_EXIT.txt` | `bde294368bfed77c2cddf8cec271d398aee9cdbab3b26e1059281bd33adb0120` |

V1's parse failure and V2's completed component result are separate attempts. Neither is silently dropped. This V2 panel satisfies its exact local self-check but does not measure resultant construction work or a complete index-calculus solver. A subsequent comparison would require an independently reviewed verifier and a frozen method that isolates target specialization from target-dependent relation density.
