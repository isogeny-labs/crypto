# Stage A, attempt 1: lost to a host reboot

Started 2026-10-18 11:50:03Z (`run.sh stage A`). The EC2 host rebooted at about
18:50Z, after about 7 h, before any of the five processes wrote its end line or
`.meta`. These files are the last committed copies (commit `20d85464f`, 17:18Z);
draws that finished between 17:18Z and 18:50Z, if any, were not committed and
are lost. No `.meta` exists for this attempt.

Per PROTOCOL.md (card directory) a host loss is not a cap kill: the stage is
repeated whole with the same flags and binary, and the lost attempt's files are
kept here. The seeds are deterministic per (cell, shape, instance), so the
repeat must reproduce these draws line for line (ignoring `secs`); that is
checked when the repeat ends.
