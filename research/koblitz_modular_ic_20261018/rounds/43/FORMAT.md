# R43 coefficient and product-certificate formats

All field words0..31 use F2[a]/(a5+a2+1), with bitk the coefficient of a^k.
Jets use uniformizer z=u-u(Q); there are615 coefficient words per function.
C rows order u,v,b,c,lambda,nu; U/I are the eight registered entries/inverses.
space1/2/3 are the fixed68/196/324 bases with product pairs pinned in source-data.

Each MULT43 selected-basis text row contains1-based selected index,1-based
parent pair i,j, and a comma-separated615-word series. Parent basis is the
original L2 for L4, original L3 for L6, then selected L6 for L12 and L12 for
L24. Recursively this is an explicit original-function product expression.

Each binary coordinate record is400bytes: i,j as two little-endian u32
ZERO-based parent indices, followed by49 little-endian u64 words of F2
coordinates in the selected ORIGINAL basis. Trailing bits/unused basis
coordinates are zero. No header; read the exact count from its stage result.
The coordinates remain valid after later basis selections append columns.

Pivot files alternate rows tagged pivot and coordinates. Each has its
ambient high-bit index and49 hex u64 words. Ambient bit5*k+b represents
coefficient a^b z^k, 0<=k<615,0<=b<5. Coordinate bits refer to the selected
basis. Every native pivot is reconstructed from original columns, not just
from an evolving echelon matrix. Jet pivot files use the same convention
with a single line containing index, comma-separated ambient and label words.

These formats are byte/exact-linear-algebra contracts, not geometric
normalization or saturation certificates. Restore from the raw artifact;
full independent scientific replay remains a separate gate.
