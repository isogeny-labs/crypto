"""Load the hash-linked edge shards of the frozen overlap-anchor Sage receipt."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
RECEIPT = "type_i_overlap_anchor_relation_fan.json"


def load_overlap_anchor_receipt():
    receipt = json.loads((HERE / RECEIPT).read_text())
    shards = receipt["edge_shards"]
    assert len(shards) == 32
    edges = []
    for index, entry in enumerate(shards):
        first = 81 + index
        name = f"overlap_anchor_edges_{first}.json"
        assert entry["file"] == name
        assert entry["first_endpoint"] == first
        assert entry["last_endpoint"] == first
        data = (HERE / name).read_bytes()
        assert hashlib.sha256(data).hexdigest() == entry["sha256"]
        shard = json.loads(data)
        assert len(shard) == entry["edges"]
        assert all(edge["selected_endpoint"] == first for edge in shard)
        edges.extend(shard)
    assert len(edges) == sum(entry["edges"] for entry in shards)
    assert len(edges) == 1397
    assert len({(edge["query_index"], edge["root_index"], edge["selected_endpoint"])
                for edge in edges}) == len(edges)
    receipt["new_split_edges"] = edges
    return receipt
