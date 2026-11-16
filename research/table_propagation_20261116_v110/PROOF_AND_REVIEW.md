# C110 finite-domain propagation contract

For domains D_i of distinct tuple tokens with possibly identical images f_i,
the one relation is XOR_i f_i(t_i)=b. Integer Walsh convolution computes the
exact number of completions of every token. Delete a token only when that
number is zero. Every satisfying tuple witnesses support for each of its
tokens, so all satisfying tuples survive and all unsupported tokens disappear.
One pass is already a fixed point for this single relation: every witness
uses tokens that also have that same witness. Empty domains and zero support
are handled exactly. A cap raises unresolved, never a zero count.

The actual eighth observation is quotiented by the image of the four residual
kernel bits. Membership in this quotient is necessary and sufficient for an
eighth accepted tuple to have a common original theta. Its kernel image has
rank four, so compatible pairs have a unique kernel preimage. Multiplicities
of projected images must be retained; deduplicating them changes counts.
Original q bits are still recovered through C109's bijection.

Bounds: at most4096 images, at most1000000 histogram/butterfly/pointwise work
units, exact unbounded Python integers. Eight domains require O(8*4096)
stored integers; their magnitudes are bounded by products of input domain
sizes. Cold setup, table generation, final evidence and full process lifetime
are charged. Worker demotes at78s, emits by84s, outer90s/4GiB/core0.

The result is exact for ONE necessary finite-domain constraint, not complete
original339-variable roots or an UNSAT certificate for other charts. Repeated
roots are represented by all root multisets in old exact tables. This binary
XOR mechanism is not a prime-field interval algorithm. FWT and generalized
arc consistency are prior techniques; novelty is not established. Performance
must retain no-cut outcomes and the two-billion-plus surviving outer choices.
