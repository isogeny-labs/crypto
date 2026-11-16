# Independent review: target-information diagnostic

Verdict: the stated linear-information criterion is correct, and no correctness defect was found in the reviewed diagnostic. Six producer tests were independently rebuilt and rerun; two additional reviewer tests passed, including all 793 two-row, one-nuisance matrices over F2 and F3 checked against explicit solution sets. This is bounded software correctness checking, not a scientific cohort or performance observation.

## Mathematical check

For Ax+bk=-c, consistency is equivalently -c in col([A b]), hence rank([A b c])=rank([A b]). If b is not in col(A), subtracting two solutions gives A dx+b dk=0; dk!=0 would imply b in col(A), impossible. If b=Av and one solution exists, (x-tv,k+t) is a solution for every t; since a prime field has at least two elements, k is not unique. Adding one column changes rank by at most one, yielding the stated rank difference. The implementation explicitly gates target_determined on consistency.

## Implementation and dependency check

`information` checks nuisance_columns+2 for overflow and checks exact row width. The following +1 and rank+1 expressions cannot overflow for any realizable input satisfying that bound. It projects each row to the first n, n+1 and n+2 columns and invokes Basis separately. Basis computes forward echelon form with normalized nonzero pivots, so its retained pivot count is rank. Incoming coefficients are reduced modulo p, duplicate accumulations are canonicalized and zeros removed. The internal u128 arithmetic supports u64 modular products without overflow. Basis constructor rejects nonprime moduli using its existing deterministic u64 primality implementation; this review did not independently prove the primality algorithm. Empty systems and zero nuisance columns are supported; p=2 is handled by the additional test.

The code passes usize::MAX coefficient limits. This is an intentionally unbounded offline diagnostic, not a memory-safe admission policy for arbitrarily large input streams. It performs three reductions and changes the kernel's thread-local metering; the header and report correctly state that it must stay outside a measured producer. There is no silent partial-rank acceptance after a cap outcome.

## Existing collector behavior

GraphSolver::new assigns both final label columns to small columns and preserves their order. Collector::new enumerates small columns in original order. GraphSolver::relation returns early for dependent/joined/anchored admissions, then on Independent checks that no constant pivot exists and examines the target pivot. Thus it already uses a target-specific extraction condition, rather than a generic total-rank stopping threshold. The report explicitly preserves this fact. A dependent insertion cannot newly increase the row space.

Scalar convention deserves retention in any future exporter: the diagnostic states Ax+bk+c=0, so a normalized row k+c=0 means k=-c. GraphSolver returns its final-column coefficient under the collector's signed-label convention. The present diagnostic reports only consistency and uniqueness, which are unchanged by an invertible sign change of a target column; it does not claim to recover GraphSolver's scalar. A future scalar-valued comparison must explicitly bind that convention rather than copy the returned coefficient into this equation without checking its sign.

## Report assessment and remaining boundaries

The report's rank-one comparison is exact: x=0 leaves k arbitrary; k+3=0 over F7 determines k=4. Both full coefficient arrays have rank one. The proof, fixture labels, counter caveat, no-new-stopping-rule statement and lack of historical per-row reconstruction are accurate. It makes no collector improvement or cost-ratio claim. The review supports this component scope only; actual archived relation-stream fidelity, instrumentation overhead, or a changed collector policy needs separate evidence.

## Review provenance

The initial tool read displayed the full diagnostic and draft report while fetching the claims header. Therefore this is NOT a source-blinded review. PLAN.md was frozen before dependency/GraphSolver inspection and native checking. The reviewer is a separate agent from the producer. Source hashes before and after checks are retained; diagnostic and dependency bytes stayed unchanged, while the producer changed the draft report renderer and shortened a source-path label. The updated draft was reread: its mathematical statements and claim boundaries remain supported. Root files were not changed by this reviewer. Native execution used an isolated copied crate and private CARGO_TARGET_DIR, with the production dependency at its pinned read-only source path. Each command had a 120-second limit. Tests and logs remain in this packet.
