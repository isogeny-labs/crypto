# Concrete normalization prediction before the first map execution

This handwritten modular derivation uses only the frozen unit13 constants;
no map/dual output or scientific compiler run has been observed. It is an
interpretive prediction, not a signed or Lean verdict. [pcc:unverified]

The new target's constant coefficient is16, and c=4. Since that constant
is H_leading/c², H_leading=16*16=256=54 mod101. Thus the old constructor's
raw leading coefficient is84*54=4536=92 mod101. Its roots are30 and71,
so the prescribed smallest positive root is T=30. The roots of54 are16
and85, so the prescribed s is16. Finally c_normal=4/16=76, since
16*76=1216=4 mod101. [pcc:unverified]

The fixed source b_normal=96 satisfies36*96=3456=22 mod101. In the
conjugacy coefficient,96*76=24 and24*30=13 mod101. The required minus sign
from x-z=-(w-v)/(wv) consequently predicts

    old_scale30, s16, c_normal76, new_scale13, kappa100.

The native harness must determine its values from the frozen constructor
and formula, before its divisor comparisons. This note does not add a new
fitted sign or replace any frozen test criterion. [pcc:unverified]
