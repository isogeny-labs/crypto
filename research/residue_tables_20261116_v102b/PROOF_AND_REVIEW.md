# Fixed-trace exact tables, with repeated roots

The H4 coefficient is fixed in the inherited affine family. A monic quintic
over a residue field splits completely, including all multiplicities, iff it
is the product of a sorted multiset of five field elements. In characteristic
two their XOR is H4. Enumerate the first four in nondecreasing order and set
the fifth to H4 XOR their sum; keep it iff it is at least the fourth in the
canonical field-element ordering. Every multiset appears exactly once.
Unique factorization makes its coefficient tuple unique. Thus membership in
the table of H0..H3 is exactly equivalent to C100's nilpotency predicate for
the same residue, not merely necessary locally. Odd degree five and the
characteristic-two translation action make all trace fibers equicardinal,
with binomial(q+4,5)/q entries. This count is checked, not used to assume membership.

Bottom-up truth-vector reduction constructs a binary decision diagram. Each
node has exactly the low/high subfunctions for one coefficient bit. Equal
children collapse; equal pairs at the same level share a node. Induction on
levels proves exact membership for every input code, preserving zero and
repeated roots. No local coefficient selector or new independent q is added;
the same 76 global parameters must supply all residue coefficient bits when
integrated. The current worker validates this binding on 16 samples per field.

Bounds: q<=32, at most binomial(35,4)=52360 four-root states per residue,
at most 2^20 truth-vector bytes and bounded DAG storage per field. There are
exactly fourteen inherited residues. No 2^73 enumeration or oversized quotient
allocation occurs. Total table generation, diagram construction, oracles and
reporting count against the existing 90-second/4GiB/one-core worker limit.

Review: literal division by every field linear factor is independent of the
nilpotency implementation, and includes repeated/zero roots. Exhaust every
fixed-trace coefficient tuple for q<=4, plus frozen random and positive cases
for larger fields. Separately compare actual shared-parameter evaluations
to the old numeric predicate. A smaller DAG, if found, is a representation
measurement, never a completed original solve or qualified speed ratio.

This is a changed representation hypothesis prompted by the measured affine
fill regression. Root-multiset enumeration and reduced decision diagrams are
known techniques; no primary-literature novelty claim is made.
