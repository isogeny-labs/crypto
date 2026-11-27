---
title: "P-192 frozen tangents: excluding minimal three-row closure"
date: "2026-11-24"
geometry: margin=0.7in
fontsize: 10pt
---

All three necessary-query families finish with no matches. Exact support
enumeration covers every minimal three-row pattern allowed by the recorded
two-residual-image construction, including signed triangles, paths and
repeated projected images. Together with the preceding one/two-row exclusion,
this conditionally raises the minimum support for any additional old-base
relation to **four selected tangent rows**. It concerns only the fixed batch;
independent signing and formal coverage proof remain open. [pcc:unverified]

## Why this is the next hypothesis

The existing P-192 d7/kernel3 plane quartic has a recorded degree6 elliptic
map f. For each selected tangent, $H_i=2P_i+R_i+S_i$ and the residual trace is
$T_i=f(R_i)+f(S_i)=\kappa-2f(P_i)$. The old alphabet A contains O and both
signs of each original projected base point. Anchor and reference images
already lie on old columns. The [parent audit](../two_row_20261124/REPORT.md)
excludes closure using one or two selected rows. H-L tests the minimum
remaining size, rather than expanding the quartic point window.
[pcc:unverified]

A row's nonzero new-column vector, up to overall sign, has type A: $e_j$
with one old residual q in A; type B: $2e_j$ with no old residual; or type C:
$e_j\pm e_k$ with no old residual. An identity residual image belongs to the
first type with q=O. Opposite residual images give a zero new part and are
excluded by the parent's single-row premise. [pcc:unverified]

In a minimal three-row dependency every coefficient is nonzero and no column
can occur in just one row. The six residual slots therefore use at most three
new columns. On those columns there are 12 vectors of the stated forms after
normalizing the first nonzero sign. The 220 triples of distinct vectors cover
all cases up to row/column relabeling; a repeated or proportional pair is
already excluded. Exact integer minors and modular Gaussian ranks inspect the
entire finite domain and verify every retained primitive kernel equation.
The characteristic exceeds 32, while the relevant determinants are bounded
by 16 in absolute value, so this domain has no characteristic collapse.
[pcc:unverified]

| Classification | Recorded triples |
|---|---:|
| Contains a proportional pair | 30 |
| Full rank | 150 |
| Minimal dependency U: magnitudes (1,1,1) | 16 |
| Minimal dependency V: magnitudes (2,1,1) | 12 |
| Minimal dependency W: magnitudes (1,2,2) | 12 |

The retained 40 cases meet the proposed old-residual weight restrictions.
U contributes at most two signed old symbols; V/W contribute either zero
or twice one old symbol. Nine structural controls pass, including signed
versus unsigned triangles, paths, repeated-image patterns, a changed kernel
coefficient and an unsupported old-weight pattern. Complete raw vectors,
coefficients and types are retained in
[templates/result.json](evidence/qualification-v1/templates/result.json).
PCC-20261124-2129c1 records this producer-unverified classification.
[pcc:unverified]

The editable [support-classification diagram](supports.svg) records this finite
partition; these are algebraic catalog cases, not samples from curves.

\clearpage

## Recorded trace queries

The [protocol](PROTOCOL.md), native sources and guards were published at
345ec56c03 before any native call. Every arm binds the same baseline, tangent
records, selected indices, old pair fingerprint and parent exclusion result.
It rederives reference and traces and checks the one-row premise. No residual
root or new quartic point is reconstructed. [pcc:unverified]

| Arm | Necessary condition | Queries | Witnesses |
|---|---|---:|---:|
| U | $T_i\pm T_j\pm T_k\in A+A$ | 686800 | 0 |
| V | $2T_i\pm T_j\pm T_k\in 2A$ | 2060400 | 0 |
| W | $T_i\pm2T_j\pm2T_k\in 2A$ | 2060400 | 0 |

U uses unordered distinct triples with the first sign positive. V/W distinguish
the singly weighted row and use unordered remaining pairs with both signs.
Symmetry of A+A and 2A covers global negatives. This is the complete frozen
domain, not a sample or a favorable prefix. [pcc:unverified]

Every query key agrees under GMP and OpenSSL arithmetic, and complete grouped
and independently sorted joins agree on all witness sets. Each arm has ten
positive/mutation controls and a complete streamed query fingerprint. The
common precomputation covers every signed pair of selected traces and its
double with both backends; its fingerprint matches across arms. Raw
[U](evidence/qualification-v1/u.json),
[V](evidence/qualification-v1/v.json) and
[W](evidence/qualification-v1/w.json) results and flushed per-anchor stdout
progress are retained. PCC-20261124-1b9035 pins source, protocol, raw results
and prior premises. Separate arithmetic does not supply an independently
signed verdict. [pcc:unverified]

