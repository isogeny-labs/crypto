#!/bin/bash
cd "$(dirname "$0")"
./runlog.sh raw/COMMANDS.log "./l06/target/release/l06 gate-enum --p 101 --count 200 --out raw/gate_enum_p101.jsonl 2> raw/gate_enum_p101.log" raw/gate_enum_p101.jsonl
./runlog.sh raw/COMMANDS.log "./l06/target/release/l06 gate-enum --p 257 --count 40 --out raw/gate_enum_p257.jsonl 2> raw/gate_enum_p257.log" raw/gate_enum_p257.jsonl
