# Trace -14 degree-four fixed-pair screen

Frozen before execution on 15 November 2026. Adapt the exhaustive degree-four denominator search from the trace +26 witness to `Y:y²=x^11+x^8+x^7+x^6+x^5+x²+2x` and `E:v²=u³+u²+(a²+2)` over `K=F3[a]/(a^5+2a+1)`.

After translation to send infinity to `O`, a degree-four map has `u=A/B` with `deg A=4` and odd `deg B≤3`. Branch parity forces the rational branch factor `x` into `B`. The possible monic denominator types are `xQ_i`, `x(x-c)²` for `c∈K×`, `x³`, and `x`. For each, every quadratic branch factor not in `B` receives one of three elliptic two-torsion residues over its quadratic extension. CRT gives the unique candidate numerator of degree at most four. If it has degree four and is coprime to `B`, test the exact polynomial identity. Expected assignment count is `5·3^4+(242+2)·3^5=59,697`. Zero exact hits excludes degree four for this exact pair only. The source SHA256 and survivors are retained.