![Complete finite query counts by arm. Each arm records zero matching identities. These are deterministic counts with no sampling interval; they are not performance or relation-yield measurements.](queries.pdf){width=78%}

The primitive scalar minor-cancellation record IDC1h3448bb57e5ee2135 uses 32
evaluations over $2^{61}-1$ and changed-constant rejection. It supports the
generic bookkeeping identity only. The finite support argument and geometric
premises are written producer evidence, not formally verified by that record.
The reused issuer's source field still points at its historical parent report;
the exact new issuer input is bound by this audit's source and primary claim.
[pcc:unverified]

## What changes in the actual collection

Under the recorded two-point geometry and parent exclusion, there is no
minimal three-row closure: every possible class has a necessary identity,
and none matches. Original baseline rows cannot alter this obstruction because
their new-column part is zero. Thus any additional relation supported on the
original base needs at least four selected tangent rows. [pcc:unverified]

This excludes three-row instances of the proposed signed-cycle and old-endpoint
path mechanisms in this batch. Larger cycles, linked paths and more general
combinations remain open. A future collector's elimination witness involving
fewer than four nonzero added-row coefficients would contradict these premises
or this audit and must be investigated before counting rank gain. A four-row
witness must still pass target principality, individual-image transport,
elliptic sums and independence checks; size alone supplies no relation.

\clearpage

## Costs and reproducibility

Run `bash research/nist_cover_three_tests_20261116/three_row_20261124/check.sh
evidence/<fresh-name>`. It creates the missing evidence parent, strictly builds
native C++17, runs structural coverage first and executes all three planned
arms. A failed query arm does not silently suppress the other planned arms.
No arm failed or timed out in this qualification. Source, input, binary,
environment, process stdout/stderr, exit and hash manifests are retained.
[pcc:unverified]

| Native stage | Exit | Wall diagnostic, seconds |
|---|---:|---:|
| Strict build | 0 | 8.864 |
| Structural enumeration and scalar record | 0 | 0.007 |
| U, including initialization/precomputation | 0 | 10.871 |
| V, including initialization/precomputation | 0 | 25.993 |
| W, including initialization/precomputation | 0 | 25.322 |

The two additional CLI calls reject an existing structural output and an
unknown mode with expected exit 1. Existing output bytes remain unchanged.
Their wall/user/system records are retained in
[accounting.json](evidence/accounting.json), along with all listed native
stages; PCC-20261124-f559d5 binds this diagnostic accounting. First strict
build, structural worker and all query arms pass. A separate query-entry
collision and missing-argument control also reject before arithmetic, preserving
all arm/classification bytes (PCC-20261124-8c50c8;
[evidence](evidence/query-guards-v1/unchanged.stdout)). Their own cost records
are retained beside that evidence. No failed attempt or wider-cap retry is
omitted. [pcc:unverified]

Build cap 300 seconds/1GiB, structural cap 30 seconds/1GiB and each arm
cap 120 seconds/1GiB. These local clocks are development diagnostics, not an
admitted benchmark. Metadata/staging outside native calls, report rendering,
publication, original map synthesis/setup, actual collection and independent
signing are not priced by these numbers. There is no complete-cost or speed
comparison claim. Actual peak RSS was not measured. [pcc:unverified]

## Sources and remaining requirements

Reviewed exact two_row and closure reports/protocols/native sources, frozen
signed_words arithmetic and the official
[Magma index-calculus documentation](https://docs.magma-maths.org/v2.30-1/AlgebraicGeometry/AlgebraicCurves/linear-equivalence.html#index-calculus).
Its relation-matrix approach motivates checking coefficient dependencies;
the present work is an elementary necessary-condition audit, not a new index
calculus algorithm. KB unavailable: no loaded tool and known wrapper has no
executable. No novelty, general curve-family separator or field-size scaling
claim follows. Smith's random-genus3 population estimate does not assign a
probability to this fixed elliptic-cover family.

The broader requested pipeline remains open: the prepared admitted collector
must reconstruct individual residuals, verify actual target principal classes
and maps, eliminate every new column and check old-base rank and complete
costs. No new quartic point, residual root or production collector call occurs
in this audit. Independent signing and original setup accounting remain open.

The [isolab-eks skill](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/isolab-eks/SKILL.md)
permits local development/static checks and requires: “Stop new launches when
the budget is exhausted or its remaining balance is unknown.” That admission
boundary remains in force. No budget/IAM change, paid launch or repeated denied
credential probe was attempted. Prior canonical plots, reports and proof
subjects remain byte-equal; new figures describe only this finite audit.
