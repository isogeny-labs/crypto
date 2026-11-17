# Complete identity-mask XCNF census, primary N83

The [frozen protocol](protocol.json) exported all 192 formulas for the public
`a=0` GF(2^83) Koblitz target: trials 0–2, hash/Gray residual pools,
5+1/4+2 splits, and masks 0–15. The first [raw census](census.json) has
SHA-256 `16161a3293d1b76405ec5ef4fedc0c11c966c16f7b3e9dc1d77b25ae79332a7e`.
The [repeat](census-repeat.json) has SHA-256
`47e4ab5d79da5f721399dbfe3a7b3e5e6ea825e3e688c7bb4607a76a687cf7b8`.
The [replay receipt](replay.json) confirms that all 192 deterministic rows
match exactly, including formula SHA-256, byte count, shape, coefficients and
group target. The 16 formulas in the earlier hosted panel also match their
independently frozen digests. Both runs used the exact Darwin ARM worker
binary `aa2fb100e4da74681c01d1523cb8d173773613eaabe68a897724f51a5d6710ff`
and locally cached archive bytes matching the 11-object S3 manifest. No
relation solver ran in this census.

The first pass charged 441.080 seconds of summed export process wall and
442.009 seconds of panel wall. The repeat charged 434.736 and 435.640 seconds,
respectively. These are L0 local construction measurements from already
fetched S3 inputs. They do not include a fresh S3 GET, hosted build, relation
failures, graph closure, rank, linear algebra, target descent or rho.

Three exact necessary-condition screens narrow the **identity-pattern** slice
for these targets and domains:

| Gate | Formulas excluded | Reason |
| --- | ---: | --- |
| [Circuit structure](analysis.json) | 96 | Adjacent identity bits require `O + affine = O`. |
| [Target x-domain](domain-screen.json) | 36 more | The last summand would have to equal the target x, absent from both exact pool archives. |
| [Exact group pair sums](pair-screen.json) | 24 more | Masks 4 and 5 leave only the final two summands to reach the target; all six trial/pool pair-sum checks found zero matches. |

This leaves masks **0, 1 and 2** in each of the 12 trial/pool/split strata,
or 36 formulas, without a contradiction from these gates. It does not prove
that any of those formulas is satisfiable. The pair screen replayed 99,600
signed points in the base and 99,600 points in each pool, then formed the
exact residual sets (99,600 hash points; 16,102 Gray points). Its independent
Python group arithmetic was pinned by SHA-256 and checked against public
curve points; all target pair-hit counts were zero. The zero-hit result is
specific to trials 0–2, the two stored pools and these splits.

The [exact support screen](support-screen.json) counts at most one distinct
target sum per unordered multiset of the remaining summands. Dividing by the
nonidentity subgroup size bounds the fraction of targets covered by each
pattern. For every trial, the upper bounds are:

| Pool and split | Mask 0 | Mask 1 | Mask 2 |
| --- | ---: | ---: | ---: |
| hash 5+1 | 1 (vacuous) | 6.784×10⁻⁶ | 2.043×10⁻¹⁰ |
| hash 4+2 | 1 (vacuous) | 1.018×10⁻⁵ | 2.043×10⁻¹⁰ |
| Gray 5+1 | 1 (vacuous) | 1.097×10⁻⁶ | 3.303×10⁻¹¹ |
| Gray 4+2 | 1 (vacuous) | 2.660×10⁻⁷ | 5.341×10⁻¹² |

These are coverage ceilings over *all possible* nonidentity subgroup targets,
not measured success probabilities for the three pinned deterministic targets.
They make mask 2 a low-coverage candidate despite its smaller formula.

For illustration, trial-1 Gray 4+2 mask 0 has 118,684 variables and 547,615
original clauses; masks 1 and 2 each have 76,686 variables and 423,696
clauses. Mask 5 is smaller (34,688 variables, 299,777 clauses), but its
necessary pair sum does not exist in these exact bases. Formula size therefore
cannot rank useful relation yield, much less total IC runtime. The hosted
[fresh-trial solver panel](../natural-yield-panel-20261025/PROTOCOL.md) already
includes masks 0 and 1 for trials 1 and 2; mask 2 is an untested natural
search arm. That panel's GitHub Actions run `40185426122` was queued when this
result was recorded. `selected_best_total_runtime` remains `null`.

Recheck the committed receipts against the exact cached archives with:

```sh
python3 research/koblitz_n83_factor_base_sweep_20261008/verification/full-identity-mask-census-20261025/audit_replay.py \
  --inputs /private/tmp/n83-complete-s3-get-5_f2e7o_
```

The two full XCNF exports used 877.649 seconds of aggregate panel wall, plus
the separately measured pair-screen audit. None of those charges is a cold
one-target IC runtime, and the finite mask census does not close the larger
factor-base, solver and symmetry-composition search.
