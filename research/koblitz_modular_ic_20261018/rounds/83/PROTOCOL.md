# R83 direct positive fraction membership, frozen before data

R82 was analyst-interrupted in the known full 22-variable standard
basis before checking an identity. R83 loads the same pinned R50
selected source, fractional module, and rejected `normalP` candidate
ring. It checks the literal `x,y,p` source map and the 19 surviving
`T(k)` names. For each, reduce the complete denominator-cleared
polynomial `module50[27]*T(k)+module50[k]` (after source-map
substitution) against the saved `norid` generators **without**
computing their standard basis. A zero remainder is a sound positive
ideal-membership certificate. A nonzero remainder is explicitly
inconclusive, not a disproof, and fails the acceptance gate. The
producer's earlier selected-prime denominator check is pinned as a
donor rather than recomputed in a new global verifier.

Run locally in Singular, one Singular/FLINT thread, seed 83, 1200
seconds and 4 GiB virtual. Copy source/protocol/driver bytes from
committed Git and verify donor hashes. Accept only exit 0, exact
marker, zero native whitespace-`?` errors, and all 19 zero
remainders. Retain failures and timeouts; freeze any repair before a
fresh ID. No timing comparison. The result, if accepted, is positive
fraction membership in the rejected candidate presentation only.
