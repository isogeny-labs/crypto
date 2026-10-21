"""Sorted-list replay of the exact selected-plus-three-pivot quotient union."""

import hashlib
import json
from bisect import bisect_left
from math import comb
from pathlib import Path

HERE = Path(__file__).resolve().parent
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
selected_paths = [HERE / "type_i_line_guided_collector.json",
                  HERE / "type_i_line_full_new_quotient_points.json"]
fan_paths = [HERE / "type_i_residual_pivot000_three.json",
             HERE / "type_i_residual_pivot001_cross.json",
             HERE / "type_i_residual_pivot002_cross.json"]
replay_path = HERE / "type_i_residual_pivot002_cross_replay.json"
replay = json.loads(replay_path.read_text())
assert replay["receipt_sha256"] == sha(fan_paths[2])
assert replay["fan_sha256"] == [sha(path) for path in fan_paths]
selected_old = json.loads(selected_paths[0].read_text())["base_points_fp"]
selected_new = json.loads(selected_paths[1].read_text())
streams = [[tuple(map(int, point)) for point in selected_old+selected_new]]
for path in fan_paths:
    fan = json.loads(path.read_text())
    streams.append([tuple(map(int, pair)) for row in fan["rows"]
                    for pair in row["projected_residuals"]])
assert len(streams[0]) == 9925
assert [len(stream) for stream in streams[1:]] == [10074, 9942, 10052]

ordered = []
new_by_stream = []
prior_hits_by_stream = []
within_repeats_by_stream = []
for stream in streams:
    previous = set()
    new = hits = within = 0
    for point in stream:
        at = bisect_left(ordered, point)
        if at < len(ordered) and ordered[at] == point:
            if point in previous:
                within += 1
            else:
                hits += 1
        else:
            ordered.insert(at, point)
            new += 1
        previous.add(point)
    new_by_stream.append(new)
    prior_hits_by_stream.append(hits)
    within_repeats_by_stream.append(within)

assert new_by_stream == [9925, 10071, 9939, 10049]
assert prior_hits_by_stream == [0, 2, 2, 2]
assert within_repeats_by_stream == [0, 1, 1, 1]
assert len(ordered) == replay["exact_selected_plus_three_fan_union"] == 39984
capacity = comb(len(ordered)+2, 3)
assert capacity == replay["unordered_three_sum_capacity"] == 10654671159440
density_path = HERE / "type_i_character_density.json"
accepted = int(json.loads(density_path.read_text())[
    "accepted_nonzero_scalar_lower_bound"])
assert capacity * 2**146 < accepted <= capacity * 2**147

out = {
    "status": "independent_sorted_three_pivot_union_replay",
    "new_exact_classes_by_stream": new_by_stream,
    "prior_exact_hits_by_stream": prior_hits_by_stream,
    "within_stream_repeats": within_repeats_by_stream,
    "exact_union": len(ordered),
    "unordered_three_sum_capacity": capacity,
    "ideal_uniform_accepted_query_hit_less_than_two_to_minus": 146,
    "selected_sha256": [sha(path) for path in selected_paths],
    "fan_sha256": [sha(path) for path in fan_paths],
    "replay_sha256": sha(replay_path),
    "density_sha256": sha(density_path),
    "source_sha256": sha(Path(__file__)),
}
path = HERE / "type_i_three_pivot_union_replay.json"
path.write_text(json.dumps(out, indent=2, sort_keys=True)+"\n")
print("THREE_PIVOT_UNION_REPLAY_PASS", len(ordered), capacity, flush=True)
