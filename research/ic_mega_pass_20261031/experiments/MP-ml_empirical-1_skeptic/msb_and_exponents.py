import numpy as np, json, math
out = {}
# (1) MSB-of-log filter (M=2): the canonical hard-core predicate. Exact G(2) and |f^(1)|.
for r in (4099, 65537):
    x = np.arange(r); f = (x < r/2).astype(float)
    F = np.fft.fft(f)/r
    conv = np.fft.ifft(np.fft.fft(f)**2).real
    pi_00 = (conv @ f)/r**2 / (f.mean()**2)           # Pr[h(A+B)=0 | h(A)=h(B)=0]
    g = (x >= r/2).astype(float)
    conv01 = np.fft.ifft(np.fft.fft(f)*np.fft.fft(g)).real
    pi_01 = (conv01 @ f)/r**2/(f.mean()*g.mean())
    out[f"msb_r{r}"] = dict(G2=float(2*max(pi_00, 1-pi_00, pi_01, 1-pi_01)), abs_fhat1=float(abs(F[1])), one_over_pi=1/math.pi,
                            null=math.sqrt(0.25/r), bound=float(1+4*abs(F[1:]).max()))
# (2) exponent arithmetic. M = r^{1/m}, G = M^theta (theta<=1 since G<=M).
rows = []
for m in (2,3,4,5,6,8):
    rows.append(dict(m=m, idea_formula_theta_gt=round(2-m/8,3), correct_stated_cost_theta_gt=round(2-m/4,3),
                     tight_cost_theta_gt=round(1.5-m/4,3), klist_theta_gt=0.5,
                     stated_cost_exponent_at_theta1=round((4-2)/m,3), tight_cost_exponent_at_theta1=round((3-2)/m,3)))
out["exponents"] = rows
print(json.dumps(out, indent=1))
json.dump(out, open("RESULTS_msb_exponents.json","w"), indent=1)
