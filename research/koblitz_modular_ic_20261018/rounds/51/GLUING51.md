# Actual normalization-generator maps and remaining gluing checks

Native z is named u in the saved PID/field files; V is v and lambda is l.
Let E=V²+z²V+z+z³, h0=z+V and h1=z+V+z². The accepted
pure-polynomial identities are

    h0² + z(1+z+z²+zV) = E
    h1² + z(1+z+z²+z³+zV) = E
    h0*h1 + z(z+1) = E.

Thus on either principal open h_i != 0, z has the explicit inverse
q_i/h_i². These are identities on the whole affine base, including branches
at z=0 and z=1; no cancellation by z was used to prove them.

The actual first/ninth diamond maps on (lambda,V,z) satisfy both defining
source equations by complete polynomial lift matrices. The accepted ten
iterates have fifth map equal to the sign involution and tenth map identity.
The localization unit checks bind the reciprocal z-coordinate to h0/h1.

For the actual R50 rank32 normal-chart algebra basis e_i=H_i/D, every one
of its32 generators passes h0^62*T(e_i) in the PID module, and every generator
passes h1^62*T^-1(e_i) in the same module. Each result includes the entire
32-entry coefficient vector, the full H-times-lift product, and an altered
coefficient rejection. Both complete32-column products pass. Bound62 comes
from the maximum coefficient degree37, denominator degree37, lambda pole
bound4e and V pole bound2e: 37-37+4*15+2=62. The older source-lift bound454
was also frozen and checked; no coefficient search selected62.

The fresh GLUE_RELOAD51 stage is the remaining independent compatibility
check: substitute each polynomial numerator directly in three coordinates,
check a full E-times-quotient identity, link the result to every saved PID
coefficient, and bind the first/ninth maps to the tenth-iterate identity.
Until its terminal receipt is accepted, the two directional regularity
checks are recorded separately rather than labeling the independent gluing
check complete.

The Lean Overlap51 module proves the three displayed identities under the
explicit characteristic-two assumption; constructs z's inverse on each
principal open; and proves that inverse field maps induce inverse maps under
injective compatible embeddings. It does not encode the native arrays or
prove the RR identification, source primality, geometric open cover, or
polarized embedding. Kernel compilation is accepted; axiom audit is pending.

Once compatibility passes, the geometric interpretation uses the retained
R50 complete-RR/normal-algebra identification and the recorded disjoint
excluded-fiber cover. This supplies concrete chart regularity data for that
construction. The selected modular-factor map and proper polarized embedding
remain distinct construction tasks.
