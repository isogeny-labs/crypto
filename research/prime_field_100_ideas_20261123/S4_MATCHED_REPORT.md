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
