"""Overlay for the l1c1-cost build (tooling only; no science).

After ../../l1-steps-2-3/build/{fetch.sh,assemble.py} have assembled $SCRATCH/mini23
(unchanged), add the complete-accounting lane's counted linear stage beside the
frozen one and select it with `--features counted`, exactly as
../../descent-walltime/build/assemble.py does:
  default build          zero_dim_la.rs  (frozen, SHA-256 5ae85b34..., Run50's bytes)
  --features counted     zero_dim_la_counted.rs (SHA-256 f0c50f5f..., counted LinOps)
Usage: python3 -I overlay_counted.py SCRATCH
"""
import hashlib, sys

SCR = sys.argv[1]
C = f"{SCR}/mini23/src/cryptanalysis"
COUNTED = ("/home/user/crypto/research/extension_ic_graph_20261115/leads-20261118/"
           "complete-accounting/counted-linear-stage/zero_dim_la.rs")
orig = open(f"{C}/zero_dim_la.rs", "rb").read()
assert hashlib.sha256(orig).hexdigest() == "5ae85b3458c92dd4350604830966b1a83b240e2340f584e976ae2516ac4a841d", "frozen zero_dim_la changed"
counted = open(COUNTED, "rb").read()
assert hashlib.sha256(counted).hexdigest() == "f0c50f5f3cf8c9150932524b45255a2ee0a948f76146c2b3775aa16d33e17ec5", "counted zero_dim_la changed"
open(f"{C}/zero_dim_la_counted.rs", "wb").write(counted)
mod = open(f"{C}/mod.rs").read()
assert mod.count("pub mod zero_dim_la;\n") == 1
mod = mod.replace("pub mod zero_dim_la;\n",
                  "#[cfg(not(feature = \"counted\"))]\npub mod zero_dim_la;\n"
                  "#[cfg(feature = \"counted\")]\n#[path = \"zero_dim_la_counted.rs\"]\npub mod zero_dim_la;\n")
open(f"{C}/mod.rs", "w").write(mod)
print("overlay: counted linear stage behind --features counted")
