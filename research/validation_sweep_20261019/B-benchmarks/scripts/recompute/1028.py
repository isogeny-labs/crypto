import re
P = "/home/user/crypto/research/eigen_seta_rust_20261011/reports/run-20261011T1956Z-pke-l1/records/run_output.txt"
txt = open(P).read()
m = re.search(r"^L1 keygen_s (\S+) encrypt_s (\S+) decrypt_s (\S+) pk_bytes (\d+) pke_ct_bytes (\d+) message_bytes (\d+) correct (\w+)$", txt, re.M)
kg, enc, dec, pk, ct, msg, ok = m.groups()
print("keygen", kg, "encrypt", enc, "decrypt", dec, "pke_ct_bytes", ct, "message_bytes", msg, "correct", ok)
print("CLAIM KeyGen 25.044 s; Encrypt 0.3897 s; Decrypt 11.208 s; 1411-byte ciphertext for 39-byte message")
