# Named PARI stack-growth sites, version 2

The p=751 degree-three replay completed but the conservative gate rejected
the same bounded stack-growth warning attributed to `Mod`, rather than an
arithmetic operator. Preserve that raw session and rejection. Recognize a
nonempty ASCII PARI function/operator name followed by exactly
`: Warning: increasing stack size to N.`, with the original decimal byte
bound. This accepts no other warning/error text or appended material.
Retain oversized/mixed-error negative controls and add this `Mod` case.
Re-adjudicate unchanged sessions into fresh directories. No mathematical
program, result, source field or resource cap changes.
