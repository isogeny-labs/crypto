# Frozen attack comparison for the two order-4-mod-8 witnesses

The exact sources are the ordinary curves `E_b: v^2+uv=u^3+b` over
`GF(512)` in Conway representation, with `(b,#E,r)=(155,548,137)` and
`(32,556,139)`. The source points, target points, and Hess factors are
fixed by `cert_548.magma` and `map_556.magma`; in both cases `Q=[7]G`.

An end-to-end transfer run starts before source and cover construction,
constructs the Hess cover, maps `G,Q,O`, obtains a discrete logarithm from
relations in the cover Jacobian, and verifies `[x]G=Q` on the source. It
records setup, map, relation and linear-algebra/decomposition, and source
verification times separately where available. A generic Jacobian group
solver is a transfer-control result and is never reported as a GHS
index-calculus cost. Failed index-calculus attempts and transport failures
remain separate receipts.

The rho comparison uses the same exact `E,G,Q,r`, deterministic seeds,
and a wall-clock start/end measured inside the runtime. The measured total
GHS attack includes setup and map time. Rho uses no precomputed answer and
must return `x` verified by `[x]G=Q`. Each result records the implementation,
host/runtime, input hash, and raw output. The answer 7 is known solely for
verification and never supplied to the solver.

The first index-calculus attempt uses Magma's documented `IndexCalculus`
intrinsic on the Hess divisor images. Magma documents a `q >= d!` practical
requirement for its line-sieve method, so the curve model and field size
are part of the receipt. If that intrinsic cannot process the source
model, the next stage is a canonical quartic map and extension-field
factor base, with an independently checked pushforward of the source
divisor images.
