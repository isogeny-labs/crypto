#define _GNU_SOURCE
#include <spawn.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>
extern char **environ;
static double now(void) { struct timespec t; clock_gettime(CLOCK_MONOTONIC, &t); return t.tv_sec * 1e6 + t.tv_nsec * 1e-3; }
static int cmp(const void *a, const void *b) { double x = *(double *)a, y = *(double *)b; return (x > y) - (x < y); }
int main(int argc, char **argv) {
    int n = 200, mode = argc > 1 ? atoi(argv[1]) : 0; double v[200];
    char *args[] = {"/bin/true", NULL};
    for (int i = 0; i < n; i++) {
        double t0 = now(); pid_t p; int st;
        if (mode == 0) posix_spawn(&p, args[0], NULL, NULL, args, environ);
        else { p = fork(); if (!p) { execv(args[0], args); _exit(127); } }
        waitpid(p, &st, 0); v[i] = now() - t0;
    }
    qsort(v, n, sizeof v[0], cmp);
    printf("%s /bin/true: min %.0f us, median %.0f us, p90 %.0f us (n=%d)\n", mode ? "fork+exec" : "posix_spawn", v[0], v[n / 2], v[n * 9 / 10], n);
    return 0;
}
