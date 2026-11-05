#ifndef NIST_FREELIP_BRIDGE_H
#define NIST_FREELIP_BRIDGE_H
typedef void *nist_lip_number;
int nist_lip_read(char *text, nist_lip_number *out);
void nist_lip_print(nist_lip_number value);
long nist_lip_test(nist_lip_number value, int ordinary);
void nist_lip_free(nist_lip_number value);
long nist_lip_nbits(void);
int nist_lip_double_words_ok(void);
#endif
