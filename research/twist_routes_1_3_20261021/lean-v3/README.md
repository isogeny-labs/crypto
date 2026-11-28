# Classical F3 square lifting

Lean 4.34.1 and mathlib d13f23b723b8a846827a245b89c10fc7d3f11612
check five exported theorems in [TwistPairLift.lean](TwistPairLift.lean).
The native axiom auditor accepts all five, with no sorry, trusted declarations
or native_decide. Initial [build receipt](receipts/build-v1.json) and
[axiom audit](receipts/axiom-audit-v1.json) retain the original executions.

For f(x)=x^3+a*x+b, A=(x-z)^2 and M=(x+z)*(x*z+a)+2*b, the classical
Semaev polynomial obeys

```
(2*A*y-2*M)^2 = 16*f(x)*f(z) + 4*A*F3(x,y,z).
```

Consequently if w^2=f(z), 4*w is nonzero and F3(x,y,z)=0, the element
(2*A*y-2*M)/(4*w) squares to f(x) in the same field. Symmetry supplies the
other lift. The unconditional polynomial identity also has native certificate
[IDC1hfab3acdc297476e5](../execution-v3/identities-20261026-v1/0-IDC1hfab3acdc297476e5.json),
32 issuer and 64 extra checker probes, plus a rejected false-plus-one control.
The exact Lean proof, rather than those finite probes, proves the identity.

These are standard identities, not a novelty or attack claim. The formal
theorem establishes square lifting; its elliptic-curve sum bridge is checked
by native finite tests and independent PARI, and is not yet formalized here.
The anchored target's nonzero y-coordinate is essential. Arbitrary unanchored
summation systems are not covered by this field-lifting statement.

Replay with a cached tool root containing `lean/` and the pinned `mathlib/`:

```sh
TWIST_PROOF_TOOL_ROOT=/workspace/shared/twist-tools-20261023 \
  bash research/twist_routes_1_3_20261021/lean-v3/replay.sh /absolute/fresh/receipts
```

The transport records source, binary, dependency, stdout/stderr hashes and
resource limits. A fresh native axiom auditor is compiled from the repository
source. No hard virtual-address cap is imposed on Lean's cached mapped objects;
the managed-memory and sampled-resident bounds are described in the protocol.
