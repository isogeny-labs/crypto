#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Sparse quotient-core kernel
Genus 3 · F127 · 15 November 2026
#image("flow.svg",width:100%)

Two separately frozen correctness gates passed once. The reduced-core solver now uses sparse scalar Lanczos, adapted from repository N4v2, with exact verification on the original matrix. No dense fallback is used by the candidate.

== Standalone gate
Source e5179bfd111d; checkpoint 20261116T002935Z. The fresh232×47matrix has2413nonzero entries. Attempt0returned a nonzero exact kernel. All129point logs match the independent dense reference up to one common nonzero scale; all314original relation residuals pass. Four seeds each at dimensions47,128,512 pass twelve planted controls. Zero vectors and corrupted original matrices reject. No breakdown occurred; this is not a failure-rate estimate.

== Full cold integration
Source8eadddd88e0d; checkpoint20261116T003102Z. All fresh setup, discovered chart, alignment, inverse and conic stages pass. The sparse core succeeds at attempt0. The D1/B/calibrator logs are556422/314822/383403; their scale changes while the ratio yields k183049. Four/two conic descents agree; public scalar multiplication verifies on source and destination, while k+1rejects. All positive stages exit0, expected missing-loader101. No scientific rerun or omitted input.

== Mathematical safeguard and cost case
Over finite fields, a kernel of AᵀDA can contain vectors outside the kernel of A. Every result must therefore be nonzero and pass Ax=0 on original sparse rows. Retries are bounded at32, with n+5iterations each. Reduced-step storage is O(nnz+rows+columns); universal termination is not claimed.

Diem's published expected soft-O(q) plane-quartic algorithm offers an exponent advantage over the known hyperelliptic genus3 soft-O(q^(4/3)) algorithm. For constants Ch,Cd and full setup/transfer cost S(q), a conditional tenfold budget is S(q)+Cd·q ≤ Ch·q^(4/3)/10, omitting logarithmic factors. This is not a measured crossover. An unchanged positive q²setup term prevents an eventual advantage under that budget.

== Remaining work
Full relation assembly still uses dense rows and fixed48base points. Field parameterization, efficient construction and verification, applicable rational kernels, larger-field charged comparisons and genus two/four remain. Known algorithms and this engineering integration do not establish a novel tenfold mathematical speedup. The previous failed cold performance comparison remains unchanged.

Raw output, protocols, source, lockfile and hashes accompany RESULTS.md. Primary sources: Diem, jtnb.centre-mersenne.org/articles/10.5802/jtnb.815/; GTTD, eprint.iacr.org/2004/153.pdf.
