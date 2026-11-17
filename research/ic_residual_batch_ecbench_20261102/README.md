# ic.residual_batch vs rho.multitarget_pooled (ecbench, p36)

Parity rule (frozen): paired res/pool 95% interval contains 1 and lies in [0.9, 1.1] at w = 0, 0.1, 1.
- L = 64: 1.0077 [0.9811, 1.0350], **holds**.
- L = 16: 1.0465 [0.9886, 1.1078], **fails** (upper edge above 1.1).
Per target, both batch methods are 3.3-3.4x (L=16) and 6.6-6.7x (L=64) cheaper than single-target rho; the matched reference is multi-target rho.
320/320 verified; replay audit OK. Local L0; L2 replay owed. Void attempts 1-2 kept (Amendments 1-2). Raw records in the artifact store.
Reproduce: `ecbench run --spec specs-local/E-p36.json --out sessions/E-p36 --allow-busy --quiet`, then `scripts/analyse.sh`.
