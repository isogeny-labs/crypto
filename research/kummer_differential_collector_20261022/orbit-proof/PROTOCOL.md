# Frozen fixed-offset orbit protocol

Study ID: `KUMMER-FIXED-OFFSET-ORBIT-20261024`. The input file was frozen
before deriving the bound rows. It contains the 21 exact prime-order subgroup
orders already recorded by the Kummer model gate and 16 target-independent
nonzero generator multiples `J_i=iG`, `1 <= i <= 16`, per model. Its SHA-256
is pinned in `generate.py`.

For the state model, let `S` be a nonempty finite set of offsets and store one
quotient class `K(T+s)` for every `s in S`. Updating the entry at `s` after a
jump `J` by one differential xADD with a stored difference needs the prior
entry at `s-J`; therefore this model requires `S-J` to be contained in `S`.
The Lean theorem proves that closure includes every distinct iterate under
translation by `-J`. The native validator checks the exact cyclic orbit size
`r/gcd(r,j)` for every frozen row. All subgroup orders are checked prime and
all 336 jump scalars are checked nonzero and coprime to `r`.

Each bound row reports the exact minimum number of logical offset labels. The
storage column is specific to the current Rust `[u64;4]` Kummer-class state:
32 bytes per explicit class. The update-cost vector is specific to the regular
cached-theta xADD in `../src/walk_impl.rs`: one call per entry, with 24 field
multiplications and 36 field additions per call. The canonical 36-cell grid is
`S/M in {0,0.5,1}`, `I/M in {1,10,100,1000}`, and logical GAE in
`{0,0.1,1}`. It prices the 24 multiplications; additions remain separately
reported because this grid gives them no GAE coefficient.

This is a proof-and-derivation run. It does not execute a target, recover a
scalar, synthesize a recurrence, or measure wall time. The canonical jumps are
orbit witnesses, not reconstructed per-target jump tables. Because each group
has prime order, every nonzero jump has the same orbit bound.

Completion requires 336 rows, 12,096 price rows, zero validator failures, a
Lean proof-gate pass with no hidden axioms, a graph, PDF, and checksums. The
finding is restricted to the stated explicit fixed-offset update model.
