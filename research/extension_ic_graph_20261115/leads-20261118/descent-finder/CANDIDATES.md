# Candidate improvements to the extension-field descent

Every number here is a producer observation or a model, labelled
[pcc:unverified]. No candidate below is a measured end-to-end improvement.

## Setting (from the code, not assumed)

The graph pipeline's descent (`jv_extension_deferred_pair_dlp.rs`,
`DescentBackend::Original`; the same `nagao_decompose` as original IC) walks
the residual stream `R_i = a_i G_J + b_i Q_J` on the genus-3 Jacobian over
`F_{p^2}`. Per residual it builds Nagao's system (Nagao, ANTS IX 2010;
Joux–Vitse, EUROCRYPT 2012, §3.1): the quotient
`S = (C^2 H − A^2)/u_R`, `C = x + mu`, with three `F_{p^2}` unknowns
`(lambda0, lambda1, mu)`; asking the six coefficients of the monic sextic `S`
to lie in `F_p` gives six quadrics in six `F_p` unknowns (Bezout bound 64).
F4 (grevlex, stopped at a 64-monomial staircase) and the linear stage
(`zero_dim_la::fast_stage`: transposed Krylov FGLM, `gcd(f, x^p − x)`,
rational read-out) give the `F_p`-points; a point is a decomposition only if
`S` has six distinct `F_p` roots (heuristically `1/720` for a random sextic)
and the ordinates are consistent.

## C1. F4 trace replay (implemented here as an opt-in path)

Source: Joux and Vitse, "A variant of the F4 algorithm", CT-RSA 2011
(LNCS 6558; IACR ePrint 2010/158): record which rows of each F4 step were
useful on one system, replay only those rows on later systems of the same
shape, check the new leading monomials at each step. The engine already
implements record/replay (`f4_fp::f4_record`, `f4_replay`); this lane's
pipelines never enabled it for descent. All 979 occurrences of
`descent_replayed` in recorded JSON under `research/` and `benchmarks/`, found
by grep, are 0. The lane's older ledger
(`research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md` §12,
2026-10-04, a different pipeline) reports `2.44x` fewer F4 multiplications
per test with replay. [pcc:unverified]

Exact correctness condition (as implemented in `traced_descent_check.rs`):

1. Every row a replay reduces is a monomial multiple of an input polynomial or
   of an earlier element, so every returned polynomial lies in the input ideal
   `I`. This holds by construction of `f4_replay`; it does not depend on the
   trace being "right".
2. Accept a replayed report only if it did not time out, dropped no pair above
   the degree bound, and either derived `1` (so `V(I)` is empty, exactly) or
   stopped at a staircase of at most 64 standard monomials. The second case is
   the same certificate the production arm already relies on: `J ⊆ I`,
   `dim R/J ≤ 64`, and `dim R/I = 64` for a system without solutions at
   infinity, so `J = I`. For a degenerate system (degree below 64) neither the
   production arm nor the replay is certified by this argument. That is why the
   local control includes an oracle arm that runs F4 to a certified basis with
   no stop.
3. Everything else is rejected: in particular a replay that returns early on a
   shape mismatch (`staircase_at_stop == None`; `f4_fp` then returns a
   non-basis with `trace_mismatch`). The production `solve_zero_dim_traced`
   would pass that non-basis to the linear stage. This path therefore never
   calls `solve_zero_dim_traced(trace = true)`; it re-solves by ordinary
   staircase F4 and charges both runs.
4. The learning run's tracked RREF (`tracked_rref_contributors`) is not counted
   by `f4_fp`. An explicit upper bound (`steps · rows · cols · rank` multiply-adds)
   is reported and charged in the most conservative comparison.

Local result (identical residuals and RNG seeds; see README):
F4 multiplications fall by 2.29x on the Run50 deferred stream and by 2.30x on
the nagao-e2e stream. Under the conservative pricing the descent subtotal
falls by 1.71x and 1.62x. Solution sets and decompositions agree with
production and with the certified oracle on every one of 4,921 residuals.
[pcc:unverified]

## C2. Anchored `(1^4, 2)` partial decompositions against the deferred graph's exact-log class domain (not implemented)

Sources: Joux–Vitse, "Cover and decomposition index calculus on elliptic curves
made practical", EUROCRYPT 2012, §§3.1–3.3 (author PDF SHA-256
`7ca88c54…b38`, already recorded by this lane); Gaudry, Thomé, Thériault and Diem, "A
double large prime variation for small genus hyperelliptic index calculus",
Math. Comp. 76 (2007); the lane's
`LEAD_ANCHORED_QUADRATIC_DESCENT.md` (sign and row derivation).

Mechanism. This uses the same F4 solution as C1, so it composes with C1. Accept
an `F_p`-solution whose sextic factors as four distinct supported `F_p` roots
times a monic irreducible quadratic `U`. It gives `R + Σ s_i B_i + c_D D = 0` for
the degree-2 class `D = (U, V)`, `V = −A/C mod U`. Only the post-processing
changes, and that is 0.17% of the legacy descent multiplications. For a random
sextic this factorization pattern is `1/48` likely against `1/720` for full
splitting, a factor of 15. Stage43 recorded 26 such partials against 2 full
decompositions over 1,506 residuals. [pcc:unverified]

