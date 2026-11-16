import re
P = "/home/user/crypto/research/eigen_seta_rust_20261011/reports/run-20261011T1955Z-final-l1/records/run_output.txt"
txt = open(P).read()
kg = re.search(r"^L1 keygen_s (\S+)", txt, re.M).group(1)
enc, dec = re.search(r"^L1 trial 0 encaps_s (\S+) decaps_s (\S+) ok true", txt, re.M).groups()
res = re.search(r"^L1 RESULT (\d+/\d+) decapsulations correct", txt, re.M).group(1)
print("keygen", kg, "encaps", enc, "decaps", dec, "result", res)
print("CLAIM KeyGen 31.050 s; Encaps 0.3032 s; Decaps 11.464 s; one sample, 1/1 correct")
