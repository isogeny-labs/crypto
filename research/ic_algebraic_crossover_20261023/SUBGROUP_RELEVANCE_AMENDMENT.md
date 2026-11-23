# Bounded actual-subgroup enumeration after the first audit

2026-10-25; declared after the first source-c1a7c2d4e receipt, before the
second audit's output. Preserve the complete first receipt and source pin.

The first audit leaves 152 generated-subgroup membership outcomes unknown.
All belong to the E0 model over GF(2^10), whose group order is 2^3 * 11^2.
The conservative gcd(r,h)=1 criterion is insufficient there. The actual
declared subgroup has only 11 points, so exact enumeration is feasible.

Amend the native auditor with a rule applied to every model: if uniqueness
does not follow and r<=4096, enumerate the actual <G> by repeated addition,
checking distinctness and the final return to O. Test target membership by
lookup in that complete set. Larger ambiguous subgroups remain unknown.
Add a small-field control that finds an r-torsion point outside <G> and
requires rejection; [r]P=O alone must not become a positive certificate.

For the selected original SAT witnesses, explicitly report the known zero
usable-subgroup relation count when a summand is known outside <G>, instead
of leaving that count null. Unknown memberships continue to propagate.
The first audit's conservative nulls are retained unchanged.

Keep every input, model, generator, branch and execution limit unchanged.
No selective retry or scientific campaign decision is made. Schema v2
names its totals generically because it supports both uniqueness and exact
cyclic-subgroup enumeration. Run into a fresh receipt directory, freeze
updated source before output, compare a fresh native rerun byte for byte,
and report both audit versions with their exact scopes. This is relevance
and correctness only; no speed or degree-scaling claim is authorized.
