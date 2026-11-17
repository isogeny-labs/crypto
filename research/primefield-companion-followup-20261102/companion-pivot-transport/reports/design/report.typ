#set page(paper:"a4",margin:17mm)
#set text(size:10.5pt)
= Same-field pivot transport
2026-11-01 · Derived identity and syntax-checked source; zero numerical trials

*Exact construction.* Given Z_a A − A Z_b = G Kᵗ, let u=e_i−e_j and
P=I−u uᵗ be a row transposition. Then B=PA has generators

```text
GB = [P G | u | -Z_a u]
KB = [K   | A^t Z_a^t u | A^t u]
```

The identity follows from Z_a P−P Z_a = u uᵗZ_a−Z_a u uᵗ.
Constructed width grows by two. Two charged transpose source queries
obtain the added right columns. The base prime and wraps stay unchanged.
An identity swap makes no source queries. Outside the query ledgers, the
implementation charges two field products and N+1 subtractions.

#image("pipeline.svg",width:100%)

*Pivot guarantee at one position.* Every nonsingular A has a nonzero first
column. Its first nonzero row supplies a nonzero scalar pivot after a swap,
with no randomized field-size condition. Total queries: one when the top
entry is already nonzero; three for a nontrivial swap. An all-zero first
column yields the actual right-null vector e0. It does not establish RHS
inconsistency or produce a left-null exclusion witness. Scan length is an
explicit native counter. The diagram's zero-column case terminates; only
the nonzero branch proceeds to row swapping.

*Certificate transport.* Ax=b is equivalent to (PA)x=P b. Solution and
right-null coordinates stay unchanged. A left-null vector w for PA maps
to Pᵗw for A, preserving (Pᵗw)ᵗb=wᵗ(Pb). Feature-nullness needs its own
check. The invertible swap matrix with zero leading scalar is a required
control: this pivot maps it to the identity.

*Cost dependency.* One scalar pivot does not repair an arbitrary balanced
leading block. Repeated row pivots plus scalar Schur reduction can grow
constructed width to s+4d after d levels. This helper is not inserted as
an uncharged fallback into the balanced recursive inverse. Rank-revealing
block selection or controlled generator compression is the next construction.
Audit histories, copies and temporary encodings count towards memory.

*Validation next.* Freeze all declared transpositions, both query directions,
zero and later pivots, boundary rows, wrap guards, corrupted correction signs,
RHS and witness transport, and exact query/scalar/scan counters. No trials
precede the full upstream validation gates and a committed protocol.

*Files and prior art.* pivot.py and DERIVATION.md. Bostan et al. (2017),
Section 6, DOI 10.1137/16M1062855, motivates explicit rank-profile handling;
its randomized regularization guarantee is not assumed here. This is a
derived pivot primitive, not a measured speed result.
