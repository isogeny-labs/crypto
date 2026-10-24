#!/usr/bin/env bash
cd "$(dirname "$0")" && sha256sum PROTOCOL.md make_specs.py run_sessions.sh specs/*.json > FROZEN.sha256 && cat FROZEN.sha256
