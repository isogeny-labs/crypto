# Exact formal scope

The cube is ordered00,01,10,11; the first listed bit is the first physical
coordinate. Matrix indices3,3 therefore mean Y=X=(1,1).

| Declaration | Faithful mathematical statement |
| --- | --- |
| `GaugeCounterexample.composed_kernel_entry` | `(K(W) * K(V))[11,11]=4` for the displayed integer matrices |
| `GaugeCounterexample.kernel_product_entry` | `K(W*V)[11,11]=6` for the same matrices |
| `GaugeCounterexample.kernel_not_multiplicative` | `K(W*V) != K(W)*K(V)` as4-by-4 integer matrices |

`kernel` is the actual product of the two affine Boolean interpolation
weights, not a replacement matrix chosen to have the desired entries.
The first two proofs expand actual matrix multiplication and finite sums.
The third follows by applying equality to entry3,3 and integer arithmetic.

No general gauge, determinantal-ideal invariant, entry-cost theorem,
Cauchy-family obstruction, contraction theorem or ECDLP algorithm has
been proved by this project. Those claims are outside its formal scope.

No custom axiom, sorry, admit, native_decide or native trust is used.
