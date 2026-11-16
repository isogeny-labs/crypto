# Relative guard admission correction

The initial resume PID195750 advanced6vertices and stopped cleanly before the
original deadline. Remaining seconds had been calculated before a filesystem
walk; native start+guard was2,264ms later than the original deadline. No actual
deadline overrun occurred. The metadata calculation was corrected, without
changing mathematical budgets, native guards or the original absolute deadline.

The corrected launch computes remaining time after traversal with5sec admission
margin, then verifies actual native start+guard before making a live claim.
PID196273's verified relative guard ends4,098ms before the original deadline.
No new interrupted attempt. Both runtime epochs remain unqualified.

These JSON paths are named by the guide/status/run card and remain loose:
[initial envelope](resume-envelope.json), [initial launch](resumed-launch.json),
[initial process/progress](resumed-verified.json), [calculation](correction.json),
[clean stop/native status](stopped-verified.json). The initial retained raw log
is ROOT/worker-resume-20261018T2313Z.log; scientific6vertices remain in ROOT and
will enter the next stopped checkpoint, never the immutable7,937 archive.
