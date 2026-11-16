# Attempt 1 admission result

Source commit `c57dd5a6d2ff7e862295c216aca319c82fe3c8ee`; command `bash research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-full-fixture-20261116/launch.sh`.

At 2026-11-16T02:47:03Z, Docker returned 125 before starting a container because the full-checkout path was not mounted in the Colima VM. The launcher then attempted an inspect, which found no object, and its cleanup found no container. The outer receipt records child exit 125, inspect exit 1, cleanup exit 0, and clean source. The source and fixture hash gates had passed before Docker. No Rust test ran. See the byte-preserved `.log`, `.child.receipt`, `.outer.receipt`, `.inspect.json`, and `.source.commit` files in this directory.