Exact correctness condition (the Run50 NEXT.md requirement, made explicit):

1. Class-domain match. `D` must be equal, as a signed canonical `(U, V)`
   Mumford pair in the same encoding and normalization as the collected graph
   vertices, to a class whose discrete log is determined by the certified
   homogeneous kernel. Today that means a retained anchor
   `Σ s_A_i B_i + c_A D = 0`. A graph vertex is admissible only if a recorded
   chain of collected rows connects it to an anchor, every row in that chain is
   in the system the kernel was certified on, and the chain's witness is
   replayed. `U` alone is not a class.
2. Exact incidence. The affine row substitutes `D` by its exact witness
   combination. It keeps target coefficient `−b`, right-hand side `a`, and the
   cofactor/sign conventions of the full materializer. The cancellation witness
   and the combined group identity `R = −Σ s_P B_P + c_D c_A Σ s_A B_A` are
   checked in the group, then projected with the actual kernel. Elliptic target
   replay stays mandatory.
3. Any unmatched partial is discarded and counted. A partial is never counted
   as a decomposition.

Yield model (to be measured, not asserted). Per F4 solve,
`P(row) ≈ P_full + P_(1^4,2) · |D_exact| / |D_universe|`. Here `|D_universe|`
is the number of signed canonical degree-2 classes with `U` irreducible over
`F_p`; a heuristic estimate is `≈ p^2/2`, about 5·10^5 at `p = 1009`, and it must
be counted, not assumed. The open quantity is `|D_exact|`. Stage43 had 11,206
retained classes and found 0 of 26 matches, consistent with an expectation of
about 0.6. Run50's final deferred graph reports 125,491 observed classes and
20,510 anchors, but not how many classes have exact logs after the kernel.
An illustration, not a result: take pooled `P_full ≈ 4/4921 = 8.1·10^-4` from
the two local streams, `P_(1^4,2) ≈ 26/1506 = 1.7·10^-2` from stage43, and
`|D_exact|` equal to the anchors alone, about 4% of the universe. Then
`P(row)` rises to about `1.5·10^-3`, roughly 1.9x fewer residuals. The sign
and universe conventions make this uncertain by about a factor of 2. If most of
the 125k classes are exact-log, the model gives several times fewer residuals.
That range is the reason to measure it. [pcc:unverified]

## C3. Early abort before or inside the linear stage (rejected after reading the code)

Correctness: if `gcd(m_ℓ(T), T^p − T) = 1` for the minimal polynomial of
multiplication by any linear form `ℓ`, then `V(I)(F_p)` is empty, because an
`F_p`-point `s` makes `ℓ(s)` an eigenvalue in `F_p`. This needs no separating
form. But `fast_stage` already computes `gcd(f, x^p − x)` before any read-out
and skips the read-out when there are no roots. The remaining cost, normal
forms and `2δ` Krylov products, is what produces `f`. A pre-F4 abort has no
cheap test, because the splitting pattern of `S` depends on the solution. The
sextic roots, ordinates and signs stage costs only 0.17% of legacy descent
multiplications. Not pursued.

## C4. Linear stage reuse across residuals (open, not implemented)

With replay accepted on 3,414 of 3,415 systems, the staircase is the same
almost everywhere. The normal-form pattern that builds `M_{x_n}` (the
`nf_muls` part of `fast_stage`) could be recorded once and replayed, in the
spirit of Faugère–Mou sparse FGLM (J. Symbolic Comput. 2017) and of C1. The
linear stage is 22% of legacy descent multiplications and is almost entirely
unhooked (below). After C1 it is the largest remaining unpriced descent term.
Correctness condition: the replayed normal forms must be checked against the
accepted basis (each NF reduces to zero modulo the basis minus its standard
part), with fallback otherwise.

## C5. Coordinate reduction 6 to 5 (prior lane work, measured)

The exact constant-pivot elimination (`rho-parity/unit_elimination.rs`) was
measured end to end in `nagao-e2e/`: paired wall ratio 0.998 (95% CI
0.967–1.027), no demonstrated gain. One possible reason, untested: counted
F4 savings do not reach wall time because the linear stage (unhooked) and
other uncounted work are a large share of wall. [pcc:unverified]

## Knowledge-base receipt

`scripts/research-kb search …` printed: `research-kb: no executable found; set
RESEARCH_KB_BIN or put research-kb on PATH`. No research-kb MCP tools are
loaded in this runtime. Fallback sources read: the lane's own
`LEAD_ANCHORED_QUADRATIC_DESCENT.md`, `AMENDMENT_13_CERTIFIED_TRACE.md`,
`RESUME_UNIT51.md`, Run50 `NEXT.md`, `runpod-owned-descent-20261117-43/README.md`,
the cover-decomposition ledger §12, and the source of `f4_fp.rs` (trace
record/replay), `f4_fp/reuse.rs`, `jv_cover.rs` (nagao_decompose_using,
solve_zero_dim_mode), `zero_dim_la.rs` (fast_stage). The primary papers were
not re-downloaded in this session; their citations rely on the lane's earlier
recorded reads and on bibliographic knowledge. [pcc:unverified]
