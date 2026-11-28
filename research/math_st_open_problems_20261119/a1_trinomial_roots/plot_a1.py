"""Plot recorded A1 summary values (rendering only; computes nothing new).
Inputs: data/summary_bits.csv and data/summary.json written by `a1_trinomial_roots analyze`."""
import csv, json
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt
rows = list(csv.DictReader(open("data/summary_bits.csv")))
summ = json.load(open("data/summary.json"))
bits = [int(r["bits"]) for r in rows]
fig, (a, b) = plt.subplots(1, 2, figsize=(11, 4.2))
a.plot(bits, [float(r["mean_per_prime_max_excess"]) for r in rows], "o-", label="mean over primes of max_d (R-L)")
a.plot(bits, [int(r["max_excess"]) for r in rows], "s--", label="max over primes and d of (R-L)")
a.plot(bits, [float(r["mean_excess"]) for r in rows], "^-", label="mean over all (n,d) of (R-L)")
a.plot(bits, [4 * x for x in bits], ":", color="#c0392b", label="lead threshold 4 log2 n (on R)")
for r in rows:
    a.annotate(f"{r['primes']} primes", (int(r["bits"]), float(r["mean_per_prime_max_excess"])), fontsize=7,
               textcoords="offset points", xytext=(4, -10))
a.set_xlabel("bits of n"); a.set_ylabel("excess R(n,d) - L(n,d)  [roots]")
a.set_title("Excess over the elementary envelope L", fontsize=10); a.legend(fontsize=7); a.set_ylim(0, 70)
h = summ["excess_histogram"]; ks = sorted(int(k) for k in h)
b.bar(ks, [h[str(k)] for k in ks], color="#2e86c1"); b.set_yscale("log")
b.set_xlabel("R - L  [roots]"); b.set_ylabel("number of (n, d) pairs")
b.set_title(f"All {sum(h.values()):,} (n,d) pairs; lead rows: {summ['lead_rows_total']}", fontsize=10)
fig.suptitle("A1 (P2.3): max root count R(n,d) of a+bX+cX^d over F_n, exhaustive in d", fontsize=11)
fig.tight_layout(); fig.savefig("a1_excess.svg")
