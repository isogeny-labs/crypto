# L08 refit: freed integer gadget (2026-11-02)

Residue 1 of SYNTHESIS.md section 5. L08 compared a *freed* char-2 Schoenhage gadget against the *fixed* integer gadget.
Here the integer gadget gets the same freedom: any (J, k) with L a power of 2, L >= 2n, L | 2J, and 2k + ceil(log2 n) + 1 <= J
(no coefficient wraparound). Code: intfree.py; parameter scan: scan.py; exactness: check.py; networks: net.py.

Exactness: 160 products (random and all-ones inputs, d = 131 and 133, four (J,k) choices) all equal x*y with every
constraint satisfied (check_out.txt).

Single 131-bit gadget: 37,038 edges (reference) -> 10,802 edges (J=134, k=66, n=2, L=4), 3.43x (scan_out.txt).

| network | m | reference gadget | freed gadget | cut |
|---|---|---|---|---|
| ECCp-131 (integer) | 2 | 1,564,506 (reproduces L08) | 567,346 | 2.76x |
| ECCp-131 (integer) | 3 | 2,724,592 | 992,688 | 2.75x |
| ECC2K-130 (char-2, L08 free27) | 2 | 2,694,680 | 787,740 | 3.42x |

Max degree drops 1,032 -> 540 on the prime network.

Conclusion: freed versus freed, the char-2 network at m = 2 is 787,740 / 567,346 = 1.39x *larger* than the prime
integer network. L08's "3.42x cut" was a freed-versus-fixed comparison; the like-for-like margin is about 1.4x in the
integer gadget's favour, confirming SYNTHESIS section 6 item 8. No decision changes: the exponent rule depends on a*c
with a about 3e3 edges per slot-bit, and a 2.8x constant moves no crossover below 2^256 bits.
Caveat: the best choice is the degenerate n = 2, L = 4 point, a constant-size optimisation at 131 bits, not an
asymptotic gadget. Python 3.13 required (the reference uses PEP 604 unions).
