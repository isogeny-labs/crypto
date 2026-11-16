#set page(paper: "a4", margin: (x: 18mm, y: 15mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9.3pt)
#set par(justify: false)
#align(center)[
 #text(size: 19pt,weight: "bold")[Shared norm core elimination]
 #v(3pt)
 Exact graph substitutions • fixed software controls • 15 November 2026
]
#v(6pt)
Eliminate the target coefficient, monic support coefficients and public
pre-wrap remainder coordinates before native solving. At depth4, arities3/5/7
use 15/18/22 variables with equations of degree at most3. Every extracted
assignment is lifted and checked against every original equation before
point decoding and group-sum verification. *PCC-20261115-be28ec:unverified.*

#v(5pt)
#align(center)[#image("pipeline.svg",width: 100%)]
#v(5pt)
#text(size: 8pt)[Diagram source: pipeline.dot; SVG links point to implementation,
derivation, protocol and full evidence. Counts are exact properties of this
encoding, not cost measurements. No statistical performance graph applies.]

#pagebreak()
#text(size: 16pt,weight: "bold")[Checked scope and unresolved solver cost]

The target equation determines `a0=s C(r)−r^h−sum a_i r^i`. On the curve,
`N=A²−f C²` vanishes at r. Monic synthetic division by `X−r` determines U
polynomially, without coefficient enumeration. Before `2^j≥m`, quotient0 and
remainder `X^(2^j)` are public. Remove only the known higher terms of the
first wrapping quotient, then keep the remaining unknown circuit.

#table(
 columns: (1.05fr,1.95fr), inset: 6pt, stroke: 0.4pt + rgb("b3c2cd"),
 table.header([*Fixed control*],[*Producer-local exact outcome*]),
 [Structural/parent], [15/18/22 variables; degree≤3; symbolic cancellation and admission controls; retained28-model parent check.],
 [Complete m3 sets], [90 cases /400 models; independent OpenSSL full sets agree.],
 [Cold point-only IC], [18 targets /72 signed relations /18 kernels replay in OpenSSL.],
 [m5,t3,c16], [2 complete models; quotient dimension8.],
 [Dense m5 / m7], [m5 exit139; m7 cap124; empty bases, both unknown.],
 [D1 diagnostics], [Both exact linear-algebra variants reproduce exit139 on the dense m5 input.],
 [D2 memory trace], [Repeated ENOMEM then null SIGSEGV under the same2GiB address limit.],
 [D2 pair batch512], [Reaches the unchanged cap, exit137; no basis. Partial F4 degree7 is not a certificate-degree bound.],
)

#v(6pt)
Shared Lean source `52bc3fb` supplies retained-power congruence, exact target
norm division and retained-circuit norm charts. The assembly probe, focused
build, explicit axiom audit and complete shared-library gate pass:101 theorems,
zero failures or trusted native proofs. New laws use only propext,
Classical.choice,Quot.sound. Full coefficient graph equivalence and bounded
witness lifting remain unformalized.

The general subexponential prime-field algorithm remains open. Variables
still grow as `O(mt)`; a small final quotient and these substitutions do not
bound initial elimination. The next solver interface should retain input-row
provenance under an explicit Macaulay cap, with a separate proof of useful
yield on growing natural targets. No scientific timing, speedup, fitted
exponent, leaderboard change or paid run is claimed.

#text(size: 8pt)[
*Custody:* native source b5e2b31d4235834ad37486091ca4acf6470d16e4; source/binary
hashes captured before calls. Three complete raw panels: core-001,diagnostic-001,
diagnostic-002; full archives and SHA256SUMS in evidence/. Same20s/2GiB caps;
original crashes/unknowns are preserved. Incidentally printed clocks are raw
backend output, not admitted timing evidence.

*Reproduce:* build.sh, README.md, PROTOCOL.md. Pins and theorem boundaries:
PROOF-MAP.md,proof-dependencies.json. Shared library PR7; parent prototype
crypto PR4234. Sources checked: original norm continuation and initial-elimination
question, prior chart/reducedness reasoning, Mathlib monic division laws and
pinned msolve help. Standard preprocessing; no novelty or exhaustive review claim.
]
