# Composite orders in the prime-field arithmetic

The modular embedding-order conclusion extends to any integer r>2
dividing the actual point count, once the original trace condition and
explicit Hasse premise are retained. Primality of r is unnecessary.
The general Hasse theorem is not proved by this increment. [pcc:unverified]

For prime p≥5, the source supersingular condition gcd(t,p)≠1 gives p|t.
The Hasse premise t²≤4p forces t=0, hence the actual point count is p+1.
Every integer r>2 dividing that count has p≡−1 modulo r. Its square is 1;
if p were already 1 modulo r, then r|2, contradicting r>2. Lean therefore proves
exact modular order 2 for composite r as well. [pcc:unverified]

An actual positive control uses the prior smooth curve E/F7:y²=x³+x,
with #E(F7)=8 and the source supersingular trace property. For the new
actual point P=(3,3), Lean checks 8P=0 and 4P≠0, so addOrderOf(P)=8.
The modular embedding order of 7 modulo 8 is 2. No cyclic operation table is
assumed; the point operations are Mathlib's actual affine group operations.
[pcc:unverified]

The source is [CompositeTorsion.lean](lean/CompositeTorsion.lean):

- `trace_zero_integer_embedding_degree`: r>2 and r|q+1 imply exact modular order 2.
- `corrected_integer_embedding_degree`: actual point count/source trace and explicit Hasse premise.
- `eightPoint_order`, `actual_composite_order_control`: actual composite-order control.
- `composite_order_audit_certificate`: one explicit conjunction binding the universal conditional result and unconditional control.

The entire census project gate passes 205 declarations, 0 failing and 0 trusted
native exceptions, including generated declarations. No custom axiom,
proof hole or native_decide is used. Packet PCC-20261109-10789d binds the
conjunction and remains unsigned. [pcc:unverified]

Its full local packet check passes at16:44:10Z. Exact source/gate/receipt
bindings and explicit missing bridges are in
[composite-torsion-verification.json](composite-torsion-verification.json).
[pcc:unverified]

Primary source: original [FST2010](https://doi.org/10.1007/s00145-009-9048-z),
p230 Definition 2.1/Remark 2.2, permits integer subgroup orders. Exact source
hashes/read scopes and bounded Mathlib lookup are recorded in
[composite-torsion-sources.json](composite-torsion-sources.json).
See [gate](evidence/composite-torsion-gate.log) and
[proof report](reports/proof-20261109-composite/report.pdf). [pcc:unverified]

Composite orders do not supply a counterexample under this trace-zero
divisibility condition. The previous actual r=2 counterexample remains a
separate boundary. General Hasse, trace/p-rank/classification and
cyclotomic-field/full-geometric-torsion bridges remain open, as do the
all-p<500 and binary/ternary censuses. This is not all-fourteen completion.
No native curve population, benchmark or paid compute was run.
[pcc:unverified]
