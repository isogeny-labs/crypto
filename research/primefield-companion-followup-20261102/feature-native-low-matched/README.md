# Isolated low-product comparison instrument

This runner compares the preserved instrumented monic backend with the checked
truncated-product backend. Baseline and A/A use feature-native-matched/generated;
candidate uses this directory's generated modules. Six non-polynomial modules are
byte-identical across the two implementations. One shared meter/allocator is used.

The generator retains existing product-helper and schoolbook-update hooks, covering
both the original and truncated leaf loops. Two added hooks cover shifted middle
and high result assembly. Removing the recorded hooks/path edits recovers all seven
original modules byte-for-byte.29 source/dependency files were hashed before builds.

Optimized production and test binaries compile. Two selected software tests pass
(85 inherited methods filtered): triangular leaf product/addition counts at
n=1,2,4,8,12 and exact complete recovery equality on all eight saved targets against
the monic baseline. The leaf contract is n(n+1)/2 contributing coefficient pairs;
it is an instrumentation check, not a comparison result. Description mode executes
no crypto. The preceding uninstrumented candidate's66-method evidence is preserved.

The separate PF-native-low-product-counts-20261103 draft binds this source commit,
production binary and the unchanged strict analyzer. It passes canonical validation
with zero findings. It remains unfrozen: preflight, remaining budget and authoritative
EKS admission are outstanding. No panel or cloud job ran. The original monic panel
and the4563-case cohort remain unchanged.

Output schema and fixed schedule reuse the existing runner. The protocol hash
identifies which backend pair a receipt belongs to. The existing analyzer checks
all records, all baseline/A-A counters, and strict primary reduction on all eight
fixtures. Custody and independent replay remain external gates. New candidate counts
must not be mixed with the original-backend/monic panel.

Source/metadata: main.rs, generate.py, instrumentation.json, source-lock.json,
receipt.json and comparison-protocol.draft.json. Prior method context is the previous
low-product DERIVATION.md and https://flintlib.org/doc/nmod_poly/mul.html.
