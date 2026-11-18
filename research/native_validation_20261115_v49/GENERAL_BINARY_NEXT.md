# Broader point-root reduction without Artin–Schreier normalization

Use original F=X³+rX²+sX+t, G=hX+k on Y²+XY=X³+AX²+B. Target vanishing fixes t=z³+r z²+s z+(hz+k)yT. At any source x!=z, a lift y gives

(x+z)r+s = x²+xz+z² + ((hz+k)yT+(hx+k)y)/(x+z).

With fixed h,k, norm coefficients are affine F2-linear in r,s, without choosing c²+c=A+z. Thus a direct coefficient-span fiber supports trace-one contexts that the normalized compiler cannot represent. Splitting this exact r,s fiber by its r projection again gives a constant s image and x-linear projected columns.

If hx+k=0, the lift term vanishes. Test the single linear equation even when the curve has no rational lift: F(x)=G(x)=0 is a nonrational-lift/ghost branch and must remain. At x=0, the unique lift is sqrt(B); include it. A complete generic scan may not use the k=0 certificate to omit zero. For nonzero G(x), a norm root requires a rational curve lift and both lifts are retained. These algebraic identities are unimplemented here. Next implement full small oracles, then actual k=1, h*x_pole and ordinary/Koblitz curve contexts; do not infer general coverage from the old k=0 run.
