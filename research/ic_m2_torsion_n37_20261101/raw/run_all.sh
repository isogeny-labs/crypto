#!/bin/sh
D=$(dirname "$0"); "$D/run_arm.sh" 1; "$D/run_arm.sh" 0
