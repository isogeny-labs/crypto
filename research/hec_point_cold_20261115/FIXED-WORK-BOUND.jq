# Post-run exploratory fixed-work diagnostic. Input SUMMARY; --slurpfile cells.
[.cases[] | . as $case |
 ($cells[0][]|select(.g==$case.g and .p==$case.p and .seed==$case.seed and .arm=="point-lazy")) as $r |
 {g,p,seed,ratio_if_setup_free:(.best_ns/($r.total_ns-$r.setup_ns)),
 ratio_if_setup_and_forest_free:(.best_ns/($r.total_ns-$r.setup_ns-$r.forest_ns))}] |
{standing:"Post-run fixed-work conditional diagnostic, not measured speed or universal bound",
 max_setup_free:(map(.ratio_if_setup_free)|max),
 max_setup_forest_free:(map(.ratio_if_setup_and_forest_free)|max),cases:.}
