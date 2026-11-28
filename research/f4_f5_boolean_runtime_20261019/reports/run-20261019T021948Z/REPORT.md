# Corrected auxiliary profiles

Corrected local batch began with the02:19:48 UTC command receipt. Five alternating before/candidate pairs per arm; complete query Ir plus setup Ir for Semaev. Every output is verified and matching work/witnesses are checked by analyze-profiles.rs. This batch uses iteration1 before the later cutoff/typed policy.

Recorded host: Xeon 6975P-C, shared managed x86-64 Linux host; Valgrind 3.19, simulated AVX2, merged workers and RAYON_NUM_THREADS=1. Counts are instruction evidence only. Local elapsed times, cycles and IPC are not supplied as native measurements. Native cloud results use their own admission and source manifests.

Sources: profiles-corrected/ in the raw archive, corrected-profile-binaries.sha256, profiles-summary.json and run-profiles.sh. The corrected baseline is the original study's exact final executable, SHA256199eb9cf47d10245e105bf9f6bed431b3ec3afc106ba7d891619c317876b3d0e. Retain the earlier failed batch separately. First candidate hot-function annotation is in records/f4-candidate-hot-functions.txt; it is not a new native timing.
