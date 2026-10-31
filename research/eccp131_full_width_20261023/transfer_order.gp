\\ Exact independent PARI/GP check: two ellcard calls, gcd and parity.
\\ Fixed before execution: one process, 30 s wall, 2048 MiB sampled RSS.
p = 0x48e1d43f293469e33194c43186b3abc0b;
a = 0x41cb121ce2b31f608a76fc8f23d73cb66;
b = 0x2f74f717e8dec90991e5ea9b2ff03da58;
n = 0x48e1d43f293469e317f7ed728f6b8e6f1;
E = ellinit([Mod(a,p),Mod(b,p)]);
Ea = ellinit([Mod(1,p),Mod(301,p)]);
ne = ellcard(E); ma = ellcard(Ea);
if(ne != n, error("published order mismatch"));
if(gcd(ne,ma) != 1, error("orders not coprime"));
if(ma % 256 != 0, error("auxiliary order not divisible by 256"));
print("PUBLISHED_ORDER ",ne);
print("AUXILIARY_ORDER ",ma);
print("GCD ",gcd(ne,ma));
print("TRANSFER_ORDER_OK");
