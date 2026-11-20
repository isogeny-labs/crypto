# Independent complete binary kernel replay
Freeze CHECK62.rs and both accepted kernel certificate hashes before data.
Standalone std-only Rust,pinned existing rustc1.99.1,no Cargo/deps/network.
Parse only exact0/1 entries of full oddMatrix62 and kernelBasis62, preserve
all76mixed and324cubic columns. Independently compute packed-F2 ranks and
full M*K product,require rankM+rankK=columns and rankK=kernelcolumns.
Reject a changed kernel and a changed source row. No CAS calls or new
source-field algebra in Rust; denominator/source/root checks remain native.
Same1200second/4GiB virtual limit independently for compile and execution,
CPU0,opt2/codegen1,one mathjob at a time. Exact marker and exit0 required.
Retain failures and freeze repairs;do not rebuild accepted producers.
