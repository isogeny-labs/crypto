#!/usr/bin/env python3
"""Run a template task card as a fresh instance (receipts are write-once, so each run needs its own card).

usage: [TARGET=<n>] python3 research/volcano_ecdlp_census_followup_20261023/scripts/card_instance.py <template card>

Copies the template to runs/cards/instances/<card_id>-i<NNNN>.yaml with a new card_id, receipt and stdout path
(NNNN = the lowest number whose card file does not exist), replaces the text {TARGET} in a command's env by the
TARGET environment variable (a positive integer, else exit 2), then runs `python3 tools/task_card.py run` on the
instance and exits with its status. Nothing else is changed; the commands, binds and timeouts are the template's.
"""
import os
import re
import sys
from pathlib import Path

import yaml

REPO = Path(__file__).resolve().parents[3]
INST = "research/volcano_ecdlp_census_followup_20261023/runs/cards/instances"


def main(argv):
    if len(argv) != 2:
        print("usage: card_instance.py <template card>", file=sys.stderr)
        return 2
    os.chdir(REPO)
    template = Path(argv[1])
    doc = yaml.safe_load(template.read_text(encoding="utf-8"))
    target = os.environ.get("TARGET", "")
    for command in doc["commands"]:
        env = command.get("env") or {}
        for key, value in env.items():
            if "{TARGET}" in value:
                if not re.fullmatch(r"[1-9][0-9]*", target):
                    print(f"INSTANCE_ERROR: TARGET must be a positive integer, got {target!r}", file=sys.stderr)
                    return 2
                env[key] = value.replace("{TARGET}", target)
    base = doc["card_id"]
    Path(INST).mkdir(parents=True, exist_ok=True)
    n = 1
    while True:
        card_id = f"{base}-i{n:04d}"
        path = Path(INST) / f"{card_id}.yaml"
        try:
            handle = open(path, "x", encoding="utf-8")
            break
        except FileExistsError:
            n += 1
    doc["card_id"] = card_id
    doc["receipt"] = f"{INST}/{card_id}.receipt.json"
    for command in doc["commands"]:
        if command.get("stdout"):
            command["stdout"] = f"{INST}/{card_id}.stdout"
    with handle:
        yaml.safe_dump(doc, handle, sort_keys=False, default_flow_style=False)
    sys.stdout.flush()
    os.execvp("python3", ["python3", "tools/task_card.py", "run", str(path)])


if __name__ == "__main__":
    sys.exit(main(sys.argv))
