/* Dependency shim: compiled with the original library's GNU89 settings. */
#include <stdio.h>
#include "lip.h"
#include "bridge.h"
int nist_lip_read(char *text, nist_lip_number *out) {
    verylong value = 0;
    long result = zsread(text, &value);
    *out = (void *)value;
    return result != 0;
}
void nist_lip_print(nist_lip_number value) { zwriteln((verylong)value); }
long nist_lip_test(nist_lip_number value, int ordinary) {
    return ordinary ? zprime((verylong)value, 8, 2) : zprobprime((verylong)value, 7);
}
void nist_lip_free(nist_lip_number value) { verylong n = (verylong)value; zfree(&n); }
long nist_lip_nbits(void) { return NBITS; }
int nist_lip_double_words_ok(void) {
    union { double d; unsigned long words[2]; } value;
    value.d = 1.0;
    return sizeof(unsigned long) == 4 && value.words[0] == 0 && value.words[1] == 0x3ff00000UL;
}
