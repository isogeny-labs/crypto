"""JKSST-type union bound with explicit packing constants.  [pcc:unverified]

eps(r, rho, T, q) = ((2C+1)^r - 1)/q is the per-trial bound on Pr[lifted points dependent],
where C is the relation-coefficient cutoff obtained from the height ratio rho = H/lambda:
  C_lean  = T * ceil(1 + 2 r sqrt(rho))^(r-1)      (kernel-checked cutoff, JKSSTConditionalHeightCore)
  C_sharp = 2 N, N minimal with (2N+1)^r > T (1 + 2 r N sqrt(rho))^(r-1)   (same pigeonhole, tighter count)
A size b (q ~ 2^b) is *excluded* when the expected number of trials 1/(2 eps) exceeds sqrt(q),
i.e. log2(eps) < -b/2 - 1.  All logs below are log2 unless named ln.
"""
import math

LN2 = math.log(2.0)


def log2_C_lean(r, rho, T=1):
    return math.log2(T) + (r - 1) * math.log2(math.ceil(1 + 2 * r * math.sqrt(rho)))


def log2_C_sharp(r, rho, T=1):
    s = math.sqrt(rho)

    def ok(N):
        return r * math.log(2 * N + 1) > math.log(T) + (r - 1) * math.log(1 + 2 * r * N * s)
    # f(N) = r ln(2N+1) - (r-1) ln(1+2rNs) - ln T is eventually increasing; scan powers of 2 then bisect
    lo, hi = 0, 1
    while not ok(hi):
        lo, hi = hi, hi * 2
        if hi > 1 << 4000:
            raise OverflowError
    while hi - lo > 1:
        mid = (lo + hi) // 2
        if ok(mid):
            hi = mid
        else:
            lo = mid
    return math.log2(2 * hi)


def log2_eps(r, log2C, bits):
    # log2(((2C+1)^r - 1)/q) ~ r*log2(2C+1) - bits
    C = 2.0 ** log2C
    return r * math.log2(2 * C + 1) - bits


def margin(r, log2C, bits):
    """>0 : the bound excludes beating rho at this size (bits of slack)."""
    return -(log2_eps(r, log2C, bits)) - (bits / 2 + 1)


def lam_petsche(D_nats, d=1, norm=0.5):
    """Unconditional lower bound on the minimal nonzero canonical height of E'/Q (Petsche 2006,
    Thm 2, constants as recorded in the repo corpus) with the Szpiro ratio bounded only by the
    trivial sigma <= log|Delta_min|/log 11 (conductor >= 11 over Q).  `norm` absorbs a possible
    factor-2 normalisation difference."""
    sigma = max(1.0, D_nats / math.log(11.0))
    c = 1.0 / (1e15 * d ** 3 * sigma ** 6 * math.log(104613.0 * d * sigma ** 2) ** 2)
    return norm * c * D_nats


def crossover(r, H_of_bits, D_of_bits, cutoff="sharp", T=1, bmax=200000):
    """Smallest bit size b at which the unconditional (Petsche) bound excludes beating rho."""
    f = log2_C_sharp if cutoff == "sharp" else log2_C_lean
    prev = None
    b = 16
    while b <= bmax:
        H, D = H_of_bits(b), D_of_bits(b)
        rho = H / lam_petsche(D)
        m = margin(r, f(r, rho, T), b)
        if m > 0:
            # bisect between prev and b
            lo, hi = (prev if prev else 16), b
            while hi - lo > 1:
                mid = (lo + hi) // 2
                Hm, Dm = H_of_bits(mid), D_of_bits(mid)
                if margin(r, f(r, Hm / lam_petsche(Dm), T), mid) > 0:
                    hi = mid
                else:
                    lo = mid
            return hi
        prev = b
        b = int(b * 1.25) + 1
    return None


if __name__ == "__main__":
    for r in range(2, 10):
        for rho in (1.0, 1.25, 4.0, 10.0):
            ls, ll = log2_C_sharp(r, rho), log2_C_lean(r, rho)
            print(f"r={r} rho={rho:5.2f}  log2C_sharp={ls:7.2f} log2C_lean={ll:7.2f}  "
                  f"margin@256(sharp)={margin(r, ls, 256):8.1f}  margin@256(lean)={margin(r, ll, 256):8.1f}")
