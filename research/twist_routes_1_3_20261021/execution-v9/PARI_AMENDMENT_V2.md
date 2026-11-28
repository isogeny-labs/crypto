# Explicit field initialization before the affected independent replay

The original native rank diagnostic completed and retained all18 systems. Its
GP process returned0 and printed the sentinel, but emitted errors. Strict
admission rejects it; a final marker alone is insufficient. Preserve
`runs/pari-v1` and the unchanged generated `runs/rank-v1/verify.gp`.

The first GP line `ck(c,s)=if(!c,error(s));p=823;` places the p assignment
inside the function definition. The initial matrix is constructed before
calling ck, so its coefficient field is initially symbolic. This is an export
initialization error, not a contradiction in the native ranks or a mathematical
failure of isogeny transport.

Before the second independent run, explicitly preload `p=823;` from
PRELOAD_V2.gp. Pin both script and preload hashes. The main GP file, native
output, all matrix entries, expected ranks, uniqueness flags, points and
acceptance gates remain byte-identical. New protocol-v2 records the actual
two GP attempts and successful reference only after zero unexpected stderr.
No matrix or outcome is selected or replaced. The original reference remains
failed. A separate small GP control checks modular rank and inverse image.
