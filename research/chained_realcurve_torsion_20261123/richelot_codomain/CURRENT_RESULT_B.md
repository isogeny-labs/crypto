# Actual prime-field Richelot endpoints and retained map exceptions

Unsigned PCC-20261123-c656d1 records first native B at source
0b292644956ff0e5ef2350721abb32245166c609. All seven strict phases and all
source/tool/immutable-qualifier guards pass, unchanged2GiB/120s/one-worker.
The first A strict-format failure and original source remain preserved.
[pcc:unverified]

The selected original C/F3:y^2=x^6+x^5+x^2+1 has6 actual roots in F27.
All3 rational Richelot kernels have their3 nonzero points permuted in a
cycle; none is pointwise rational. The actual endpoints are:

| Kernel id | delta | Endpoint |
| --- | ---: | --- |
| 0 | 2 | y^2=x^6+x^5+x^3+x^2+1 |
| 1 | 0 | Product endpoint; no smooth genus-two model asserted |
| 2 | 1 | y^2=2x^6+x^5+2x^3+x^2+1 |

Both smooth models are squarefree over F3, have N1=6,N2=12 and the same
Weil polynomial as the source. Kernel2's nonsquare leading coefficient2
is preserved: it has0 rational infinities over F3 and2 over F9. Wrong-sign
delta gives t=2 rather than-2 and is rejected for each endpoint.
[pcc:unverified]

All3 exact bivariate Richelot identities pass. Direct enumeration gives36
source points over F27, including6 affine branch points and2 infinities.
For the complete28 non-branch affine points per smooth endpoint,52
degree2 image ideals satisfy the actual codomain equation. Every copied
ordinate mutant omitting x-z is rejected on those52 ideals; the restored
rule passes. [pcc:unverified]

The remaining4 cases are **retained degree drops**, all at kernel0:
x=20 or25 in the recorded F27 encoding, each with y=1 or2. Their image
relation is linear; treating its affine remainder as the entire image would
lose a codomain infinity. Branch points are separate6-per-endpoint exceptions
to the formula dividing by y. This is a partial curve correspondence, not a
total Jacobian homomorphism or unknown-scalar ECDLP transfer. [pcc:unverified]

Whole6283B native stdout, SHA256
3599c111cfbf05aac947e7c8a36e229c4854d89e2e21af7f17520701724892b9,
retains all roots/factors/brackets, endpoint fibres, every point ideal and
every exception/control. Whole4647016B ELF, SHA256
274e9aaac64b5f65a5e7b363faaa8b941c80d6f3b4c2faa1d4b0e67bbc39adbb,
is private with remote custody pending. No external validator, Lean theorem,
signed verdict or computational advantage is claimed. [pcc:unverified]

Next: test the explicit elliptic cover in ELLIPTIC_PULLBACK_CANDIDATE.md and
the homogeneous point-image amendment, retaining these4 degree-drop witnesses
and original-source infinities as mandatory controls. Do not rerun the
unchanged census/J8/codomain population. [pcc:unverified]
