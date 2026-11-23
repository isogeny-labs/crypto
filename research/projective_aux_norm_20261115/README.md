# Auxiliary projective norm oracle and cold recovery

The auxiliary tower now supplies a complete fixed prime-field recovery over
E/F19:y²=x³+2. Given P=(4,3), Q=(18,1), the candidate recovers 11 through
algebraic factor-base construction, point-only norm queries, four signed
relations and a kernel. Independent OpenSSL replay checks the queries, rows,
kernel and [11]P=Q. This is a tiny software control, not a general
subexponential algorithm or a performance result. PCC-20261115-1f50be remains
unverified. [pcc:unverified]

The preset auxiliary A/F19:y²=x³+2x+2 has T=(16,11) of exact order 8 and
coset point G=(12,5). Three checked Vélu steps have parameters
(xi,tau)=(14,1),(16,5),(17,3), with terminal value 15 and exactly 8 abscissas
{1,3,4,7,8,11,12,17}. Neither p-1 nor p+1 is divisible by 8. The auxiliary
construction already appears in the parent PKM notes; this is an implemented
encoding/control, not a new factor-base invention. [pcc:unverified]

Projective coordinates retain N=P²-xiPQ+tauQ² and D=PQ-xiQ² modulo monic
support U. The shared CommRing law preserves IsCoprime(P,Q) when tau is a
unit. At a finite terminal N=cD, existing Mathlib coprimality/divisibility
makes D a unit, so separate inverse witnesses are redundant. Source
3fc498f77743426e1dc0a3c855d7794b3f20af20 in
[crypto-lean PR8](https://github.com/aburan28/crypto-lean/pull/8) passes the
102-theorem local gate. The pinned AdjoinRoot(F19) instance passes its two-
theorem gate, with standard axioms only. Native source binding and program
correctness are not proved by those statements. [pcc:unverified]

| Component | Fixed outcome |
|---|---|
| Projective norm encoding |14 variables, maximum degree 3 |
| Public first-wrap graph elimination |8 variables, degree 4, 6 cancelled equations |
| msolve, each presentation |12 complete sets match; 4 models total |
| Original PC D4 / early PC D4 |12 capped unknowns in each round |
| Projected early PC D6 |12 capped unknowns |
| Indexed projected PC D6 |6 checked unit ideals;6 step-cap unknowns |
| Small controls + Kummer D4 |All complete sets/dimensions match |
| Auxiliary cold |Scalar11;13 attempts,12 solver calls,4 relations,1 kernel |
| Default F17 regression |18 targets,72 relations,18 kernels; 54 trace files byte-identical |

All native return models satisfy every source equation, lift to the original
14-variable system and pass curve, membership and group-sum checks. The
independent OpenSSL model replayer enumerates the fixed finite reference.
The cold collector checks witnesses and the final scalar; it need not trust
an unverified source-ideal assertion by msolve. Source-certified PC still
supplies no positive auxiliary decomposition at these caps. Its unknowns are
not called empty ideals or degree lower bounds. [pcc:unverified]

The new indexed producer uses the same grevlex monomials, FIFO closure and
early-completion criteria, retaining a PC-DAG-CERT-1 derivation. Its verifier
is the unchanged separately compiled symbolic checker. Integer leading-column
lookup and sparse pivots replace repeated symbolic scans. A small multiplication
table is restricted to p<=257; index/work and coefficient/proof entries share
the same 2M cap. Six auxiliary unit ideals pass full source, completion and
original-inclusion checks; the other six hit 1M reductions at about 2295 pivots
and 576k entries. These are partial native counters, not total cost or timing.
[pcc:unverified]

Cold setup validates the preset A,T,G, constructs its coset and lifts main
points using native polynomial roots. Main order 13 follows from prime 13,
13P=O, Hasse[12,28], and gcd(X^19-X,X³+2)=1 excluding 2-torsion. No field
census, supplied base logs, known target scalar or norm assignment enters the
candidate. Independent reference censuses stay outside it. General auxiliary
curve search and large-prime arithmetic are not implemented. [pcc:unverified]

Builds and frozen orchestration scripts are separate by round:

```sh
research/projective_aux_norm_20261115/build.sh PREFIX NEW_AUX_BUILD
research/projective_aux_norm_20261115/build-indexed.sh PREFIX NEW_INDEX_BUILD
research/projective_aux_norm_20261115/build-cold.sh PREFIX NEW_COLD_BUILD
```

PREFIX is the pinned msolve0.7.5/FLINT3.1.3 installation documented in the
parent shared_norm study. All research computation is C++17 with strict
warnings; shell scripts only orchestrate native tools. PROTOCOL.md freezes
each panel and amendment. Qualifiers require fresh paths. Copy a qualifier
to a separate execution file before invoking it; do not edit that copy while
bash is paused in a child. Each native backend has20 s CPU/wall and2 GiB;
degree/row/node/entry/step/proof/quotient caps remain explicit. No cap is
raised to repair a failed call. [pcc:unverified]

Raw rounds live under evidence/{qualification001,loader002,early003,graph004,
indexed005,cold006}/runs.tar.xz, each with a directly checkable SHA256 file.
Check with `sha256sum -c runs.tar.xz.sha256`, then extract using
`tar -xJf runs.tar.xz` in that directory. Archives use normalized tar/xz
metadata and retain every input, candidate, certificate, exit and failure.
Artifact-autofix run 43704942686 reports one uploaded and completely replayed
688992-byte indexed archive. Commit 379576ac63 installs its manifest/ignore
and removes the tracked raw file; SHA256 is
8b8cb99d428ef8825d4522d2ed3583673605bfb5c8948b83b417e5ac6b6b5f38. Hydrate it with `python3 tools/artifacts/artifacts.py pull
research/projective_aux_norm_20261115/evidence/indexed005` after migration.
All other whole rounds are naturally below 64 KiB. No archive is split.
[pcc:unverified]

Custody/history: first msolve launches fail127 because libneogb is missing from
the loader path. A1 repeats only that arm with the pinned loader; all failures
stay retained. The original wrapper reports a final-trailer syntax error after
all cases/replays complete because its on-disk file was edited mid-execution;
the custody note and both per-arm exit0 receipts remain. Later wrappers use
frozen execution copies with hashes. Initial native/Lean build failures and
their actual fixes also remain. Shared hosted CI executes no steps because
the account is billing-locked; local proof gates pass. [pcc:unverified]

Source/call bindings:62c7042e9a initial panel;5db5ecf7f7 loader arm;
3fe2c55987 early completion;14f92aedbe projected graph;
8f9ecfff9a indexed producer;626a4e0f91 generalized cold recovery.
Full source and binary hashes precede each affected call in its raw round.
Historical source commits bind historical evidence, not later templates.

Sources checked (KB unavailable): [auxiliary extension](../prime_one_shot_20261029/continuation/theory/auxiliary-curve-extension.md),
its [primary counting review](../prime_one_shot_20261029/continuation/reviews/modular-curve-counting.md),
the [selective oracle contract](../prime_one_shot_20261029/continuation/theory/selective-oracle-contract.md)
and [initial elimination obligation](../prime_one_shot_20261029/continuation/theory/initial-elimination-obligation.md);
existing native isogeny_eval/small_degree and isogeny-vs-mu harness;
[prior PC component](../norm_pc_closure_20261115/README.md); actual Mathlib
Coprime/AdjoinRoot/ZMod declarations. The [proof map](PROOF-MAP.md) precedes
Lean edits. No novelty or exhaustive literature-review claim.

The [diagram](pipeline.svg) and [three-page report](report.pdf) describe this
component. No canonical leaderboard, progress timeline or performance-gains
value is affected: these are bounded software qualifications without an
admitted performance or security comparison. The raw controls remain scoped
to their exact fields, target census and source commits. [pcc:unverified]

The complete classical subexponential prime-field objective remains active.
The per-attempt monomial envelope alone gives no useful growing-family yield
or sufficient degree. The next setup candidate has a [source review](SETUP-REVIEW.md), without
new implementation or run. Further work includes generated auxiliary setup, arbitrary
prime arithmetic and an explicit degree/yield argument on an admissible family.
The tiny presets do not meet the parent theorem's large-size parameter bounds,
and there is no T1/T2/T3, fitted exponent or matched-rho claim here.
[pcc:unverified]

The full root Rust suite at source 2fd16de7ca finishes nonpassing: 6406 passed,
48 failed,139 ignored. Failure output reports omitted sparse data/device
sources in untouched tests. The earlier partial snapshot remains immutable;
the complete final log is retained by the generated-setup continuation.
No full root-suite pass is claimed. [pcc:unverified]
