# R52 module closure proof scope, before data
Compile Closure52.lean using retained Lean4.34.1/mathlibd13f23b cache,
-j1,-M4096 and1200s separately compile/audit,one serialized math job,no
virtual cap. Audit all four declarations,zero failing/trusted/native_decide.
Preserve failures. Actual matrix dimensions/entries,source-bound identities,
PID calculations,normalization bridge and the rank32 Cayley-Hamilton bound
are external; the formal statements certify range invariance from an
intertwining product,two-inclusion range equality,containment of every
adjunction stage in any invariant containing module,and the stability test.
No native result is asserted as a Lean theorem here.

Before second compile: first source reports missing range import and an
elaboration error in the sup inclusion. Import Submodule.Range and give the
inclusion its full module type. Preserve first compile receipt; same four
mathematical propositions and toolchain.

Before second compile add two relevant implications: unit scalar cancellation
in a submodule,and degree-pattern compatibility for(8,8) versus(2,14).
Total six declarations. Neither actual fibers nor monic specialization
premises are asserted. The latter is a new proposed route for primality
of the larger source without requiring an impossible irreducible16fiber.
