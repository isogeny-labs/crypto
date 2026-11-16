import random, sys
sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent)); sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent.parent / "lib"))
import freelip_widths as F
from curves import CURVES
F.WIN[62] = 2
p, gx, gy = CURVES["P-384"][0], CURVES["P-384"][4], CURVES["P-384"][5]
ctrl = [bool(F.scan(F.zrandomb_x(sd, p, 62, 7), 384, 62)) for sd in (987654321, 31337, 2**90 + 12345, 5, 2**100 + 7)]
rnd = random.Random(1997)
fp = sum(bool(F.scan(rnd.randrange(p), 384, 62)) for _ in range(200))
print("P-384 NBITS=62 two-limb window: controls", sum(ctrl), "/5; random values flagged", fp, "/200")
print("Gx:", F.scan(gx, 384, 62) or "none", " Gy:", F.scan(gy, 384, 62) or "none")
