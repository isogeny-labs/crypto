"""Figure for research/descent_m3_memory_20261115 (numbers from raw/*.out, *.err; [pcc:unverified])."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

k = [5, 6, 7]
# peak RSS (MB): VmHWM at the end of the call; direct k=7 killed at the 9 GB address-space cap (>= 8551 MB seen)
direct_rss = [219, 7822, 8551]
chain_rss = [669, 1408, 1773]
direct_wall = [1.2, 182.8, None]
chain_wall = [36.2, 142.1, 321.5]
fig, ax = plt.subplots(1, 2, figsize=(9, 3.6))
ax[0].plot(k, direct_rss, "o-", color="#c0392b", label="direct S4 (deg 6, N=3k)")
ax[0].plot([7], [8551], "x", color="#c0392b", ms=12)
ax[0].plot(k, chain_rss, "s-", color="#2471a3", label="chain S3∘S3 (deg 3, N=3k+n)")
ax[0].axhline(9000 / 1.024, ls="--", color="gray", lw=1)
ax[0].text(5.05, 8000, "9 GB cap (direct k=7: OOM at 27 s)", fontsize=8, color="gray")
ax[0].set_xlabel("subspace dimension k (binary-subspace:dimension)")
ax[0].set_ylabel("peak RSS of one F4 call (MB)")
ax[0].set_xticks(k)
ax[0].legend(fontsize=8, loc="center left")
ax[1].semilogy([5, 6], direct_wall[:2], "o-", color="#c0392b", label="direct (k=7: no result)")
ax[1].semilogy(k, chain_wall, "s-", color="#2471a3", label="chain (k=7: F4 matrix cap hit)")
ax[1].set_xlabel("k")
ax[1].set_ylabel("wall of one F4 call (s), 1 thread")
ax[1].set_xticks(k)
ax[1].legend(fontsize=8)
fig.suptitle("descent-algebraic m=3, f4-f2, binary n=21, one target [pcc:unverified]", fontsize=10)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(f"figures/f4_memory_wall.{ext}", dpi=130)
