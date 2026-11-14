"""Concrete-size evaluation of the obstruction, using the toy-measured growth of the lift sizes.
[pcc:unverified]

Inputs: out/summary.json (per (bits, r): median H = max hhat of the lifted points and median
log|Delta_W| in nats, from the honest arm's height subsample).
Fits H(b) = a_H + c_H b and D(b) = a_D + c_D b (least squares over bits 8..16) per r.
Then:
  * unconditional (Theorem 2): rho(b) = H(b)/lam_petsche(D(b)); smallest b where the bound
    excludes beating rho (expected trials 1/(2 eps) > sqrt(q)); lean and sharp cutoffs;
  * conditional on a height-ratio certificate rho (Theorem 1): margin at 256 bits for the
    measured rho_G (median and max), and for 4x and 10x those values (Lang-type slack).
"""
import os, sys, json, math
sys.path.insert(0, os.path.dirname(__file__))
from bounds import log2_C_sharp, log2_C_lean, margin, lam_petsche, crossover

OUT = os.path.join(os.path.dirname(__file__), '..', 'out')


def linfit(xs, ys):
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    c = sxy / sxx
    return my - c * mx, c


def main():
    S = json.load(open(os.path.join(OUT, 'summary.json')))
    X = [s for s in S if s['arm'] == 'X' and s.get('h_n')]
    lines = ['# Concrete-size evaluation  [pcc:unverified]', '']
    lines.append('Growth fits (least squares over bits 8..16; H and log|Delta_W| in nats; medians of the height subsample):')
    lines.append('')
    lines.append('| r | H(b) = a + c*b | log|Delta_W|(b) = a + c*b | rho_G median over cells (max) |')
    lines.append('|---|---|---|---|')
    fits = {}
    for r in range(2, 7):
        cells = sorted([s for s in X if s['r'] == r], key=lambda s: s['bits'])
        if len(cells) < 2:
            continue
        b = [s['bits'] for s in cells]
        aH, cH = linfit(b, [s['H_median'] for s in cells])
        aD, cD = linfit(b, [s['logDW_median'] for s in cells])
        rhos = [s['rho_G_median'] for s in cells]
        rmax = max(s['rho_G_max'] for s in cells)
        fits[r] = (aH, cH, aD, cD, max(rhos), rmax)
        lines.append(f'| {r} | {aH:.2f} + {cH:.3f} b | {aD:.1f} + {cD:.2f} b | {max(rhos):.3f} ({rmax:.3f}) |')
    lines.append('')
    lines.append('## Unconditional bound (Theorem 2: Petsche + sigma <= log|Delta|/log 11, Mazur T <= 16, factor 1/2 normalisation slack)')
    lines.append('')
    lines.append('| r | rho at 256 bits | log2 C_sharp at 256 | margin at 256 bits (bits) | smallest b excluded (sharp, T=16) | smallest b excluded (sharp, T=1) | smallest b excluded (lean, T=16) |')
    lines.append('|---|---|---|---|---|---|---|')
    for r, (aH, cH, aD, cD, rmed, rmax) in fits.items():
        Hf = lambda b, aH=aH, cH=cH: max(1.0, aH + cH * b)
        Df = lambda b, aD=aD, cD=cD: max(math.log(11), aD + cD * b)
        rho256 = Hf(256) / lam_petsche(Df(256))
        lc = log2_C_sharp(r, rho256, 16)
        m256 = margin(r, lc, 256)
        b1 = crossover(r, Hf, Df, 'sharp', T=16)
        b2 = crossover(r, Hf, Df, 'sharp', T=1)
        b3 = crossover(r, Hf, Df, 'lean', T=16)
        lines.append(f'| {r} | {rho256:.3e} | {lc:.1f} | {m256:.0f} | {b1} | {b2} | {b3} |')
    lines.append('')
    lines.append('## Conditional bound (Theorem 1 with a height-ratio certificate rho), margin in bits at 256-bit q, T = 1')
    lines.append('')
    lines.append('Positive margin = the bound excludes beating rho at 256 bits (expected xedni trials > sqrt(q)), with that many bits of slack. r = 7..9 use the r = 6 measured rho (the Weierstrass path stops at r = 6).')
    lines.append('')
    lines.append('| r | rho used | margin sharp | margin lean | rho x4: margin sharp | rho x10: margin sharp |')
    lines.append('|---|---|---|---|---|---|')
    rho6 = fits.get(6, (0, 0, 0, 0, 1.0, 1.0))[4]
    for r in range(2, 10):
        rho = fits[r][4] if r in fits else rho6
        ms = margin(r, log2_C_sharp(r, rho), 256)
        ml = margin(r, log2_C_lean(r, rho), 256)
        m4 = margin(r, log2_C_sharp(r, 4 * rho), 256)
        m10 = margin(r, log2_C_sharp(r, 10 * rho), 256)
        lines.append(f'| {r} | {rho:.3f} | {ms:.1f} | {ml:.1f} | {m4:.1f} | {m10:.1f} |')
    txt = '\n'.join(lines) + '\n'
    open(os.path.join(OUT, 'bounds.md'), 'w').write(txt)
    print(txt)


if __name__ == '__main__':
    main()
