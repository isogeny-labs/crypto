# One-pass affine consequences, v1

The Rust compiler appends certified affine consequences and substituted quadratic
residuals while retaining every original variable and equation. A separate
BigUint coefficient checker reconstructs the admitted output from the sealed
public request and certificate. Native claim **PCC-20261116-313284** and corrected
thin-shell interface claim **PCC-20261116-503900** remain unsigned/unverified.
[pcc:unverified]

On the 28 fixed curve requests, before/after outcome classes and search-node
counts are unchanged. The 13 original toy requests remain Unsupported; the 13
indexed toy and two distinct actual-q1 requests return original-checked and
indexed-row/group-replayed roots. This is a null result for branch counts of
this static pass on these inputs, not a cost comparison or rejection of dynamic
or nonlinear methods. [pcc:unverified]

The two huge actual-field inputs receive coefficient certificates only: original
1059 variables/1089 equations becomes 1174 equations with 37 pivots; indexed
1989/2223 becomes 2548 equations with 157 pivots. Every original variable and
equation stays, degree is two, and neither input invokes a solver. [pcc:unverified]

- [Protocol](PROTOCOL.md), [frozen identity](FROZEN.md), [reproduction](REPRODUCE.md)
- [Report and figures](../reports/affine-closure-v1/REPORT.md)
- [Next discriminating work](NEXT.md)
- [Interface launcher provenance](INTERFACE_PROVENANCE.md)
- [Artifact publication status](PUBLISHING.md)

Classical linear elimination and telescoping polynomial identities are used;
no new generic method or signed/formal compiler correctness result is claimed.
