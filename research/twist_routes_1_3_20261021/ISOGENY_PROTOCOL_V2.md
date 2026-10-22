# E14 repeated-eigenvalue dual amendment, version 2

Keep V1's complete 30-cell records and its explicit missing-dual case. V1 only
tried complementary torsion with a *different eigenvalue*. A repeated
eigenvalue does not imply that its eigenspace contains just one cyclic line.

Repeat V1's exact ten fixtures, degrees, seeds, forward-map checks and model
controls in a fresh directory. For a dual, use the other eigenvalue when there
are two; otherwise sample the same eigenvalue. Require the sampled order-ell
point to lie outside the original kernel by checking its nonzero image under
the forward map. That image is the proposed dual kernel. Try at most 16 calls
to the original bounded point finder (each at most 128 raw samples). Record
the number of calls and retain a missing dual if the search is exhausted.

Construct and verify the full rational dual functions and model scale as in
V1: every rational source point must satisfy the composition [ell], and eight
scalar controls per successful map must return to the source via ell^-1.
Accept no kernel based on an eigenvalue alone. Keep the finite lookup control
and its table size separate from the algebraic return. Preserve failed
attempts; any incorrect composition blocks acceptance.

The same one-process 4 GiB, 30-minute L0 envelope applies. No timing, full-cost
comparison, new algorithm or NIST benefit follows from completing this map.
Freeze this source/protocol amendment before execution.
