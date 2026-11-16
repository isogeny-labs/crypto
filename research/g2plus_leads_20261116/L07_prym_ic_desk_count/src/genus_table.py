from fractions import Fraction

HEADER = ("g\th\tgenus_D\tdim_P\tdominant\tprym_2lp_exp\thyper_g_2lp_exp\trho_prym_exp\t"
          "prym_beats_hyper\tprym_2lp_dec\thyper_g_2lp_dec\trho_prym_dec")


def genus_row(g):
    prym = Fraction(2) - Fraction(2, 2 * g + 1)
    hyp = Fraction(2) - Fraction(2, g)
    rho = Fraction(g, 2)
    return {
        "g": g, "h": g + 1, "genus_D": 2 * g + 1, "dim_P": g,
        "dominant": "true" if g <= 5 else "false",
        "prym_2lp_exp": prym, "hyper_g_2lp_exp": hyp, "rho_prym_exp": rho,
        "prym_beats_hyper": "true" if prym < hyp else "false",
    }


def fmt(fr):
    return "%d/%d" % (fr.numerator, fr.denominator)


def main():
    print(HEADER)
    for g in range(1, 7):
        r = genus_row(g)
        print("\t".join([str(r["g"]), str(r["h"]), str(r["genus_D"]), str(r["dim_P"]),
                         r["dominant"], fmt(r["prym_2lp_exp"]), fmt(r["hyper_g_2lp_exp"]),
                         fmt(r["rho_prym_exp"]), r["prym_beats_hyper"],
                         "%.6f" % float(r["prym_2lp_exp"]),
                         "%.6f" % float(r["hyper_g_2lp_exp"]),
                         "%.6f" % float(r["rho_prym_exp"])]))


if __name__ == "__main__":
    main()
