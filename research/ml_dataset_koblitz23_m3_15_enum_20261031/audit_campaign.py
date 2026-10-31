#!/usr/bin/env python3
"""Independently replay every Koblitz23 restricted archive and export binding."""

import argparse
import collections
import hashlib
import json
import tarfile
from pathlib import Path, PurePosixPath

HERE = Path(__file__).resolve().parent
DEFAULT_ARCHIVES = Path("/mnt/spot-codex-state/tmp/ml-dataset-koblitz23-m3-15-enum-20261031/archives")
METHODS = ("enumerate",)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(path.read_text())


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archives", type=Path, default=DEFAULT_ARCHIVES)
    args = parser.parse_args()
    protocol, plan = read(HERE / "protocol.json"), read(HERE / "plan.json")
    require(protocol["freeze"]["status"] == "frozen" and
            plan["protocol_sha256"] == digest((HERE / "protocol.json").read_bytes()),
            "frozen protocol binding")
    require(plan["field_sizes"] == [23] and plan["families"] == ["koblitz"] and
            plan["arities"] == list(range(3, 16)) and plan["solvers"] == list(METHODS) and
            plan["generators"] == plan["keys"] == 1 and plan["twists"] is True,
            "unexpected fixed plan")
    rows, statuses = [], collections.Counter()
    artifacts_checked = 0
    for index in range(26):
        checkpoint = read(HERE / "checkpoints" / f"shard-{index:02d}.json")
        custody, published = checkpoint["custody"], checkpoint["published"]
        require(checkpoint["index"] == custody["index"] == index and
                custody["plan_file_sha256"] == digest((HERE / "plan.json").read_bytes()) and
                custody["protocol_file_sha256"] == plan["protocol_sha256"] and
                published["archive_fresh_get"] and published["pointer_fresh_get"] and
                published["archive_sha256"] == custody["archive_sha256"],
                f"checkpoint {index} identity or readback")
        archive = args.archives / f"shard-{index:02d}.tar.gz"
        raw = archive.read_bytes()
        require(digest(raw) == custody["archive_sha256"] and
                len(raw) == custody["archive_bytes"], f"archive {index} hash or length")
        prefix = f"shard-{index:02d}/runner/attempt/cells/{index:012d}/"
        with tarfile.open(archive, "r:gz") as tar:
            names = set(tar.getnames())
            require(prefix + "manifest.json" in names and prefix + "public.json" in names,
                    f"archive {index} missing native cell")
            manifest_raw = tar.extractfile(prefix + "manifest.json").read()
            public_raw = tar.extractfile(prefix + "public.json").read()
            manifest, public = json.loads(manifest_raw), json.loads(public_raw)
            for artifact in manifest["artifacts"]:
                relative = PurePosixPath(artifact["path"])
                require(not relative.is_absolute() and ".." not in relative.parts,
                        f"archive {index} unsafe artifact path")
                name = prefix + str(relative)
                require(name in names, f"archive {index} missing artifact {name}")
                data = tar.extractfile(name).read()
                require(len(data) == artifact["bytes"] and digest(data) == artifact["sha256"],
                        f"archive {index} artifact digest {name}")
                artifacts_checked += 1
        method, twist, arity = "enumerate", bool(index % 2), 3 + index // 2
        key, generator = 0, 0
        config = manifest["config"]
        require(manifest["cell_index"] == index and config["family"] == "koblitz" and
                config["n"] == 23 and config["m"] == arity and config["solver"] == method and
                config["twist"] == twist and config["key_index"] == key and
                config["generator_index"] == generator and public["twist"] == twist,
                f"archive {index} axes")
        require(public["field"]["characteristic"] == "2" and
                public["field"]["degree"] == 23 and
                public["curve"]["model"] == "binary-ordinary" and
                public["curve"]["twist_parameter"] == "1" and
                public["curve"]["twist_parameter_absolute_trace"] == 1 and
                int(public["curve"]["a"]) == (int(public["curve"]["base_a"]) ^ int(twist)) and
                public["curve"]["b"] == public["curve"]["base_b"] and
                public["curve"]["b"] == "1" and
                public["symmetry"]["coefficient_field_degree"] == 1 and
                public["symmetry"]["frobenius_power"] == 1 and
                public["symmetry"]["frobenius_nontrivial_on_model"] is True,
                f"archive {index} Koblitz field, twist or symmetry")
        certificate = public["curve"]["order_certificate"]
        if twist:
            require(certificate["method"] == "quadratic-twist-order-v1" and
                    certificate["base_certificate"]["method"] == "koblitz-frobenius-trace-recurrence-v1",
                    f"archive {index} twisted point-count certificate")
        else:
            require(certificate["method"] == "koblitz-frobenius-trace-recurrence-v1",
                    f"archive {index} base point-count certificate")
        status = custody["native_status"]
        require(status["verified_and_exported"] and
                status["cell_spec"] == {"m": arity, "solver": method, "twist": twist,
                                        "generator_index": generator, "key_index": key} and
                status["native_status"] == manifest["outcome"]["status"] and
                status["independently_solved_ecdlp_cells"] == 0 and
                not status["ecdlp_solve_promoted"], f"archive {index} result binding")
        if status["method_artifact_verified"]:
            require(any(a["role"] == "all-visited-pdp-transitions" for a in manifest["artifacts"]),
                    f"archive {index} method artifact")
        statuses[status["native_status"]] += 1
        rows.append({"index": index, "method": method, "m": arity, "twist": twist,
            "generator_index": generator, "key_index": key, "native_status": status["native_status"],
            "archive_sha256": custody["archive_sha256"],
            "manifest_sha256": digest(manifest_raw), "public_sha256": digest(public_raw),
            "s3_uri": custody["s3_uri"]})
    require(all(rows[2*j]["public_sha256"] != rows[2*j+1]["public_sha256"]
                for j in range(13)), "opposite twists must differ at each arity")
    export = read(HERE / "experiment-export.json")
    receipt = read(HERE / "experiment-export-producer-receipt.json")
    flow = export["flow"]
    require(receipt["export_sha256"] == digest((HERE / "experiment-export.json").read_bytes()) and
            receipt["official_checker_exit_code"] == 0 and
            flow["planned"] == flow["attempted"] == 26 and
            flow["completed"] + flow["not_completed"] == 26 and
            flow["interrupted"] == flow["resumed"] == 0 and not flow["excluded"],
            "official export binding")
    result = {"schema": "ml-dataset.koblitz23-m3-15-enum-independent-audit/v1",
        "scope": "26 fixed Koblitz n23 m3..15 cells; exact archive, artifact and native manifest replay",
        "protocol_file_sha256": plan["protocol_sha256"],
        "plan_file_sha256": digest((HERE / "plan.json").read_bytes()),
        "export_sha256": receipt["export_sha256"], "planned": 26, "retained": 26,
        "native_status_counts": dict(sorted(statuses.items())),
        "independent_ecdlp_solves": 0, "artifacts_rehashed": artifacts_checked,
        "twist_counts": {"untwisted": 13, "twisted": 13}, "rows": rows}
    target = HERE / "AUDIT.json"
    encoded = (json.dumps(result, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != encoded:
        raise ValueError("AUDIT.json differs; review source changes before replacing")
    target.write_bytes(encoded)
    print(json.dumps({"retained": 26, "native_status_counts": result["native_status_counts"],
                      "artifacts_rehashed": artifacts_checked}, sort_keys=True))


if __name__ == "__main__":
    main()
