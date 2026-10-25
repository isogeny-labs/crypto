import re, os
P = "/home/user/crypto/research/eigen_seta_rust_20261011/reports/run-20261011T1958Z-pke-l3-l5/records/run_output.txt"
txt = open(P).read()
vals = {}
samples = {}
for m in re.finditer(r"^(L3|L5) keygen_s (\S+) encrypt_s (\S+) decrypt_s (\S+) .* correct (\w+)$", txt, re.M):
    set_, kg, enc, dec, ok = m.groups()
    vals[set_] = {"keygen": kg, "encrypt": enc, "decrypt": dec, "correct": ok}
    samples[set_] = samples.get(set_, 0) + 1
for s in ["L3", "L5"]:
    print(s, vals[s], "n_samples=", samples[s])
print("CLAIM L3/L5 Encrypt:", vals["L3"]["encrypt"], "/", vals["L5"]["encrypt"], "s")
print("CLAIM L3/L5 Decrypt:", vals["L3"]["decrypt"], "/", vals["L5"]["decrypt"], "s")
