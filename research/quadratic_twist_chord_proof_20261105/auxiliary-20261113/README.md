# Twists as auxiliary trace fibers

Date: 2026-11-13. This is a source-linked desk derivation and next-test design,
not an executed experiment, checked Lean theorem, IDC1 certificate, novelty
claim or measured improvement. It develops the existing trace proposal
`PCC-20261105-cfd582`, whose standing is unverified. Every mathematical
assertion below remains unverified pending the listed checks. [pcc:unverified]

## The lead: use the nonsplit chord branch

Let E/Fq be the nonsingular short model y²=x³+A*x+B, with characteristic
at least 5. Let d be a nonsquare, K=Fq(w), w²=d, and sigma(w)=-w. Supply an
actual nonidentity source point Q=(a,b); b²=a³+A*a+B is a hypothesis, not a
target-log lookup. For a line through -Q, put

    L(X) = l*(X-a)-b
    s = l²-a
    p = a*l²+2*b*l+a²+A
    Delta_Q(l) = s²-4*p
               = l⁴-6*a*l²-8*b*l-3*a²-4*A.

The auxiliary chart to investigate is

    C_Q: d*t² = Delta_Q(l),   l,t in Fq.

For t != 0, set X=(s+w*t)/2 and Y=L(X). The candidate points R=(X,Y) and
sigma(R) are distinct conjugates: X solves X²-s*X+p=0, both points lie on E,
and the chord through them contains -Q. Thus R+sigma(R)=Q. Unlike the
source factor-base chord census, the two abscissae here are NOT in Fq.
They must not be admitted as two main-curve factor-base points. [pcc:unverified]

**Readable algebra.** Expand s²-4*p to obtain the displayed quartic.
The identity 4*(X²-s*X+p)=(2*X-s)²-(s²-4*p), together with w²=d and the
chart equation, gives the quadratic condition. The previously proposed
curve/line factorization then gives the curve equation. Frobenius fixes l,a,b
and negates w, so the conjugate has the other root and lies on the same line.
The ordinary chord law gives the trace relation. Each step is an obligation
to bind to actual finite-field and nonsingular-group objects. [pcc:unverified]

### How the actual twist enters

Use the monic twist T: Y²=X³+d²*A*X+d³*B and the explicit embedding

    psi(X,Y) = (X/d, Y/(d*w)),   psi(O)=O.

Its coordinates satisfy E's equation and sigma(psi(T))=-psi(T). For Q in
an explicitly supplied odd-order subgroup of order r, choose public h with
2*h=1 mod r and put S_Q=[h]Q. For any candidate R with trace Q,

    C = R-S_Q,   sigma(C)=-C,   R=S_Q+psi(T_aux).

The proposed inverse is psi^-1(C)=(d*x(C), d*w*y(C)), with O handled
separately. Its output is Fq-rational because C is Frobenius-minus.
Neither S_Q nor this translation needs the unknown scalar of Q. Computing [h]Q and all target-dependent maps must be charged to the online workload. This is
an auxiliary twist torsor/presentation, not a rational homomorphism from
the original main subgroup to a different twist subgroup. [pcc:unverified]

**Cases that stay explicit.** If t=0, the residual roots coincide and the
candidate must satisfy the tangent equation 2*R=Q, rather than a secant
formula with a zero denominator. Such rational halves are retained, not
discarded. Q=O uses a separate trace-zero/identity branch, not affine a,b.
Projective points, tangent denominators, both t signs and actual twist-map
infinity cases need full coverage. No coverage theorem is claimed here.
The map R -> sigma(R) changes t to -t; after translating by S_Q its action
is C -> -C. It is NOT permission to count the two witnesses as two useful
coefficient rows. [pcc:unverified]

## A necessary control: free decorations can be redundant

For the earlier fixed-lift construction, let B_i,Q be in the odd subgroup,
C_i=psi(T_i), and A_i=[h]B_i+C_i. A source relation sum(c_i*B_i)=Q gives

    sum(c_i*A_i) = [h]Q + psi(sum(c_i*T_i)).

Conversely, taking trace of an admitted lifted equation recovers the source
relation. On a valid row, injectivity of psi makes its auxiliary witness
exactly T_aux=sum(c_i*T_i). Therefore a free auxiliary T does not create
additional coefficient rows for a fixed base. It can still change the
polynomial presentation or the degree at which a particular solver certifies
a row; that is the discriminating hypothesis, not an increased fiber count.
This calculation closes only the free-decoration yield interpretation,
not the auxiliary-encoding research program. [pcc:unverified]

