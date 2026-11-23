# EXP16bv: one capped attempt at the m=n=4 generic affine line ideal degree (Q = 2^d), reusing exp16bv_check.sage setup.
# usage: sage exp16bv_generic4.sage d seed
import sys
src = open('exp16bv_check.sage').read(); src = src[:src.index('u = K.random_element()\nwhile True')]
exec(preparse(src))
u = K.random_element(); c0 = u * K.random_element()
eqs = descend(c0, u); print(f"generic: span dim {spandim(c0/u)}, dim U {len(Ubasis(eqs))}", flush=True)
dg, tg_ = gdeg(Rq, eqs); print(f"generic ideal degree {dg} (GB cpu {tg_:.1f}s)", flush=True)
print("DONE", flush=True)
