#set page(paper: "a4", margin: 17mm, footer: align(right)[#context counter(page).display()])
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: 0.55em)
#set heading(numbering: none)
#let repo = "https://github.com/aburan28/crypto/blob/codex/nist-skipped-curve-jobs/research/nist_skipped_curve_dissection_20261021/factorization/"
#let local-link(path, label) = link(repo + path)[#label]

#text(size: 20pt, weight: "bold")[Continuing the NIST curve factorizations]

Run #raw("nist-factor-continuation-20261021T1424Z") · 21 October 2026

*Six of nine distinct integers are fully factored into proven primes.*
Three retain composite cofactors. This resolves *eleven of fourteen*
timeout-valued fields across seven of the ten affected rows in the earlier
438-case upstream DiSSECT run. The original outputs are preserved.

The fixed panel comprises P-224 stream blocks 777, 1485 and 5736, P-256
block 876, and published P-224/P-256 controls. Imported fixture keys and
exact hex parameters identify the models. No model with the opposite sign
of #raw("b") was substituted.

#image("factor-sizes.svg", width: 100%)

The graph plots exact bit lengths serialized by the native verifier:
one partition per fixed integer, with no sampling interval. A residual
cofactor is explicitly composite. It is not a proven prime factor.
Full decimals: #local-link("FACTORIZATIONS.md", "factorizations and cofactors").

Both P-256 #raw("4p-1") square-part roots and the skipped P-256
#raw("4n-1") root are 1. Resolved largest-factor sizes are 147 bits for
skipped #raw("4n-1"), 176 for published #raw("6n+1"), and 133 for
published #raw("7n-1"). These facts do not explain the historical skips.

#pagebreak()
= Inputs, executed search and controls

#image("evidence-flow.svg", width: 100%)

Let #raw("t=p+1-n"). Native PARI reconstructs the degree-2 extension-twist
orders #raw("(p-1)^2+t^2"), the degree-4 conductor-ratio factorization target
#raw("abs(t*(t^2-2*p))"), and the required #raw("kn±1") and #raw("4p-1")
integers directly from hex. The conductor-ratio term is not, by itself, an
endomorphism-ring conductor. Despite its name, the upstream
#raw("square_4p1") trait uses minus one for both inputs.

Two P-256 #raw("4p-1") requests share an integer; skipped #raw("4n-1")
duplicates another request. Three bit-length timeout fields duplicate their
factorization fields. Thus ten rows mean fourteen fields and nine integers.
The additive erratum preserves the initial protocol's eleven-field miscount.

PARI/GP 2.17.2 used trial division, exact algebraic partitioning, read-only
FactorDB hints and 600-second native factorization attempts. All hints were
checked by exact division. Six targets completed in this stage. GMP-ECM
7.0.6 then produced more factors for each remaining target lineage.

#set table(stroke: 0.3pt + luma(80%), inset: 4pt)
#text(size: 8pt)[
#table(columns: (1fr, auto, auto, auto),
  [Target lineage / stage], [B1], [Curves tried], [Stage 2 done],
  [Published P-224 / first split], [100000], [24], [24],
  [Published P-224 / remainder], [100000], [76], [76],
  [P-224 block 5736 / first split], [100000], [6], [6],
  [P-224 block 5736 / next split], [100000], [4], [4],
  [P-224 block 5736 / remainder], [100000], [90], [90],
  [P-256 block 876 / first split], [100000], [1], [0],
  [P-256 block 876 / remainder], [100000], [99], [99],
  [Each of three residuals], [1000000], [200 each], [200 each],
  [Each of three residuals], [3000000], [100 each], [100 each],
)
]

After a split, only the unused curve allocation continued on its smaller
cofactor. The final 300 curves found no new factor. Every curve parameter,
saved residue, command and exit is retained; the
#local-link("ecm-attempts.json", "attempt inventory") enumerates all 15
ECM subprocesses including controls. Nonzero even exits can encode factors.

The first random positive control reported its entire input, a trivial
result, and remains a failed control. The corrected deterministic control
at sigma #raw("1:1184114830") finds #raw("1000003") with cofactor
#raw("1000033"). The wrong-target replay control rejects its input.

At most two workers ran together. Each ECM cofactor attempt had a 600-second
limit and 1024 MB stage-2 memory cap. The unisolated Linux 6.18.55 x86-64
container had a four-CPU, 16 GiB cgroup limit. No remote machines ran.
This is a completion search, without performance or speedup claims.

#pagebreak()
= Verification, failures and remaining composites

Separate native replay reconstructs expected targets, validates every saved
PARI prime certificate using #raw("primecertisvalid"), separately calls
#raw("isprime"), checks positive exponents, composite residuals and exact
products. All nine partitions pass. It is a separate process using the same
PARI arithmetic implementation, not an independent implementation.

Lean 4.35.0 proves all nine exact partition products over natural numbers
with #raw("by decide"), including the three explicit composite cofactors.
The repository gate audits nine theorems, zero failing and zero trusted
native proofs; every axiom list is empty. *Lean proves products, not
primality.* Native PARI certificates establish the separate primality claims.
#raw("lean4checker") was unavailable. The
#local-link("proof-map.md", "proof map") names every theorem and identity record.

