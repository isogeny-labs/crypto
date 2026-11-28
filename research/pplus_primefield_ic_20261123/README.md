# Prime-field p+1 index-calculus route

This study identifies the norm-one-torus Dickson selector as a concrete
instantiation of prime-field PKM/Semaev index calculus, qualifies the existing
production selector, and implements a binary line FFT for its full unfiltered
quotient algebra. Efficient elliptic relation extraction remains an open
algorithmic dependency. [pcc:unverified]

## Read first

- [Research report](REPORT.md) and [illustrated PDF](Pplus_PrimeField_ECDLP_Research.pdf).
- [Same-team mathematical and source review](REVIEW.md).
- [Portable source/evidence instructions](PUBLICATION_README.md) and
  [byte-copy manifest](PUBLICATION_MANIFEST.json).
- [Selector qualification](selector/README.md),
  [line-FFT component](fft/README.md), and
  [finite identity-evaluation records](fft/identity/README.md).
- [Explanatory figures and rendering sources](figures/README.md).

The report uses the pinned ordinary curve y^2=x^3-3x+21 over p=2^31-1.
It separates selector size, rational x/negation-orbit columns, and signed
points. Transform complexity assumes the native hierarchical basis and
precomputed domains/twiddles. Reduced quotient multiplication is distinguished
from unreduced multiplication and from elliptic relation finding.

## Evidence standing

| Record | Scope | Standing |
| --- | --- | --- |
| PCC-20261123-7ceb9d | Finite native selector, FFT and identity-record observations | Unverified |
| PCC-20261123-ab6e6e | Source-linked construction, derivations and analysis | Unverified |

Both standings were read with the repository PCC tool. The ledger chain passed
its structural check; neither claim has a signed verdict. The model review,
source hashes, process separation and numerical controls do not replace that
verdict. [pcc:unverified]

The native programs report the specified successful checks and deliberate
rejections. They do not report timings, complete ECDLP recovery, useful
relation yield or a comparative speed improvement. [pcc:unverified]

## Reproducibility and publication notes

`render_report.py` and `figures/render_figures.py` perform document/figure
rendering only. Research arithmetic and accepted result generation are Rust.
The nine-page PDF was rendered to PNG and every page visually inspected;
tables, figures, equations, source links and page flow were reviewed. This is
document QA, not validation of the scientific claims.

The accepted evidence is compact and included. The rejected first selector
execution path is disclosed in [ATTEMPT01_CUSTODY.md](ATTEMPT01_CUSTODY.md).
Its original oversized log remains locally retained; object-store archival is
pending. The runtime has no `aws` CLI, and the repository's `docs/artifacts.md`
also records a store-recovery dependency. No successful remote archival is
asserted, and the oversized log was neither renamed nor truncated.

Existing competitive IC charts remain unchanged because this work supplies
no competitive cost result. The next extraction obligation is the existing
[`prime31-exact-pair-spectrum-query` problem](../../problems/prime31-exact-pair-spectrum-query.md).
