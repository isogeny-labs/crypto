# Stored n83 witness replay

Before replay trials, freeze the original `runs/cyclotomic-n83.json` as
input. `verify-n83` rebuilds all167 roots with the generic multiword backend,
checks every stored C2 witness, its distinctness/hash and Frobenius closure.
The original input must pass. `tamper-n83` makes a separate labelled fixture
with the first witness's x coordinate replaced by zero, leaving its root
indices intact. That fixture must be rejected by `verify-n83` with nonzero
exit status; retain its stderr and partial receipt. Do not edit the original.
