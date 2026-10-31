/* Independent public-fixture verification through the PARI C ABI.
 * Build with a system libpari or an explicitly pinned libpari shared object:
 * cc pari_check.c /path/to/libpari.so -Wl,-rpath,/path/to -o /tmp/pari-check
 * /tmp/pari-check < runs/CASE/pari_checks.gp > runs/CASE/pari_checks.jsonl
 * No Python interpreter or wrapper participates in this verification.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef long *GEN;
extern void pari_init(size_t, unsigned long);
extern void pari_close(void);
extern GEN gp_read_str(const char *);
extern char *GENtostr(GEN);
extern void pari_free(void *);
int main(void) {
    char line[16384];
    unsigned long checks = 0;
    pari_init(128UL << 20, 2UL << 20);
    char *version = GENtostr(gp_read_str("version()"));
    fprintf(stderr, "PARI version %s\n", version);
    pari_free(version);
    while (fgets(line, sizeof(line), stdin)) {
        char *result = GENtostr(gp_read_str(line));
        puts(result);
        if (strcmp(result, "[1, 1, 1]") != 0) {
            fprintf(stderr, "verification failed at fixture %lu\n", checks);
            pari_free(result);
            pari_close();
            return 1;
        }
        pari_free(result);
        ++checks;
    }
    fprintf(stderr, "verified %lu curve orders, generator orders and targets\n", checks);
    pari_close();
    return checks ? 0 : 1;
}
