"""Compare complete six-summand, single-target runs of two compiled solvers."""

import argparse
import json
import os
import statistics
import subprocess
from pathlib import Path


def run(binary: Path, seed: int, kind: str, direct_first: bool) -> dict:
    env = os.environ.copy()
    if direct_first:
        env["S3_DIRECT_FIRST"] = "1"
    else:
        env.pop("S3_DIRECT_FIRST", None)
    env.pop("S3_DISABLE_RESULTANT_STEP_CACHE", None)
    env.pop("S3_CACHE_RESULTANT_STEPS", None)
    env.pop("S3_DISABLE_TARGET_SPECIALIZATION", None)
    env.pop("S3_DISABLE_COMPOSED_FINAL", None)
    env.pop("S3_COMPOSED_FINAL", None)
    env.pop("S3_PROFILE_RESULTANT", None)
    result = subprocess.run(
        [str(binary.resolve()), "larger-six", str(seed), kind],
        check=True,
        capture_output=True,
        text=True,
        env=env,
    )
    return json.loads(result.stdout)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("baseline", type=Path)
    parser.add_argument("candidate", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--seed", type=int, default=11781661)
    parser.add_argument("--seed-count", type=int, default=1)
    parser.add_argument("--repetitions", type=int, default=9)
    args = parser.parse_args()
    rows = []
    for seed in range(args.seed, args.seed + args.seed_count):
        for rep in range(args.repetitions):
            for kind in ("planted", "unplanted"):
                baseline_first = (rep + seed - args.seed) % 2 == 0
                direct_first = rep % 2 == 0
                cases = [args.baseline, args.candidate]
                if not baseline_first:
                    cases.reverse()
                outputs = {binary: run(binary, seed, kind, direct_first) for binary in cases}
                baseline, candidate = outputs[args.baseline], outputs[args.candidate]
                for field in baseline.keys() | candidate.keys():
                    if not field.endswith("_seconds"):
                        assert baseline[field] == candidate[field], (seed, field, baseline[field], candidate[field])
                rows.append(
                    dict(
                        seed=seed,
                        kind=kind,
                        rep=rep,
                        baseline_first=baseline_first,
                        roots=baseline["roots"],
                        top_coefficient=baseline["top_coefficient"],
                        baseline_total_seconds=baseline["direct_total_seconds"],
                        candidate_total_seconds=candidate["direct_total_seconds"],
                        baseline_indexed_seconds=baseline["indexed_total_seconds"],
                        candidate_indexed_seconds=candidate["indexed_total_seconds"],
                    )
                )
    args.output.write_text("".join(json.dumps(row, separators=(",", ":")) + "\n" for row in rows))
    for kind in ("planted", "unplanted"):
        sample = [row for row in rows if row["kind"] == kind]
        before = statistics.median(row["baseline_total_seconds"] for row in sample)
        after = statistics.median(row["candidate_total_seconds"] for row in sample)
        paired = statistics.median(
            row["baseline_total_seconds"] / row["candidate_total_seconds"] for row in sample
        )
        print(f"{kind}: {before:.3f}s -> {after:.3f}s, median paired ratio {paired:.2f}x")


if __name__ == "__main__":
    main()
