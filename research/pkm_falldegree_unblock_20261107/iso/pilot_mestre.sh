#!/bin/sh
# Amendment R4 (draft) wrapper: the R4 pilot (f13a690df0 + gaux/aux_order_f13a690df0.patch, musl, SHA-256 705b151b...)
# with --aux-order mestre prepended. Used as BIN by entry_iso.sh and as PILOT by prime_fall_degree_tools gate3, so no
# frozen tool changes and the gate rebuilds isogeny systems at P1 with the exact O(p^1/4) order, not the O(p) count.
exec /usr/local/bin/pkm_tower_pilot_r4 --aux-order mestre "$@"
