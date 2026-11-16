# Rectangular generator bridge, before affected data
A final claim-scope check noticed that Closure52.ranges_eq_of_two_products
uses equal generator-domain types. It applies to square normalized-basis
comparisons,not directly to a64-generator combined module versus32generators.
Keep that accepted source/object unchanged. Add a new one-theorem module
RectBridge52 with distinct domain types and rectangular witness maps. This
matches the actual two complete closure products without padding assumptions.
Same pinned Lean4.34.1/mathlibd13f23b,-j1,-M4096,1200s separately compile/audit,
no virtual cap,serialized math and pre-data Git freeze. Require compile0,
audit0,one theorem,zero failing/trusted/native_decide. Retain any failure.
No actual matrix entry,finite standard basis or normalization premise is
formalized by this generic implication. This is new proof work,not a repeat
of an accepted producer for publication.
