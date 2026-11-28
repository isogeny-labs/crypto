# Fresh complete transfer check for the relation pipeline

Successor to the fixed relation qualifier; do not modify its pinned source.
Use the same actual kernel3 curve and freshly retained point pool. Recompute
F=phi*pi* and B=pi_*psi for the fixed public inputs O,G,-G,2G,Q,-2G.
G and Q are public coordinates from the retained public-class input. No
scalar is consumed to construct their images. The last input makes P+2G=O,
so no shifted elliptic-fiber shortcut is permitted in this check.

For each nonzero input: construct pi*, execute the complete forward
correspondence, encode the resulting cycle in the actual target KM model,
transpose its complete closed-point cycle including multiplicities, subtract
the correct reference divisor with source degree, and apply the original
cover norm. Require psi*phi(D)=3D before checking B*F(P)=6P. Zero is checked
through the identity class. Compare G/Q with their pinned target KM classes;
check negatives and doubling in target arithmetic.

Use a separate executable containing only the preceding separate checker
arithmetic (its main removed) to check every final source/norm pair with
OpenSSL, and reject a norm changed by adding G. Retain input point identities,
forward cycles, source dual outputs and stage timings. This is a cross-library
elliptic check of fresh producer maps, not independent geometric or signed
verification. Preserve caps:600 seconds and1GiB per native worker. No paid run.

Native source snapshots and source/input/binary hashes precede calls. Use a
new output directory for every attempt. Existing protocols and reports stay
immutable. Admitted EKS measurements and independent signed verification are
still required by the broader goal. Sources: existing complete/transpose
protocols and Khuri-Makdisi source already reviewed for the parent pipeline.
