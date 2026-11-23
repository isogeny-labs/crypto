/* Strict I/O adapter. The original FreeLIP source bodies remain unchanged. */
#include <ctype.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "bridge.h"
int main(int argc, char **argv) {
    char input[1024], extra[8];
    nist_lip_number n = 0;
    long result;
    if (argc != 2 || (strcmp(argv[1], "probprime") && strcmp(argv[1], "ordinary"))) return 2;
    if (sizeof(long) != 4 || sizeof(double) != 8 || CHAR_BIT != 8 || nist_lip_nbits() != 26 || !nist_lip_double_words_ok()) return 3;
    if (!fgets(input, sizeof(input), stdin) || !strchr(input, '\n')) return 4;
    input[strcspn(input, "\r\n")] = 0;
    if (!input[0] || (input[0] == '0' && input[1])) return 5;
    for (size_t i = 0; input[i]; ++i) if (!isdigit((unsigned char)input[i])) return 6;
    if (scanf("%7s", extra) == 1) return 7;
    if (!nist_lip_read(input, &n)) return 9;
    printf("CONFIG long=%zu double=%zu charbits=%d NBITS=%d DOUBLE_WORDS=low-high\n", sizeof(long), sizeof(double), CHAR_BIT, (int)nist_lip_nbits());
    printf("INPUT "); nist_lip_print(n);
    if (!strcmp(argv[1], "probprime")) result = nist_lip_test(n, 0);
    else result = nist_lip_test(n, 1);
    printf("RESULT %s %ld\n", argv[1], result);
    nist_lip_free(n);
    return (result == 0 || result == 1) ? 0 : 8;
}
