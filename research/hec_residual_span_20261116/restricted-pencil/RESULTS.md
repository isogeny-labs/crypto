# Restricted producer: exact rows, incomplete genus4 panel

Once-only run at aec41463d93 exits0 with64countedcalls and24incomplete calls. These are two repetitions of32workloads:20complete(12genus2,8genus3),12genus4unresolved at the fixed128anchors. Incompleteness is retained, not a successful attack or favorable speed ratio. All561acceptedprefixes agree with independent solve_for_last; every returned scalar verifies against the original public source/target and expected scalar. All128modulusrecords repeat and satisfy REDCidentity. [pcc:unverified]

|genus|completed workloads|base-field old/new REDC on completed cases|subgroup old/new REDC on completed cases|
|---|---:|---:|---:|
|2|12/12|0.541971–1.083696|0.791111–3.079288|
|3|8/8|0.719255–0.966886|0.951841–2.283154|
|4|0/12|censored incomplete|censored incomplete|

Ratios below1 are regressions. Each range is conditional on completion and not a genus-wide speedup. SUMMARY.tsv includes every raw per-case old/new count; unresolved cases explicitly print censored_incomplete instead of a ratio. No modulus weighting, CPU/wall or external-fastest claim. Old retained baseline completes all32workloads.

The implementation actually shortens point scanning to a fixed small-orbit union, uses touched-parameter BTreeMap buckets rather than O(p)allocation/scan, factors the remaining degree<=2polynomial, and retains all failure/reconstruction/normalization/finalverification costs. It permutes LPcolumns first in the general sparse solver; no unweighted graphcycle or freeLPlog is used. Full FBenumeration/partition/action setup remains charged. This is a stronger cost test than filtering rows after a fullscan, but it is the classical shortened sieve, not a priority claim.

## Yield/cost interpretation

Shorter scanning does not guarantee cheaper usable rows. Under an explicitly heuristic model of independent residual roots landing in a small subset of fractionrho, a fully split degreeg+1residual has at leastg-1smallpoints with probability

    sum_(j=g-1)^(g+1) binom(g+1,j)*rho^j*(1-rho)^(g+1-j).

For smallrho this is asymptotic tobinom(g+1,2)*rho^(g-1). Scanwork scales asrho, so scanwork per accepted fullysplit residual scales asrho^(2-g), before quadraticfactoring or LPelimination. This is not a theorem about the structured inputs or useful rank. Large-prime techniques can still win overall by reducing the final matrix; this test shows why one cannot claim their full gain merely from fewer scanned points. The observed genus4rank failures are retained, not explained away by the heuristic.

Decision: close this fixed-cut producer as a completing tenfold candidate on thispanel. It fails completion forgenus4 and supplies no tenfold completedcostcase elsewhere. No adaptivecut,extraanchors,rerun or favorable subset promotion. General mathematical10xgenus2+objective remains active.

Raw gate/count/output/exit/run records, frozen source/native/import/input/binary hashes and Rustsummary retained in Git while S3 unavailable. No S3polling,cloudspend or oldbaseline rerun.
