"""Teaching figures for TUTORIAL.typ.

preimage_tree: the Kummer tower at p = 31, degrees (2, 3), drawn as the tree
of preimages of c = 1. Leaves are the recorded V of ../data/kummer_p31.json;
the middle level {1, 5, 25} is the hand computation shown in the tutorial
(v^2 mod 31 for each leaf). iso_tree: the same shape for the recorded p = 101
isogeny tower (leaves V, root c from ../data/isogeny_p101_x50.json), middle
level unlabelled.
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})


def tree(ax, root, mid, leaves, groups, l1, l2, title):
    ax.axis("off")
    ax.set_xlim(-2.3, len(leaves) - 0.3)
    ax.set_ylim(-0.6, 2.6)
    xs_leaf = list(range(len(leaves)))
    xs_mid = [sum(xs_leaf[i] for i in g) / len(g) for g in groups]
    x_root = sum(xs_mid) / len(xs_mid)
    for xm, g in zip(xs_mid, groups):
        ax.plot([x_root, xm], [2, 1], color="#2c5aa0", lw=1.2)
        for i in g:
            ax.plot([xm, xs_leaf[i]], [1, 0], color="#4a7d2a", lw=1.2)
    def node(x, y, t, fc):
        ax.text(x, y, t, ha="center", va="center", fontsize=10,
                bbox=dict(boxstyle="round,pad=0.3", fc=fc, ec="#333"))
    node(x_root, 2, root, "#fde9d0")
    for xm, t in zip(xs_mid, mid):
        node(xm, 1, t, "#eef3fb")
    for x, t in zip(xs_leaf, leaves):
        node(x, 0, t, "#f3f8ec")
    ax.text(-2.25, 1.5, l2, fontsize=8.5, color="#2c5aa0", ha="left", va="center")
    ax.text(-2.25, 0.5, l1, fontsize=8.5, color="#4a7d2a", ha="left", va="center")
    ax.text(-0.75, 0, "V", fontsize=11, weight="bold", ha="left", va="center")
    ax.set_title(title, fontsize=10)


def main():
    k = json.load(open(os.path.join(HERE, "..", "data", "kummer_p31.json")))
    # Hand step from the tutorial: 1² = 30² = 1, 6² = 25² = 5, 5² = 26² = 25 (mod 31).
    mids = [1, 5, 25]
    leaves = ["1", "30", "6", "25", "5", "26"]
    assert sorted(int(x) for x in leaves) == sorted(k["V"]), "leaves must be the recorded V"
    groups2 = [[0, 1], [2, 3], [4, 5]]

    i = json.load(open(os.path.join(HERE, "..", "data", "isogeny_p101_x50.json")))
    fig, axes = plt.subplots(1, 2, figsize=(10, 3.4))
    tree(axes[0], f"c = {k['c']}", [str(m) for m in mids], leaves, groups2,
         "L₁(x) = x²\n(2-to-1)", "L₂(x) = x³\n(3-to-1)",
         "Kummer tower, p = 31, S = 2·3 = 6")
    tree(axes[1], f"c = {i['c']}", ["·", "·", "·"],
         [str(i["V"][j]) for j in (0, 3, 1, 4, 2, 5)],  # R'+kQ and R'+(k+3)Q share an image
         [[0, 1], [2, 3], [4, 5]], "L₁: 2-isogeny\nx-map", "L₂: 3-isogeny\nx-map",
         "isogeny tower, p = 101, S = 2·3 = 6 (middle level not labelled)")
    fig.tight_layout()
    fig.savefig(os.path.join(HERE, "preimage_tree.svg"), bbox_inches="tight")


if __name__ == "__main__":
    main()
