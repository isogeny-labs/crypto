#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R71: rational omitted fibers in the normal affine order
The full rank-32 source multiplication table is specialized at both rational omitted primes. At each point, all 32 basis-square columns define Frobenius on the entire 32-dimensional fiber algebra. Iterating it removes the nilradical. The stable image has dimension four at $z=0$ and eight at $z=1$.

#image("fibers71.svg", width: 100%)

== Exact checks and interpretation
The $z=0$ image stabilizes after five squares and is $bb(F)_4 times bb(F)_4$. The $z=1$ image stabilizes after four squares and is $bb(F)_4^4$. The program computes fixed dimensions for all 32 Frobenius powers, inverts the gcd-degree equations over the integers, verifies nonnegative degree counts, and reconstructs every fixed dimension. The source unit squares to itself in both fibers; Frobenius preserves each stable image.

One native run passed exit 0, the exact terminal marker, and zero Singular error lines. Six historical executed-source byte pins and five donor hashes match. The four-second UTC interval is only a run receipt.

The reduced algebra determines two residue-degree-two branches over $z=0$ and four over $z=1$. The unreduced fiber dimension is 32 at each prime. Primitive idempotents and their full local-factor lengths are required to separate the ramification indices. Local uniformizers and valuations of the source coordinates and registered denominators, plus the degree-five and infinity fibers, remain next.
