---
title: "Original E0: constructed-kernel conformance"
date: "27 October 2026"
geometry: "margin=19mm"
papersize: a4
fontsize: 10pt
colorlinks: true
header-includes:
  - '\usepackage{float}'
  - '\usepackage{graphicx}'
---

The native query helper now constructs the full cofactor kernel for the
original public E0 fixture when supplied with points from both trace cosets.
This is a source-conformance result under the fixture's previously recorded
ambient order. It does not supply an IC cost or an unknown logarithm.

**Source:** local component `44b65a769533`, linked against the verified
`974f2e9cd46b` library. **Native development gate:** 15 tests pass; ordinary
Cargo integration and full-library/hosted results are separate.

## Fixed fixture and input control

The model is `EC1N22Cbinhd39a9df391db`, over the polynomial-basis field with
modulus 4,194,307. Its equation is $y^2+xy=x^3+1$. Existing metadata supplies
$\#E=4,193,912$, $r=991$, $h=4,232$ and $G=(97,814,2,630,359)$.
This check does not repeat or replace the earlier curve-order audit.

The first positive fixture supplied independent reference lifts at $x=0,\ldots,63$.
It generated only 2,116 kernel points, and the helper correctly refused it.
The failed native attempt is preserved: build exit 0, test exit 101,
14 passes and one failure. The corrected test retains that proper span as a
negative control, then adds reference lifts at
$x=\mathrm{trace\_one}\mathbin{\mathrm{XOR}}i$, for $i=0,\ldots,63$.
Here trace_one is the first polynomial-basis element of absolute trace one.

\begin{figure}[H]
\centering
\includegraphics[width=\linewidth]{kernel.pdf}
\caption{Deterministic supplied-point construction and independent checks.
Counts are correctness checks, not runtime or yield measurements.}
\end{figure}

## Verified scope

| Check | Recorded outcome |
|:---|:---|
| Complete supplied-point span | 4,232 distinct kernel points |
| Independent point arithmetic | Every point is on E0 and satisfies $[4,232]P=O$ |
| Query/recovery conformance | 36 fixed known-target cases agree |
| Combined native test executable | 15 passed, zero failed |
| Original failed candidate set | Explicit incomplete-span control retained |

Construction uses standard projection by $[r]$, successive cosets and a final
census check. Candidate discovery and random sampling belong to the caller.
The logs used by the recovery checks are supplied from known synthetic scalars;
no factor-base relation or unknown logarithm is discovered. Setup and checks
remain chargeable backend calls. Relation collection, logarithm/linear-algebra
integration, sampling/yield, complete cold price and the general Lean counting
bridge remain open. No crossover or whole-pipeline gain follows.

**Evidence:** the [first attempt](../../pipeline-receipts/projected-kernel-e0-development-20261027/README.md)
and [corrected attempt](../../pipeline-receipts/projected-kernel-e0-development-20261027-attempt-2/README.md)
retain commands, source/library hashes, patches and raw logs. Final build/test
exits are 0/0. This report supplies no new timing result.
