# Independent-host replay of the frozen balanced class unit

Prospective CI wrapper, written before its CI calls. The Rust adapter97b38286,
original13 tests and226/3952/39 coverage remain unchanged. The first local run
13c2b7/50757B report5ea1e1 remain frozen. This is a new host replay of the same
implementation, not independently authored or signed verification. [pcc:unverified]

Copy all actual compiler inputs (Cargo manifest/lock, fixture CSV, full src,
unchanged arithmetic vendor modules and included balanced core) into a fresh
custody directory. `COMPILER_INPUTS.sha256` pins these exact bytes to the tested
source. Do not copy or execute any old producer or full-class suite.

Unlike the local cached-registry run, CI first vendors the four locked registry
packages with Cargo's checksum handling. This is read-only dependency download,
not compilation or a scientific experiment. Retain its entire output and whole
vendored source tree; hash every vendored file before compiling. Use explicit
Cargo source replacement pointing to that directory, so both build and Clippy
are locked/offline and read the guarded vendor bytes. The original local
post-call registry custody is not relabeled as a pre-call guard.

Rust1.99.1/release, one compiler worker/codegen unit and120s per compiler/native
phase, one test worker. Compile without running, then copy a separate chmod555
qualifier and hash it before calls. Run all13 tests with complete traces, strict
all-target no-deps Clippy and format. Check source, copied-source, dependency and
qualifier hashes afterward. Keep every output/exit and entire custody tree on
failure as well as success. Setup/metadata/download failures do not count as
semantic rejection. Any later comparison retains complete raw outputs and may
exclude only the elapsed harness-time field as declared metadata.

All positive/negative witness criteria remain those of PROTOCOL_12.md. CI does
not supply actual Richelot correspondence/dual[2], original LP workloads,
runtime/admission/cost results, canonical trust/PCC/store acceptance or merge
waivers. Paid STOP and locked vault persist. [pcc:unverified]
