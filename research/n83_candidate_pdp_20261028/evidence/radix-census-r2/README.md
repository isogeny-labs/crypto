# Radix census data

All3010 partitions of27 are indexed by summary.json with immutable raw case hashes. Exact-cases.json contains35 full native certificates; the remaining2975 retain rigorous lower bounds in the raw per-case records. The candidate interval and actual prime factor base are unchanged. The compact product-range equation representation is documented in RADIX_CENSUS_FORMAT.json.

Restore the raw S3prefix in custody.json to a fresh directory, run sha256sum -c manifest.sha256 there, set instrument executable permission, then run ./instrument verify census. This recomputes all3010 records, source/base binding and the primality certificate and compares native payload bytes.

The original EKS attempt remains in ../radix-census-r1, including its frozen protocol accounting mismatch and failed export check. Protocol v2 corrects the execution unit to two whole-census phases before any arithmetic data. The two phases are deterministic producer/replay, not independent algorithms or performance arms.
