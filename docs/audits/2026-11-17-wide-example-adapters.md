# Maintained example masks and unit-test harnesses

The wide library build exposed historical example interfaces that still use
64-bit assignments. Current Cargo entry points use complete maintained source
copies, with original file digests retained in
2026-11-17-wide-example-source-custody.json. Original programs, archived inputs,
workflow pins and recorded outputs remain unchanged. [pcc:unverified]

Conversions into the library mask use the existing widening helper. Algorithms
whose assignments or serialized requests are u64 use checked narrowing; they
refuse an unrepresentable variable instead of truncating it. Canonical 128-bit
closure diagnostics retain their full mask. The native JSON worker continues
to accept its original 64-bit request domain, including trace survivors. These
adapters do not enlarge a frozen experiment's admitted domain. [pcc:unverified]

Cargo normally leaves example unit-test harnesses disabled. A local compiler
control confirmed that an invalid test body escapes ordinary all-target Clippy
when test=false and fails that check when test=true. The maintained examples
now enable their existing harnesses, so future all-target checks include those
bodies. Test mains remain distinct from research command entry points; no
experiment is launched by compiling a harness. [pcc:unverified]

The replay-failure supplement follows the standard ecbench auditor's optional
work-counter rule. When retained phases contain work_native, every phase's map
must match. Older phases without those maps keep their original comparison
contract. A changed count or a missing recorded phase is rejected; newly
introduced counters are not written into old records. [pcc:unverified]

The partial-enlargement checkpoint fingerprint retains its fixed sixteen-byte
mask encoding in both feature builds. Its conversion annotation documents that
encoding. Two test-only polynomial clones are replaced with borrowed slices;
a capacity expression uses the equivalent integer ceiling operation. No
performance measurement or scientific admission follows from these edits.
[pcc:unverified]

Both integration Clippy configurations pass. All 42 functional example harnesses pass in each build: 193 default and 204 wide tests, with one existing ignored test per build. Functional runs use release configuration with package optimization level 1; the library suite uses its normal release level 3. Four exact committed runtime fixtures are declared for sparse CI. These are correctness checks, not performance measurements. [pcc:unverified]
