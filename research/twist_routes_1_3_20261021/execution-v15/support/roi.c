/* Exact documented Callgrind client requests. Operational measurement only. */
#include <valgrind/callgrind.h>
void twist_roi_start(void) { CALLGRIND_START_INSTRUMENTATION; CALLGRIND_ZERO_STATS; }
void twist_roi_stop(void) { CALLGRIND_STOP_INSTRUMENTATION; CALLGRIND_DUMP_STATS; }
