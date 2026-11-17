# Primary N83 direct-subgroup D18 K2048 base

This experiment constructs a smaller, nested factor base on the primary
`a=0` N83 curve. Its source is the independently replayed D18 subgroup-x
orbit census and its control is the retained `public_x_hash`, seed
`2026100801`, K2048 base. Both have 2,048 signed-Frobenius columns and
339,968 point records. The new base takes exactly the first 2,048 rows of
the census's sorted `selected-representatives.csv`; the exporter checks the
entire 16,627-row file against SHA-256
`534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4`
and BLAKE3
`2d5e7b6a5ce1574183d0b236c601f5c2ac5d47008c520b923338fd52e6dc6a44`
before taking that prefix. It also checks the census summary digest.

The exact curve is `icv1-f2m83-tm6151469093347-debefd74`, with subgroup
order `2417851639230796216685689`, cofactor four, and polynomial-basis
modulus low terms `[0,1,2,45]`. This is a factor-base construction study:
the candidate and run IDs remain null until a complete point-decomposition,
relation, matrix, and target pipeline is frozen. The combinatorial
five-summand count `binomial(339968,5)/r = 15.6518808172` is a uniform-target
support model, not an observed relation rate.

## Frozen construction and replay

1. Compile `examples/n83_d18_k2048_export.rs` from a clean committed source
   head. Use the local `rust:1.93.1-bookworm` Linux arm64 image pinned by
   image ID
   `sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.
   The program verifies its own committed bytes before exporting. Record the
   exact Linux binary SHA-256 and image ID in the run receipt.
2. Run `export` in a new output directory outside Git with a 900-second
   process-wall cap and a 4 GiB memory limit, recording exit status, peak RSS,
   host, source commit, executable hash, and input hashes. Keep the complete
   compressed object and manifest even if replay later fails.
3. Run `replay` with the same caps. It checks every representative in the
   subgroup, its Frobenius eigenvalue, all 339,968 point and coefficient
   rows, uniqueness, closure, point-set hash, and compressed/plain hashes.
   A failing replay cannot promote the base.
4. After replay passes, store the content-addressed object and manifest in a
   new S3 prefix under the existing N83 factor-base bucket and download both
   for fresh digest checks. The old U object and D K16627 object are immutable.

The execution container has `--cpus=1 --memory=4g --memory-swap=4g
--network=none` and a read-only bind mount of the committed source. A separate
writable mount holds the export. A host-side 900-second watchdog stops the
container and retains its exit and `docker inspect` memory/OOM record. The
replay uses a fresh container with the same envelope. Compilation is outside
both measured intervals and may use a larger build envelope.
`run_guarded.py` implements that envelope and writes one receipt and raw
stdout/stderr log per phase. Run `export` and then `replay` with the same
`--binary` and `--output` arguments; the output path is an existing empty
directory outside the checkout.

The U control object's compressed BLAKE3 is
`82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851`;
its point-set BLAKE3 is
`d528f5828a05a00add4252212bc909ece481f33093e4074ff7dd8c6a51d3c97e`.
The comparison must state that the factor base is the variable and keep all
other candidate stages, the target corpus, run caps, and resource envelopes
paired. A planted decomposition checks correctness only. Natural relation
yield, useful rank, full matrix and target solve, same-point rho, and both
cold and one-target online times are required to select a runtime winner.

The K16627 U/D m5 panel ended in four `UNKNOWN_budget` natural searches; it
does not supply a relation-rate estimate for this K2048 experiment. The next
panel must retain every timeout and failed arm and apply the Tier 1 host gate
before promoting cloud wall-time differences.
