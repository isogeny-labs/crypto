#!/bin/bash
set -euo pipefail
cd /var/tmp/pencil-musl-private-20261031/musl-1.2.6
./configure --prefix=/var/tmp/pencil-musl-private-20261031/prefix --syslibdir=/var/tmp/pencil-musl-private-20261031/prefix/lib --disable-shared CC=/usr/bin/gcc 'CFLAGS=-O2 -march=x86-64 -mtune=generic'
make -j2
make -j2 install
