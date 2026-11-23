# Checked general companion block interfaces

`operators.py` supports arbitrary dimension n over the prime coefficient
field F_r and distinct canonical wraps a,b, including zero wraps. It applies
A or A^t from Z_(n,a)A-AZ_(n,b)=GK^t without forming dense A.

The completed ../companion-blocks-validation/ panel contains738contextrows,
32328productchecks,3072principalidentities,192inverseidentities,384singular
oracle rejections,2376equal-wraprejections and67920corruptioncontrols, with
exactAA. Its full export and post-run record reconciliation passed. The
predata seed-cell audit verified all738cells; the previous matvec panel's
seed-exportfailure is preserved unchanged.

Principal construction uses four explicit boundary corrections and a counted
public-query cache. Inverse-generator transport consumes supplied solve
oracles; the checks used explicit reference inverses. This does not implement
fast inverse synthesis. The complete ledgers distinguish query arithmetic,
boundary acquisition, correction arithmetic, retained field slots and oracle
calls. No runtime or RSS improvement is inferred.

DERIVATION.md is the frozen predata mathematical description; its initial
unexecuted status is superseded by this result record. SCHUR_NEXT.md derives
the next width-s+2 Schur construction. ../companion-schur/schur.py implements
that next interface as a syntax-checked, numerically unexecuted prototype.
See ../companion-blocks-validation/reports/final/report.pdf for the three-page
report, exact costs, custody, and separation of executed and derived work.
