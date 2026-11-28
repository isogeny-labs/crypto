# Frozen complete-cold cost comparison, fixed genus3/F127 instance

Scientific input: coldbuild13/PUBLIC.txt, same sourceD1/B/ell throughout; no
prior kernel/alpha/QF/HC/lambda read. Candidate regenerates all construction
from input, then actual maps, reduced relations, descent and source/destination
verification. Instrumentation copy of validated coldbuild13; algorithms unchanged.
Controls: nativeFast and nativeHybrid with optimized oracle, seed20260916,
max200000trials; previously measured compact anchor solver, same seed, gate=false.
All four worker modes use one binary and identical public-input validation.

Five sequential blocks, each fresh worker process. Even blocks order
cold,fast,hybrid,compact; odd blocks reverse. No warmup, parallel workers, seed
search, rerun or target substitution. Collector creates write-once run directory
and child input-only subdirectory containing onlyPUBLIC.txt. Stdout/stderr for
all20cells retained. Each child has600second aggregate cap. Nonzero/censored
cells retained; any missing cell invalidates the numerical10xgate.

Primary metric: monotonic internal algorithm-core wallns, beginning before input
parse/validation and ending after final scalar verification. Includes all cold
construction/calibration/scan/relation/solve/descent, plus algorithm log output;
excludes final certificate/measurement metadata emission and process startup.
Secondary parent process wallns includes startup and all output. Both retained,
never mixed. Phase boundaries input,sourceWeil,field/kernels,Weil/normalization,
geometry,points/Jacobian,relations,mapG,mapB,descent,verification. No fake calibrated
field-cost inference from old counters.

Rust collector computes median/min/max core and median processwall per arm,
median phases, and bestNativeOverCold=min(nativeFast,Hybrid,Compact medians)/
medianCold. Threshold10. Require20successful cells with same recovered k; report
numerical gate only. This is one public-input pair repeated for timing, not five
independent curves. Local macOS ARM, L0/unisolated; no cache-flush or CPU-isolation
claim, no statistical confidence/general scaling/novelty/admitted10xclaim.
Genus2/4/full objective remains open regardless of this fixed-case result.

Freeze source/lock/protocol and binary fingerprints, commit and S3checkpoint
before execution. Open bench.record/v1 and run Rust collector under benchrec.
Archive all raw files and store content-addressed inS3; nativeRust reduction only.
Create inspectedPDF/SVG/150dpiPNG, benchmark entry/README row and close record
with measured or censored status, producer unvalidated, no VALIDATION.json.
Existing closedrecords/defaults/leaderboard unchanged; sharedresume untouched.

Baseline provenance: hec-native-ic fast/hybrid; anchor.rs solve_compact from the
existing heldout/zero-core comparisons. Prior cold correctness a6e57ffc2e39:
botharms183049,54kernel/27alpha exact regeneration, no previousobjects loaded.
Current KB review and primaryTian/Oort-Ueno/Fay sources recorded in coldbuild13.
The benchmark tests total cost, not a claim of algorithmic novelty.
