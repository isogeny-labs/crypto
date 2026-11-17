#set page(paper: "a4", margin: 15mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#heading(level: 1)[Corrected auxiliary instruction profiles]
#text("Corrected local batch began with the02:19:48 UTC command receipt. Five alternating before/candidate pairs per arm; complete query Ir plus setup Ir for Semaev. Every output is verified and matching work/witnesses are checked by analyze-profiles.rs. This batch uses iteration1 before the later cutoff/typed policy.")


#text("Recorded host: Xeon 6975P-C, shared managed x86-64 Linux host; Valgrind 3.19, simulated AVX2, merged workers and RAYON_NUM_THREADS=1. Counts are instruction evidence only. Local elapsed times, cycles and IPC are not supplied as native measurements. Native cloud results use their own admission and source manifests.")


#text("Sources: profiles-corrected/ in the raw archive, corrected-profile-binaries.sha256, profiles-summary.json and run-profiles.sh. The corrected baseline is the original study's exact final executable, SHA256199eb9cf47d10245e105bf9f6bed431b3ec3afc106ba7d891619c317876b3d0e. Retain the earlier failed batch separately. First candidate hot-function annotation is in records/f4-candidate-hot-functions.txt; it is not a new native timing.")


#image("../../auxiliary-instructions.svg", width: 100%)
#pagebreak(weak: true)
#heading(level: 2)[Five pairs; medians and derived reduction from Rust]
#table(columns: 4, inset: 4pt, text("Arm"), text("Before Ir"), text("Candidate Ir"), text("Reduction %"), text("matrix-f4"), text("491230592.0"), text("474039407.0"), text("3.4996"), text("matrix-f5"), text("503663125.0"), text("486586404.0"), text("3.3905"), text("generalized-native"), text("88153384.0"), text("30814595.0"), text("65.0443"), text("rootfind-mitm"), text("677938605.0"), text("264759899.0"), text("60.9463"))