"""One-edit test for the B-163 seed. Any single substitution leaves >= 10 intact consecutive bytes (and the
corresponding chunk runs at other widths), so scanning EVERY short window of the published seed for drand48-
family output (top w bits of the LCG mod 2^(s+w)) covers every one-character edit. Window: ceil(72/w) chunks
(24 bits of margin over the 48-bit state). Control: the P-256 seed with byte 9 corrupted."""
import sys, math
sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent.parent / "lib"))
from scan_unexplained import lcg_recover
from curves import CURVES

def scan(V, L=160):
    hits = []
    for w in (8, 15, 16, 24, 31, 32):
        need = math.ceil(72 / w)
        for s in range(0, 48 - w + 1):
            for off in range(w):
                nw = (L - off) // w
                if nw < need: continue
                words = [(V >> (off + w * j)) & ((1 << w) - 1) for j in range(nw)]
                for order, seq in (("top-first", words[::-1]), ("bottom-first", words)):
                    for st in range(nw - need + 1):
                        if lcg_recover(seq[st:st + need], w, s + w) is not None:
                            hits.append((w, s, off, order, st))
    return hits

b163 = int("85e25bfe5c86226cdb12016f7553f9d0e693a268", 16)
p256 = bytearray(bytes.fromhex(CURVES["P-256"][1])); p256[9] ^= 0x5A
ctrl = int.from_bytes(p256, "big")
c = scan(ctrl); print("CONTROL P-256 seed, byte 9 corrupted:", "found" if c else "MISSED", c[:2], flush=True)
h = scan(b163); print("B-163 seed, all short windows:", h if h else "none")