If T_aux is restricted to a proper set, its new membership condition may
remove valid rows. Keep every removed row as a loss. If the lifts themselves
vary, state their allowed set and project/deduplicate BEFORE rank. Equal
projection alone is neither lower certificate degree nor cheaper solving.
The ordinary source baseline must be offered the same extended formulation,
not an artificially expanded elimination-only implementation. [pcc:unverified]

## What would make this useful

The concrete comparison is **retain the trace-fiber variables versus
eliminate them**, on identical source bases, actual Q and original relation
ideals. Compare with optimized source S3/membership and the earlier chord
chart. Include a source-labeled arm receiving exactly the same lifted
equations, and generic point enumeration as a separate reference.
Measure admitted projected rank, actual named matrix degree where applicable,
expanded original-generator witness degree and all extension/map/setup work.
A quartic input, more auxiliary points or a lower first-fall degree is not
an intrinsic-regularity or cost result. No such improvement has been measured.
[pcc:unverified]

An actual main-DLP benefit still requires legitimate public factor-base logs,
accepted relations transporting to sum(c_i*B_i)=Q, and the resulting scalar
checked on the ORIGINAL E/P. Keep the full strongest-source policy frontier,
matched/pulled-back policies, blocked setups, failed shifts and all losses.
Do not substitute known synthetic target labels for Q. [pcc:unverified]

## Next bounded test

[NEXT_TEST.md](NEXT_TEST.md) separates geometric coverage from the later
solver/recovery comparison. Its first fixture is the historical F37 model
E=(7,19), nonsquare d=35, T=(28,33), source P=(1,8), r=19 and h=10.
These exact tuples are historical development inputs, not new registry aliases
or newly checked points. The new native producer/auditor do not exist yet.
There is no executed census, frozen quantitative analysis or new scalar result.
[pcc:unverified]

The four declarations in [drafts/TraceFiber.lean.txt](drafts/TraceFiber.lean.txt)
are handwritten, unelaborated algebra targets outside automatic proof-gate
projects. They are not checked theorems. The actual-field Frobenius,
nonsingular group/chord law, tangent/projective coverage and twist inverse
bridges remain open. No new IDC1 is issued or implied by this document.
[pcc:unverified]

## Sources and scope

- Existing [three-idea proposal](../../quadratic_twist_followup_20261105/IDEAS.md),
  [chord algorithm](../../quadratic_twist_chord_preflight_20261105/ALGORITHM.md)
  and [Chord.lean](../Chord.lean). Their execution/standing is unchanged.
- Gorla--Massierer, [Index Calculus in the Trace Zero Variety](https://arxiv.org/html/1405.1059),
  section 2.2: trace and Frobenius. Their later index-calculus analysis concerns
  n>=3; it does NOT supply a degree-two/main-curve speedup for this chart. [pcc:unverified]
- Kosters--Yeo, [Notes on summation polynomials](https://arxiv.org/html/1503.08001v2),
  sections 2 and 4--5: summation systems, Weil descent and the distinction
  between first-fall degree and regularity.
- Sutherland, [18.783 Lecture 8](https://math.mit.edu/classes/18.783/2017/LectureNotes8.pdf):
  finite-field twist background. The displayed chart is a desk derivation here,
  not a formula attributed to that note.
- Repository searches `nonsplit chord trace fiber` and `trace auxiliary Frobenius quartic`
  found the existing proposal, the transfer survey and Nov12 lead notes. The survey
  points to `crypto-autoresearcher:knowledge/leads/ecdlp/transfer/` and established
  transported-base/trace mechanisms. No novelty or disposition change is claimed.
- **Sources checked (KB): unavailable.** No KB MCP tool is exposed; the official
  branch wrapper returned127 (no executable). The primary texts and linked
  repository antecedents are the fallback, not fabricated KB hits.

## Artifact and execution status

See [report.pdf](report.pdf), its editable [report.tex](report.tex), and
[trace-fiber.svg](trace-fiber.svg)/[trace-fiber.dot](trace-fiber.dot). This is a
diagram-only report: there are no measured values to plot. Historical study
graphs, scoreboard, benchmarks and metrics are unchanged because this round
adds no measured data or verified curve/map edge. [pcc:unverified]

The runtime was replaced: old local worktrees, toolchain caches and private
checkpoint selector are absent in this fresh filesystem. The published branch
was restored by a sparse public clone; old remote evidence was not changed.
No gate, proof/native experiment, arbitrary S3 upload or paid compute was run.
Existing branch upload approval persists; secure narrow access and pinned
toolchain restoration are prerequisites to affected scientific execution.
Conductor is unavailable (command not found); no lease or reservation is claimed.
The paused automation is not revived by this research note. [pcc:unverified]