The first PARI wrapper falsely rejected a harmless stack warning. Three
native searches hit their 600-second alarms and retained partial partitions.
One certificate-generation attempt overflowed the default 8 MB stack;
the fresh 128 MB initial / 2 GB maximum-stack attempt passed. A Lean 4.25
build proved five products but failed in the incompatible axiom auditor;
4.35.0 passes. The initial constant-expression encoding exceeded the JSON
nesting limit; the corrected native decimal-chunk encoding passes separate
issue/check invocations. Failed evidence is preserved.

The required library suite reported 4465 passed, 9 failed and 119 ignored
when loopback networking was denied. All nine failures were in
#raw("pollard_collab"). Rerunning that whole module with network permission
passed 44/44 tests, covering every failure. This is a full-suite run plus a
targeted retry, not a newly observed single all-green full-suite run.
Shell syntax, identity-record checks and all nine factor replays pass.

== Unresolved cofactors

Each decimal below is composite. Spaces/line breaks are for layout only;
the exact strings and proven prime factors are in
#local-link("results.json", "results.json").

*P-224 stream block 5736 — 327 bits*
#raw("19106509893925638590876479307257909680692826\n55793904142972730384691966042890337782660059\n78480168453", block: true)

*Published P-224 — 364 bits*
#raw("26824805404542206456882676187978690075366455\n86954534173696932703925808640719391831005646\n0278604794052610896253", block: true)

*P-256 stream block 876 — 444 bits*
#raw("33492266050798185653682380062483799213318454\n80165396349313406472105973593694452090729593\n14966144558580760484371844990904784202048883\n53", block: true)

No smallest-prime-factor lower bound follows from exhausted ECM allocations.
Complete prime-certified refinements would close this factorization gap.
The forty original generator-dependent null cases and upstream torsion and
degree-2 trace defects remain separate. No historical skip reason or new
discrete-logarithm attack is established.

#pagebreak()
= Replay and evidence sources

From the study's #raw("factorization/") directory, with PARI/GP 2.17.2:

```bash
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
bash replay_results.sh runs /tmp/nist-factor-replay-01
```

Choose a fresh replay directory. Set #raw("GP_BIN") to a single executable
path if needed. New JSON encodes large integers as decimal strings. Do not
round-trip them through JSON numbers.

The deterministic archive contains all new attempts, failures, parameters,
residues, raw hint responses, intermediate partitions, final certificates
and verification logs. Original absolute paths record provenance; the replay
builds fresh descriptors for the extracted location. Repacking instructions
and native identity-check build commands are in the
#local-link("README.md", "study README").

The unchanged external #raw("inputs/upstream-dissect.zip") has 932 files:
source, 438 cases, logs and audit. Pinned upstream revision:
#raw("8edc3030fb91411a55aa6c07d050507d840c5e55").
The fixture matches the external bundle exactly on the six models'
#raw("p,a,b,n,h"). Input and executable hashes are in
#local-link("provenance.json", "provenance.json"). The manual workflow was
not dispatched. Conductor was unavailable; the documented fallback was used.

== Formal product map

#text(size: 8pt)[
#table(columns: (1fr, 1fr),
  [Target], [Identity certificate],
  [Shared P-256 4p−1], [#raw("IDC1h31218b677856b8b9")],
  [Published P-224 twist₂], [#raw("IDC1hdb2af39b925576a1")],
  [Published P-256 6n+1], [#raw("IDC1hdecb8fce71e2eab7")],
  [Published P-256 7n−1], [#raw("IDC1h512190b8fcf5d7fe")],
  [Published P-256 twist₂], [#raw("IDC1h969e7da713aca14e")],
  [P-224 block 1485 conductor₄ term], [#raw("IDC1h7c5ecb55bf5e2915")],
  [P-224 block 5736 twist₂], [#raw("IDC1he5e12c8967667144")],
  [P-256 block 876 4n−1], [#raw("IDC1h0ba14f42c133d8da")],
  [P-256 block 876 twist₂], [#raw("IDC1hb6ab9163ca2cf443")],
)
]

The constant-identity records evaluate over #raw("2^61-1") and link the
exact Lean source digest. Modular equality alone would not prove integer
equality; the linked Lean kernel proofs establish the exact products.

== Sources and scope

- #link("https://www.anthropic.com/research/ecdsa-curve-origins")[Anthropic: ECDSA curve origins]
- #link("https://words.filippo.io/seeds-solved/")[Filippo Valsorda: Seeds solved]
- #link("https://github.com/crocs-muni/DiSSECT/tree/8edc3030fb91411a55aa6c07d050507d840c5e55")[Pinned DiSSECT source: square_4p1, twist_order, conductor and kn_factorization]
- #link("https://pari.math.u-bordeaux.fr/dochtml/html-stable/Arithmetic_functions.html#primecert")[PARI prime certificates]
- #link("https://gitlab.inria.fr/zimmerma/ecm")[GMP-ECM]
- #link("https://github.com/aburan28/crypto/pull/2379")[PR #2379: jobs and continuation evidence]

No existing canonical graph tracks these factorization entries. IC
scoreboards/timeline, curve/performance figures and seed-study visuals remain
unchanged. The new graph and rendered copies use only recorded native values;
plotting Python performs layout. The remaining composite-cofactor problem is
linked from the study, together with exact acceptance conditions.
