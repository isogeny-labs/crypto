# R116: all four x=0 base series beyond G51 denominator orders

Bind the R51 donor, R105 parser, R98 denominator audit, R111/R112 Newton outputs, and R115 direct uniformizer output by SHA-256. Construct source-bound simple-root equations for the four x=0 base places:

- y0/e1: x=s, y=s*z, divide f by s^2, lift z(0)=1.
- y0/e3: x=s^3, y=s*z, divide f by s^4, lift z(0)=1.
- y1/e1: x=s, y=1+s+s^6*z, divide f by s^10, lift z(0)=1.
- y1/e2: s=t=v/x, use the exact R115 equation E(x,t), lift x(0)=0 and y=1+x+x^2+x^3*t.

Compute through order s^640 in F2[[s]]. Verify each transformed equation has a simple root, all residuals vanish, the original R51 f residuals vanish, x orders are 1,3,1,2, and the first 128 coefficients of the e2 series match R115. The R98 maximum denominator exponent 161 pulls back to at most order 483, strictly below 640. Fresh byte-identical replay, altered-source rejection, independent source evaluation, and Lean finite residual checks are acceptance controls. Commit source/protocol before data. This supplies base inputs, not yet actual G coefficient valuations. Local exact algebra; no cloud or timed claim.

Sources: R51 donor, R98 denominator audit, R105 parser, R111/R112/R115 local data, [Stacks Henselian lifting](https://stacks.math.columbia.edu/tag/04GE), [Newton polygon theorem](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf).
