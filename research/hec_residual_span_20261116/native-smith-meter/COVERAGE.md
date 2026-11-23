# Pic product/inverse endpoint coverage [pcc:unverified]

The original frozen Pic64 source has3raw modular u128 product sites: Fq::mul, Fq::mulu and free mmq. Each receives one disjoint hook. All exponentiation and polynomial helper products reach one of these sites. Fq::invu and free pinv receive one inverse hook each; counted Fq::inv calls invu and is deliberately NOT hooked again. Both Euclidean inverse loops have iteration hooks. The constructor's local Fq objects, smoothness rank checks, normal-form tables, free point search and primality mmq calls therefore reach the same meter; resetting legacy local counters does not reset this thread meter.

Vector indices per modulus:

0 counted_mul;1helper_mulu;2free_mmq;3backend_invu;4free_pinv;5inverse_euclidean_iterations;6powu_calls;7add_calls;8sub_calls;9sub_nc_calls;10neg_calls.

Total executed Pic modular products are0+1+2; inversions are3+4. Other columns are overlapping helper metadata, NOT independent arithmetic to sum. Add/sub/neg call columns do NOT cover every direct modular addition/remainder in the module, and Euclidean iterations are not field multiplications. This repairs Pic product/inverse coverage, not complete machine-instruction/host accounting.

INSTRUMENTATION.json records matching original and hook-stripped SHA256 values. All12hooks are additive telemetry; numerical source is otherwise unchanged. An appended primitive calibration helper is separately excluded from this byte-identity check. The original source and global repository files remain unmodified.

Native source/transport polynomial arithmetic and ell elimination still use their separate16column meter, with raw convolution products and REDCs retained. They do not pass through Pic's endpoints. Model/kernel/trigonal/canonical construction is supplied by frozen prior artifacts in this experiment and is not recomputed or counted as total attack setup. Memory, hash/sort/RNG, allocation, branch, compile and host costs remain outside this field ledger. No total10xclaim follows from the repaired meter alone.
