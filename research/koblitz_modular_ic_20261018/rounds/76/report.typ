#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R76: exact finite zero divisors for b and c
The accepted full-table calculation classifies all six rational omitted branches by the integral source coordinate $V$. Above $z+1$, masks 6 and 7 have $V=1$; masks 8 and 10 have $V=0$. Both $z=0$ factors have nonzero square-zero $V$. The source relation and quadratic norms give the displayed valuations.

#image("valuations76.svg", width: 100%)

The quadratic equation is Eisenstein at $z=0$. At $z+1$ it is etale, and $R+1=(z+1)(V+z^2+z+1)$ with norm of the second factor $z+1$. The source norms are $N(R)=z$ and $N(R+1)=(z+1)^3$, so these are the complete finite zero supports. Each row has residue degree two; total finite zero degrees are 96 for $b$ and 80 for $c$. The proper pole divisors have these total degrees, while individual orders at infinity remain to be resolved.

One native attempt passed exit 0, exact marker, zero-error, six source-byte pins and five donor hashes. The chart is derived from that attempt and the stated valuation argument; Lean identity-check status is recorded separately.
