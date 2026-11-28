# Frozen #39 spectral V2 syntax repair

Date 2026-11-27. Conductor T-13054 claimed after a clear path check. The V1 `EC_SPECTRAL_039.rs` and protocol remain frozen. V1 compile exited 1 because the partition loop omitted a semicolon after `solve(label,order,&rows,g,q)`. V1 stderr SHA-256 `5de4cd8b3722194e98007cb8578f7e0b35f29d3e12cfda2d11d67e15c20a7046`, with stdout/exit preserved separately. No V1 experiment ran.

V2 source `EC_SPECTRAL_039_V2.rs` SHA-256 `b15f6166601ad1e27613d8be937abcb9780ce8a77ccacd8e067d815e2fcb818a` changes that one semicolon only. It retains all V1 mathematical inputs, controls, EC witness requirements, spectral threshold/iteration cap and result fields. Input `WS68_NATURAL_PARTIAL.stdout` SHA-256 `922945c9a81568d23a504e2d296ef6c468f8dd47738e9b9e8a303ce12246f2b3`. The V1 protocol contains literature, comparator and cost definitions. Local checkpoint gate previously exited 2 for missing `.resume/RESUME.md`; the user's authorized local bypass remains in effect. No cloud work.

Run once; preserve stdout/stderr/exit. Stop if any EC witness, eigen residual, rank, target or row equation check fails.

`rustc -O research/prime_field_100_ideas_20261123/EC_SPECTRAL_039_V2.rs -o /tmp/ec_spectral_039_v2_20261127.bin`

`timeout 60 /tmp/ec_spectral_039_v2_20261127.bin > research/prime_field_100_ideas_20261123/EC_SPECTRAL_039_V2.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_SPECTRAL_039_V2.stderr.txt`

## Post-run receipt

V2 compiled with unused-helper warnings and the one bounded run exited 0. Stdout SHA-256 `32d155184a39be0bbe94cd1936f0577caa96be3907f35c5b4435d7d7a1ead204`; stderr empty, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`; exit receipt SHA-256 `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa`. All 152 source and 136 completed EC identities passed. All three modular solves had rank 9 and checked target scalar 701.

The 8-vertex co-occurrence graph had total edge weight 65. Jacobi used 82 rotations with maximum eigenpair residual 8.155e-13. The spectral partition [1,2,3,5] cut weight 39 and crossed 39 rows, all target-bearing; fixed [0,1,2,3] also cut 39. The third predeclared balanced partition [0,2,5,7] cut weight 35 and crossed 35 rows. Its source label `seeded_control` is imprecise: this subset was specified literally in the frozen source, not drawn by an RNG. Modular elimination used 3,440 adds/3,530 multiplies/9 inversions for spectral, versus 3,840/3,930/9 for fixed and 3,570/3,660/9 for the third partition. Spectral graph construction and eigensolve add cost and are not converted into those modular operation counts. Cross rows preclude independent block solves on this matrix.
