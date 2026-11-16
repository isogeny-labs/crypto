# Post-gate descriptive reduction only; not a timing estimator.
[inputs | select(startswith("COUNTS ")) |
 capture("ell=(?<ell>[0-9]+) h=(?<h>[0-9]+) n=(?<n>[0-9]+) seed=(?<seed>[0-9]+) planted=(?<planted>[a-z]+) dense_bytes=(?<dense>[0-9]+) lazy_retained_bytes=(?<lazy>[0-9]+)") |
 .planted = (.planted == "true") |
 .ell |= tonumber | .h |= tonumber | .n |= tonumber | .seed |= tonumber |
 .dense |= tonumber | .lazy |= tonumber |
 . + {retained_ratio:(.dense / .lazy)}] |
{standing:"Synthetic exactness-stream retained capacities; excludes transient maps and allocator metadata; not RSS or speedup",
 streams:length,
 by_h:(group_by(.h) | map({h:.[0].h,n:length,
 min_dense_over_lazy:(map(.retained_ratio)|min),
 max_dense_over_lazy:(map(.retained_ratio)|max)})),
 cases:.}
