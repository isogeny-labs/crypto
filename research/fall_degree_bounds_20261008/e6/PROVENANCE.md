# E6 binary provenance

- **Part A** ran on a copy of `hky_last_fall_degree` (`sha256_partA`), the same build E4
  used.
- **Part C and the start of Part B** (cell 3:18:6, started 00:52Z) ran on `hky_grid`,
  sha256 `124f121ae8e85dfcf4ac10251eeeeb19361879987c6cfa4fe7b50a2c8d00e099`.
- At 2026-10-12T00:56:28Z, `hky_grid` was rebuilt in place to add the Part D `control`
  subcommand. The new sha256 is
  `1f2b375eefb741f43a858555ab9d499a593b8ecfe216e3c0cda19f3cefeb7b7a`.
- Part B cells launched after that time used the rebuilt binary. The `newcells` and `sat`
  code paths are unchanged in the diff, which adds only a match arm and two imports, so
  their outputs do not depend on which build ran them.
- Part D used the rebuilt binary.
- 2026-10-12T00:57:25Z: rebuilt again for Part D′ (`--dlast`); sha256 `0ef3a347a2e32dd3cdc6d3bee216743ff2bda692a2e504569b78bc97079679ac`. Part B cells launched after this time and Part D′ used it. The `newcells` path is unchanged.
- 2026-10-18T16:27:55Z: the container restarted mid-Part B (after 3 of 4 m=5 (10,2) draws; m=4 (13,4) not started). Both cells were rerun from scratch with the same binary (sha256 0ef3a347…) into B_5-10-2.rerun.jsonl and B_4-13-4.jsonl; the first three m=5 draws must reproduce exactly.
- 2026-10-18T18:30:47Z: the queued m=4 (13,4) cell was started in parallel with the m=5 rerun's last draw (the wrapper that would have run it afterwards was stopped; the m=5 process continues).
- 2026-10-18: a second container restart killed the m = 4 (13, 4) run before it finished its first draw, leaving `B_4-13-4.jsonl` empty. That empty file was removed. The cell was rerun with `examples/hky_par.rs` (Amendment 7) into `B_4-13-4.par.jsonl`, binary sha256 `49d61e3c1b2f15a40133b5bb46363419f15595cae1bd4736e348c279cf4c7d73`. Before that run, the same build had reproduced the validated engine's `d_F` and closure dimensions on 24 committed draws (`e7/validate_prechunk/`), and all four m = 5 (10, 2) draws of `B_5-10-2.rerun.jsonl`, bit for bit (`e7/validate_prechunk/val_B_5-10-2.jsonl`). The (13, 4) draws took 270–330 s each on 4 threads.
