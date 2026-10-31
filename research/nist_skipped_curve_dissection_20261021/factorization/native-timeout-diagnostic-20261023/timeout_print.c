#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    uint32_t timeout = (uint32_t)strtoul(argv[1], NULL, 10);
#if FORMAT_SITE == 1
    int tid = 1;
#if FIX_TIMEOUT_PRINT
    printf("nfs: thread %d halting, nfs timeout of %u sec exceeded\n", tid, (unsigned)timeout);
#else
    printf("nfs: thread %d halting, nfs timeout of %1.4sec exceeded\n", tid, timeout);
#endif
#else
    double t_time = 3601.0;
#if FIX_TIMEOUT_PRINT
    printf("nfs: elapsed time of %6.4f seconds exceeds specified "
           "nfs timeout of %u sec, exiting\n", t_time, (unsigned)timeout);
#else
    printf("nfs: elapsed time of %6.4f seconds exceeds specified "
           "nfs timeout of %1.4f sec, exiting\n", t_time, timeout);
#endif
#endif
    return 0;
}
