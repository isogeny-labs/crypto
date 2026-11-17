# Membership constraint cost per slot, m=3 (k = ceil((m-1)/m * b) parity rows), W = bits(s) = b+2.
# Units: "nl" = nonlinear ports (AND / MAJ / 3-input product gates); "xor" = XOR constraints.
import math
print('b   W  | Bochum: mults(var x const) nl-ports  xor-ports | dense-F2: rows x width, xor-terms | sp3 | sp5 | sp7 (rows x w, xor2-gates) | window nl')
for b in (16,32,64,128,256):
    W = b+2; k = math.ceil(2*b/3)
    # Bochum: alpha*enc: constant multiplier = ~W/2 shifted copies of the W-bit word => (W/2-1) W-bit adders, each W MAJ (nl) + 2W XOR (sum bits)
    cm = W//2 - 1
    # Barrett-style mod s: two further constant multiplications (quotient estimate, q*s) of the same shape, + subtract + correct
    nl = 3*cm*W + 3*W + W       # 3 const-mults, add beta/sub/correct (3 carry chains of W MAJ), compare (W MAJ)
    xr = 3*cm*2*W + 3*2*W + W
    mults = 3
    dense = (k, (b+1)//2, k*((b+1)//2 - 1))
    sp = {w:(k, w, k*(w-1)) for w in (3,5,7)}
    win = 3*W    # sub offset mod p (carry chain + correction) + compare with width, ~3 carry chains
    print(f'{b:3d} {W:3d} | {mults} mults, nl={nl}, xor={xr} | {dense[0]}x{dense[1]}, {dense[2]} | ' + ' | '.join(f'{sp[w][0]}x{w}, {sp[w][2]}' for w in (3,5,7)) + f' | win nl~{win}')
