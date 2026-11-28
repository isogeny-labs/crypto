CC ?= gcc
CFLAGS ?= -O2 -Wall -Wextra
LDLIBS ?= -lflint -lgmp -lmpfr -lm

root_nmod: root_nmod.c
	$(CC) $(CFLAGS) -o $@ $< $(LDLIBS)

.PHONY: clean
clean:
	rm -f root_nmod
