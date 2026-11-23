# Nonlinear coefficient-domain constructor

lib.rs constructs square-free cubic character identities from r and q in
F_r[X]/(X^r-X), with explicit allocation limits and caller-certified primality.
The r479/q16 packet in program.tsv has 478 nonzero terms. The q15 control is
inconsistent. check.rs independently certifies the saved polynomial identity
and rejects negative-program.tsv. Receipts preserve all bounded native commands.
See DERIVATION.md for exact scope/costs and NEXT_RECURRENCE.md for the proposed
linear-work column builder. This dense reference is not a compressed solver.
