#include <valgrind/callgrind.h>
void trace_phase_dump(const char *phase) { CALLGRIND_DUMP_STATS_AT(phase); }
