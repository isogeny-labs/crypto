# Skeptic check for MP-ml_empirical-1. Everything lives on Z/r (r prime) = <P> via log.
# For a filter h: Z/r -> Z/M compute exactly
#   pi_{c1,c2}(d) = Pr[h(A+B)=d | h(A)=c1, h(B)=c2]   (all triples, via FFT convolution)
#   G = M * max pi,   Fourier bound  1 + M^2 max_{d,k!=0}|f_d^(k)|  (idea's bound),
#   slack = (bound-1)/(G-1),  null = sqrt(p_d(1-p_d)/r),
#   J = p_d/max|f_d^|^2  = samples for SNR 1 in the shift-averaging reduction.
import numpy as np, hashlib, json, sys, math

def fhat(f):  # f^(k) = E_x f(x) e(-kx/r)
    return np.fft.fft(f) / len(f)

def analyse(name, h, r, M, extra=None):
    f = np.zeros((M, r))
    f[h, np.arange(r)] = 1.0
    p = f.mean(axis=1)
    F = np.array([fhat(f[d]) for d in range(M)])          # M x r
    Fraw = F * r
    absF = np.abs(F); absF[:, 0] = 0
    maxcoef = absF.max(); dmax, kmax = np.unravel_index(absF.argmax(), absF.shape)
    # exact conditional rates via convolution
    best = (0.0, None)
    for c1 in range(M):
        for c2 in range(M):
            conv = np.fft.ifft(Fraw[c1] * Fraw[c2]).real        # (f_c1 * f_c2)(x)
            joint = f @ conv / r**2                             # E[f1(A) f2(B) f_d(A+B)] for all d
            pi = joint / (p[c1] * p[c2])
            d = int(pi.argmax())
            if pi[d] > best[0]:
                best = (float(pi[d]), (c1, c2, d))
    G = M * best[0]
    bound = 1 + M**2 * maxcoef
    null = math.sqrt(p[dmax] * (1 - p[dmax]) / r)
    out = dict(filter=name, r=r, M=M, G=round(G, 4), G_over_M=round(G / M, 4), bound=round(bound, 4),
               slack_bound_over_gain=round((bound - 1) / max(G - 1, 1e-12), 3),
               maxcoef=float(maxcoef), maxcoef_over_null=round(maxcoef / null, 2), k_star=int(kmax), d_star=int(dmax),
               J_snr1=round(float(p[dmax] / maxcoef**2), 1), sqrt_r=round(math.sqrt(r), 1),
               bound_holds=bool(G <= bound + 1e-9), argmax=best[1])
    if extra: out.update(extra)
    return out

def sha_filter(r, M):
    return np.array([int(hashlib.sha256(str(x).encode()).hexdigest(), 16) % M for x in range(r)])

def interval_filter(r, M):
    return (np.arange(r) * M) // r

def bohr_filter(r, Ms, rng):
    x = np.arange(r); h = np.zeros(r, dtype=np.int64); mult = 1
    for Mi in Ms:
        a = int(rng.integers(1, r))
        h += mult * (((a * x) % r) * Mi // r); mult *= Mi
    return h

def diluted_filter(r, M1, M2):
    return interval_filter(r, M1) * M2 + sha_filter(r, M2)

def shift_avg_demo(r, h, M, d, k, J, rng, trials=20):
    # recover e(kx/r) from J shifted evaluations f_d(x+j) e(-kj/r); report phase error in units of 1/r
    f = (h == d).astype(float)
    Fk = fhat(f)[k]
    errs = []
    for _ in range(trials):
        x = int(rng.integers(r)); js = rng.integers(r, size=J)
        est = np.mean(f[(x + js) % r] * np.exp(-2j * np.pi * k * js / r))   # ~ e(kx/r) f^(k)
        est = est / Fk
        phase_est = (np.angle(est) / (2 * np.pi)) % 1.0
        true = (k * x / r) % 1.0
        err = min(abs(phase_est - true), 1 - abs(phase_est - true))
        errs.append(err)
    return dict(d=d, k=k, J=J, abs_fhat=float(abs(Fk)), median_phase_err_cycles=float(np.median(errs)),
                frac_err_below_1_over_8=float(np.mean(np.array(errs) < 1 / 8)))

if __name__ == "__main__":
    rng = np.random.default_rng(1)
    results = []
    for r in (4099, 65537):
        for M in (8, 16, 64) if r == 4099 else (8, 16):
            results.append(analyse("sha256", sha_filter(r, M), r, M))
            results.append(analyse("interval(top bits of log)", interval_filter(r, M), r, M))
            # Bohr boxes of increasing dimension at fixed M
            if M == 8:
                results.append(analyse("bohr_l3 (2x2x2)", bohr_filter(r, [2, 2, 2], rng), r, M, dict(ell=3)))
            if M == 16:
                results.append(analyse("bohr_l2 (4x4)", bohr_filter(r, [4, 4], rng), r, M, dict(ell=2)))
                results.append(analyse("bohr_l4 (2x2x2x2)", bohr_filter(r, [2, 2, 2, 2], rng), r, M, dict(ell=4)))
                results.append(analyse("diluted interval 4 x sha 4", diluted_filter(r, 4, 4), r, M))
                results.append(analyse("diluted interval 2 x sha 8", diluted_filter(r, 2, 8), r, M))
            if M == 64:
                results.append(analyse("bohr_l2 (8x8)", bohr_filter(r, [8, 8], rng), r, M, dict(ell=2)))
                results.append(analyse("bohr_l3 (4x4x4)", bohr_filter(r, [4, 4, 4], rng), r, M, dict(ell=3)))
                results.append(analyse("bohr_l6 (2^6)", bohr_filter(r, [2] * 6, rng), r, M, dict(ell=6)))
                results.append(analyse("diluted interval 8 x sha 8", diluted_filter(r, 8, 8), r, M))
    for o in results:
        print(json.dumps(o))
    # shift-averaging reduction demo on the interval filter, r = 65537, M = 16
    r, M = 65537, 16
    h = interval_filter(r, M)
    demo = [shift_avg_demo(r, h, M, 0, 1, J, rng) for J in (200, 2000, 20000)]
    for o in demo: print("SHIFTAVG", json.dumps(o))
    json.dump(dict(results=results, shift_avg_demo=demo),
              open("/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/MP-ml_empirical-1_skeptic/RESULTS.json", "w"), indent=1)
