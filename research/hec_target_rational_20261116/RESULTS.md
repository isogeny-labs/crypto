# Target-conditioned interpolation: exact residual equivalence

The three-anchor target-conditioned rational family is implemented. Every admitted residual equals direct Cantor reduction of -D-P1-P2-P3, including repeated nonbranch target support and degree drops. This closes an implementation/domain gap left by the simple pencil, while showing that this family supplies no additional relation information for the same signed inputs. Classical Riemann–Roch/Cantor algebra, not a novelty or performance claim. [pcc:unverified]

## Frozen finite gate

Source61bcd604593f, pre-run S3 checkpoint20261116T161039Z-61bcd604593f. Scientific source/lock/native imports/binary/protocol/theory frozen before once-only execution. EXIT.txt=0 and CONTROL.stderr empty. Six reused direct-profile targets on g2/F251, g3/F211, g4/F61; no fresh-holdout claim. Target construction is outside the public-divisor construct API. H=8/9/10 prefix anchors; at most64lexicographic triples, with all8sign masks for each. Genus2 has only56triples. No replacements, sign folding, parameter adaptation or rerun.

| Genus / p | Target | Triples | Sign-class attempts | Admitted exact matches | Nonprimitive rejected | Split group checks | Nonsplit | Degree drops | Repeated-target admitted |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|2 / 251|0|56|448|436|12|209|227|2|0|
|2 / 251|1|56|448|448|0|201|247|3|0|
|3 / 211|0|64|512|492|20|75|417|5|0|
|3 / 211|1|64|512|503|9|82|421|2|503|
|4 / 61|0|64|512|485|27|39|446|6|0|
|4 / 61|1|64|512|492|20|34|458|7|0|

Native reduction totals:2944attempts,2856admitted exact matches,88nonprimitive rejections,640original-group checks for split residuals,25degree drops and503admitted repeated-target cases. No other rejection category occurs in this finite panel; raw zero counters remain. These are exact finite counters, not population estimates or uncertainty intervals. Sign attempts are not unique relations or matrix rank. Branch sign classes may coincide; no uniqueness/rank claim is made from attempts.

For every admitted pair, full norm division by u*product(x-xi) is exact, residual degree<=g, B is invertible modulo residual Q, and the resulting Mumford representative is valid. Exact equality to three signed Cantor point additions is checked without projection. Smooth/nonsmooth checks use the same root oracle on identical residuals, so they are not independent root-finding validation. Split residuals additionally reconstruct the original Jacobian identity with multiplicities. Field/group arithmetic is shared; no independent scientific validation or machine-checked full group certificate.

## Mathematical consequence

With B=b0+b1x, C=c0+c1x and A=vB+uC, the three anchor constraints are linear in four homogeneous coefficients. Primitiveness and denominator hypotheses give a degree<=g residual E and the exact principal relation

    D + P1 + P2 + P3 + E = 0.

Uniqueness of the reduced representative then forces E=reduced(-D-P1-P2-P3). The generic target-conditioned higher-pole construction is therefore a presentation of that group orbit. For the same admitted signed inputs it cannot improve residual splitting or relation information. This statement does not compare samplers, bound all arithmetic implementations, or exclude faster batching. Rejected nonprimitive cases are outside the gate; primitive normalization with canceled conditions would need separate accounting.

The target sign remains fixed. Global negation changes D as well as anchors, so the old target-free four-class sign fold cannot be applied; eight anchor masks are retained. One can fold only under an additional proven equivalence or by also accounting for the -D workload. The finite counts are not doubled relation rank.

Repeated target support works because A-vB is divisible by the entire u, including multiplicities; the condition gcd(u,v)=1 keeps target support nonbranch. The earlier repeated-support input now has503admitted exact residual comparisons. Existing Cantor operations already support it, so this domain extension is not evidence of a speedup over that comparator. Source descent and a complete logarithm solver remain outside this gate.

## Cost and next decision

Three forced anchors do not generate a new residual for a fixed signed input. Any useful successor needs an explicit cheaper evaluation/batching mechanism or a new sampler that reaches useful input combinations at lower complete cost. It must charge nullspace, primitive/denominator checks, norm/residual formation, roots, catalogue, matrix and all source/target work against Compact. No timing campaign is justified merely by replacing the residual's representation or by comparing against Gaussian interpolation alone. No count-unit or elapsed speed ratio is inferred here. The mathematical tenfold objective remains active.

## Evidence and workflow

[Derivation/source receipt](THEORY.md), [frozen protocol](PROTOCOL.md), [raw counters](CONTROL.tsv), [native totals](TOTALS.txt), [graph](COUNTS.svg), [PDF](reports/run-20261116T1610Z/REPORT.pdf). Rust reducer reads fixed raw counters and renders the figure; no scientific Python. Both report pages inspected. Source/lock and original raw outcomes are committed; prior sealed reports and canonical performance graphs unchanged. No timing benchmark, benchmark-index/leaderboard entry, independent VALIDATION.json or performance promotion. Primary external-PDF access gap remains explicit and no priority claim follows. Today's shared session log is reserved by T-6720 verifying; SESSION.md carries this scoped handoff. PR remains unmerged.
