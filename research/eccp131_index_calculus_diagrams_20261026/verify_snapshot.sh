#!/bin/sh
set -eu
artifact_directory=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$artifact_directory"
sha256sum -c SNAPSHOT.sha256
