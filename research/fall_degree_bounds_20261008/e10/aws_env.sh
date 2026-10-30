#!/usr/bin/env bash
# Maps the lab-account credentials already in the environment (AWS_ACCESS_KEY_ID2,
# AWS_SECRET_ACCESS_KEY2) onto the names the AWS CLI reads, sets the region, and runs
# the command given as arguments. It prints and stores no credential value.
set -eu
export AWS_ACCESS_KEY_ID="$AWS_ACCESS_KEY_ID2"
export AWS_SECRET_ACCESS_KEY="$AWS_SECRET_ACCESS_KEY2"
export AWS_DEFAULT_REGION=us-east-1
export AWS_REGION=us-east-1
exec "$@"
