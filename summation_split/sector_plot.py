"""Plot warm exact specialized root-oracle costs; NOT Gröbner solving degree.

    python -m summation_split.sector_plot \
        summation_split/sector_bench_medians.csv \
        summation_split/sector_root_cost_vs_m.svg
"""
import csv
import sys
import matplotlib.pyplot as plt


def draw(csv_path,svg_path):
    rows=list(csv.DictReader(open(csv_path)))
    fig,ax=plt.subplots(figsize=(8.8,5.1))
    for b in (1,4):
        for key,label in (
            ("median_resultant_root_ns","Resultant + factoring"),
            ("median_sector_root_ns","Frobenius sector"),
        ):
            xs,ys=[],[]
            for m in (3,4,5):
                cells=[r for r in rows
                       if int(r["p"])==2147483647
                       and int(r["a"])==1 and int(r["b"])==b
                       and int(r["m"])==m]
                if len(cells)!=1:
                    raise ValueError(f"missing/duplicate {(b,m)}")
                xs.append(m)
                ys.append(float(cells[0][key])/1e6)
            ax.plot(xs,ys,marker="o",linestyle="-" if b==1 else "--",
                    label=f"(a,b)=(1,{b}) — {label}")
    ax.set_yscale("log")
    ax.set_xticks([3,4,5])
    ax.set_xlabel("m input abscissae (specialized)")
    ax.set_ylabel("Median X-root extraction (ms; logarithmic)")
    ax.set_title("p = 2³¹ - 1: specialized root oracles (NOT Gröbner/F4)")
    ax.legend(fontsize=9)
    ax.grid(True,alpha=0.25)
    fig.text(.015,.01,
             "2 timed cases/cell after 50 exact gates; warm curve setup, exploratory only. d_F4 unmeasured.",
             fontsize=8)
    fig.tight_layout(rect=(0,.04,1,1))
    fig.savefig(svg_path)
    plt.close(fig)


if __name__=="__main__":
    draw(sys.argv[1],sys.argv[2])
