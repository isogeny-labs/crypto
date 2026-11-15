#set page(paper: "a4", margin: 19mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: "1.")
= Native mode and Taylor-data dimensions
15 November 2026 | PCC-20261115-907a94: *unverified*

The new native admission.dimensions utility evaluates attempted-mode,
interpolation/jet and coarse path-precision expressions without allocating a
graph. Five source-routine controls accompany it. These are unverified
producer assertions and software checks, with no signed proof verdict.

== Concrete storage correction
Interpolation returns I coefficients. The recursive contractor resizes and
truncates that vector to qt, then retains it in a Jet. Truncation drops excess
Elements and their inner allocations but retains the outer Vec buffer.

The native fixture M=2, a=2, u=2 has I=15 and six active nodes. Four jets
have length 6 and two length 3: 30 live Elements occupy at least 90 outer
Element metadata slots. This is a source-shape control; no RSS, byte budget,
runtime ratio or admitted recursive execution follows.

#figure(image("dimensions.svg", width: 78%), caption: [Lengths and capacities are distinct storage inputs. Not timing data.])

== Dimension contracts
For `1 <= u <= min(a,M), 0 <= i < u, L=2^a, K>0, p_t>=1, K*L<=3M`:

- Per-weight modes `<= K*2^(i+1) <= 6M/(L/2^i)`.
- Weight-loop attempts `<= K*2^(i+1)*(p_t-1)`, including discards.
- Samples per node `I=M*(2L-1)+1`.
- Active nodes `N=2*(L-L/2^u)`.
- Logical jets `J=h*((2u-2)*L+2L/2^u), h=ceil(3M/u)`.
- Retained outer jet-vector slots `>= N*I`; this is a lower bound.

#pagebreak()
== Coarse precision envelope and its limit
Let s bound root edges, `lambda=ceil(log2(s)), u0=floor(lambda/131072),
A=4u0, L*=2^A`. Source parameters give `a<=A`. On a shrinking path with
`M_next<=M/8`, sum `M<=2s`. The precision increment satisfies:

```text
delta_p = 8*ceil(3M/u)*(L/2^i) + 8M*(a+1)
        <= 8M*(3L*+A+1)
p_path <= P = p0+16s*(3L*+A+1).
```

When p0 is polynomial in s, the candidate, sample and logical jet dimensions
have polynomial envelopes in s alone. These expressions do not import the
original-description length inside an entry exponential. The utility checks
scalar input premises; a caller still owes their binding to actual graphs.
The derivations remain unverified.

The coarse P cannot replace the sharper tapered `p_t=O(M*D/u)` required by
Appendix D.15. Tuple and DFS refinement, rational bit widths, retained
allocation unions and original-predicate contracts remain open. No general
contraction complexity theorem or new Lean proof is asserted.

== Software checks and delivery
Five controls compare native grid/multiplicity counts, retained modes of each
weight including zero filters, source precision scales and native increments,
invalid premises and p=1, and actual interpolation-vector capacities.
The final release-library census passed 104 tests locally. Formatting and
strict library-only clippy passed. Strict all-target clippy encounters the
unchanged manual_range_contains warning in treewidth.rs:848, also reproduced
on the base. These are producer checks, not signed mathematical verification.

No native contraction body, recursion guard, gate, dependency, frozen evidence
or existing ledger event was changed. No scientific benchmark or cloud job
ran. Canonical measured graphs receive no new numerical result.

== Source and proof map
Base crypto commit: d60f19656854fdfa1b37d4976652e614db7fcc91.
The successor starts from the unmerged tuple/DFS and packet-recurrence reviews
in PRs 3478 and 3569; their divergent ledger histories remain untouched.

Primary text read: Bochum v0.2, sections 1.2 and 2, Theorem 2 and Remark 1
(1ee44e56); OpenAI, Appendix D.1, D.4.1–2 and D.4.4–7 (42907c9e).
The full references, source SHA256 manifest, derivation chain, open proof
obligations and reproduction commands are in the adjacent README.md.

KB unavailable: MCP tools absent and repository wrapper has no backend.
Used exact primary files and current PR records. No novelty claim.

Before a Lean edit, instantiate existing finite-sum/geometric-series laws in
Mathlib or crypto-lean and bind the actual grid, multiplicity and Vec lifetimes
to the semantic objects. Those bindings are not discharged here.

*Next:* refine the sharp D.11 taper and D.14–15 enumerator bounds, then total
retained capacities along a path. Retain the 30-versus-90 fixture as a check
against equating logical lengths with outer-vector allocation slots.
