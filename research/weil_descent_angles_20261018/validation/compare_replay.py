"""Exact row comparison of replayed n2_routes cells against the recorded CSVs.

Usage: python3 -I compare_replay.py <repo-root> <replay-root> <selection.json> <out.json>

Each replayed cell's CSV is compared with the recorded CSV restricted to the
same cell.  Rows are compared as multisets of strings on every column except a
trailing `seconds` column (wall clock); control.csv has no such column and is
compared on all columns.  Dump lines are tied to replayed rows by
(bits, [h], instance, target, arm) and their p, ell, N must equal the row's.
"""
import collections
import csv
import hashlib
import io
import json
import sys

RES = "research/weil_descent_angles_20261018/results"

CELLS = [
    # name, run, csv file, cell filter on recorded rows, row-key columns for dump tie
    ("itemF__b24", "itemF-control-run-20261018T2128Z", "control.csv", {"bits": "24"}),
    ("itemF__b20", "itemF-control-run-20261018T2128Z", "control.csv", {"bits": "20"}),
    ("itemOJ__b20", "itemOJ-run-20261018T2132Z", "routes.csv", {"bits": "20"}),
    ("itemOJ__b28", "itemOJ-run-20261018T2132Z", "routes.csv", {"bits": "28"}),
    ("itemOJ__b32", "itemOJ-run-20261018T2132Z", "routes.csv", {"bits": "32"}),
    ("SUPP_itemOJ__b40", "itemOJ-run-20261018T2132Z", "routes.csv", {"bits": "40"}),
    ("H4__b32", "H4-memsweep-run-20261018T2209Z", "memsweep.csv", {"bits": "32"}),
    ("H4__b36", "H4-memsweep-run-20261018T2209Z", "memsweep.csv", {"bits": "36"}),
    ("H2__b32_h1", "H2-glue-run-20261018T2240Z", "glue.csv", {"bits": "32", "h": "1"}),
    ("H2__b36_h1", "H2-glue-run-20261018T2240Z", "glue.csv", {"bits": "36", "h": "1"}),
    ("H2__b40_h4", "H2-glue-run-20261018T2240Z", "glue.csv", {"bits": "40", "h": "4"}),
    ("H1__b32_h96", "H1-hsweep-run-20261018T2240Z", "hsweep.csv", {"bits": "32", "h": "96"}),
    ("H1__b36_h16", "H1-hsweep-run-20261018T2240Z", "hsweep.csv", {"bits": "36", "h": "16"}),
    ("H1__b40_h12", "H1-hsweep-run-20261018T2240Z", "hsweep.csv", {"bits": "40", "h": "12"}),
    ("H6__b44_c2", "H6-large-run-20261018T2210Z", "routes.csv", {"bits": "44"}),
    # supplementary, validator-chosen (not nonce-drawn): full coverage of the remaining cells
    ("SUPP_itemF__b28", "itemF-control-run-20261018T2128Z", "control.csv", {"bits": "28"}),
    ("SUPP_itemF__b32", "itemF-control-run-20261018T2128Z", "control.csv", {"bits": "32"}),
    ("SUPP_itemF__b36", "itemF-control-run-20261018T2128Z", "control.csv", {"bits": "36"}),
    ("SUPP_itemF__b40", "itemF-control-run-20261018T2128Z", "control.csv", {"bits": "40"}),
    ("SUPP_itemOJ__b24", "itemOJ-run-20261018T2132Z", "routes.csv", {"bits": "24"}),
    ("SUPP_itemOJ__b36", "itemOJ-run-20261018T2132Z", "routes.csv", {"bits": "36"}),
    ("SUPP_H2__b28_h1", "H2-glue-run-20261018T2240Z", "glue.csv", {"bits": "28", "h": "1"}),
    ("SUPP_H2__b28_h4", "H2-glue-run-20261018T2240Z", "glue.csv", {"bits": "28", "h": "4"}),
    ("SUPP_H2__b32_h4", "H2-glue-run-20261018T2240Z", "glue.csv", {"bits": "32", "h": "4"}),
    ("SUPP_H2__b36_h4", "H2-glue-run-20261018T2240Z", "glue.csv", {"bits": "36", "h": "4"}),
    ("SUPP_H2__b40_h1", "H2-glue-run-20261018T2240Z", "glue.csv", {"bits": "40", "h": "1"}),
    ("SUPP_H1__b32_rest", "H1-hsweep-run-20261018T2240Z", "hsweep.csv", {"bits": "32", "h": ["4", "8", "12", "16", "24", "32", "48", "64", "128"]}),
    ("SUPP_H1__b36_rest", "H1-hsweep-run-20261018T2240Z", "hsweep.csv", {"bits": "36", "h": ["4", "8", "12", "24", "32", "48", "64", "96", "128"]}),
    ("SUPP_H1__b40_rest", "H1-hsweep-run-20261018T2240Z", "hsweep.csv", {"bits": "40", "h": ["4", "8", "16", "24", "32", "48", "64", "96", "128"]}),
    ("SUPP_H6__b48_c2", "H6-large-run-20261018T2210Z", "routes.csv", {"bits": "48"}),
]


def fmatch(d, filt):
    return all((d[k] in v) if isinstance(v, list) else (d[k] == v) for k, v in filt.items())


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def load(path):
    text = open(path, newline="").read()
    rows = list(csv.reader(io.StringIO(text)))
    return rows[0], rows[1:]


def strip(hdr, row):
    return tuple(row[:-1]) if hdr[-1] == "seconds" else tuple(row)


def arm_label(hdr, r):
    d = dict(zip(hdr, r))
    if "c" in d:  # routes.csv: dump arm = arm + c
        return d["arm"] + d["c"]
    return d["arm"]


def main():
    repo, rroot, selpath, outpath = sys.argv[1:5]
    sel = json.load(open(selpath))
    result = []
    for name, run, fname, filt in CELLS:
        import os
        if not os.path.exists(f"{rroot}/{name}/{fname}"):
            print(f"{name}: not replayed (missing)")
            continue
        rec_path = f"{repo}/{RES}/{run}/{fname}"
        rep_path = f"{rroot}/{name}/{fname}"
        hdr, rec = load(rec_path)
        hdr2, rep = load(rep_path)
        assert hdr == hdr2, (name, hdr, hdr2)
        rec_cell = [r for r in rec if fmatch(dict(zip(hdr, r)), filt)]
        a = collections.Counter(strip(hdr, r) for r in rec_cell)
        b = collections.Counter(strip(hdr, r) for r in rep)
        matched = sum((a & b).values())
        # per selected job
        jobs = []
        for s in ([] if name.startswith("SUPP_") else sel.get(run, [])):
            jd = dict(kv.split("=") for kv in s["job"].split(","))
            if not all(jd.get(k) == v for k, v in filt.items() if k in jd and not isinstance(v, list)):
                continue
            jf = {k: v for k, v in jd.items()}
            ra = [r for r in rec_cell if all(dict(zip(hdr, r)).get(k) == v for k, v in jf.items())]
            rb = [r for r in rep if all(dict(zip(hdr, r)).get(k) == v for k, v in jf.items())]
            ca = collections.Counter(strip(hdr, r) for r in ra)
            cb = collections.Counter(strip(hdr, r) for r in rb)
            jobs.append(dict(job=s["job"], score=s["score"], rows_recorded=len(ra), rows_replayed=len(rb),
                             rows_matched=sum((ca & cb).values()), exact=ca == cb))
        # dump tie: one dump line per replayed row, same p / ell / N
        dump = [json.loads(l) for l in open(f"{rroot}/{name}/dump.jsonl") if l.strip()]
        dkey = collections.Counter()
        dvals = {}
        for d in dump:
            k = (str(d["bits"]), str(d.get("h", "")), str(d["instance"]), str(d["target"]), d["arm"],
                 str(d.get("theta_milli", "")))
            dkey[k] += 1
            dvals[k] = d
        tie_ok = True
        tie_n = 0
        for r in rep:
            rd = dict(zip(hdr, r))
            th = ""
            if "theta" in rd:
                th = str(int(float(rd["theta"]) * 1000))
            k = (rd["bits"], rd.get("h", "") if fname in ("hsweep.csv", "memsweep.csv", "glue.csv") else "",
                 rd["instance"], rd["target"], arm_label(hdr, r), th)
            d = dvals.get(k)
            if d is None or dkey[k] != 1:
                tie_ok = False
                continue
            for col in ("p", "ell", "N"):
                if col in rd and str(d[col]) != rd[col]:
                    tie_ok = False
            # verified column consistent with k_found == k
            if (rd["verified"] == "1") != (d["k_found"] == d["k"]):
                tie_ok = False
            tie_n += 1
        result.append(dict(
            cell=name, nonce_drawn=not name.startswith("SUPP_"), run=f"{RES}/{run}", csv=fname, cell_filter=filt,
            recorded_csv_sha256=sha(rec_path), replay_csv_sha256=sha(rep_path),
            rows_recorded_in_cell=len(rec_cell), rows_replayed=len(rep), rows_matched=matched,
            exact_match=(a == b and len(rec_cell) == len(rep)),
            compared_columns="all" if hdr[-1] != "seconds" else "all but trailing seconds",
            dump_lines=len(dump), dump_rows_tied=tie_n, dump_tie_ok=tie_ok and tie_n == len(rep) == len(dump),
            selected_jobs=jobs))
        print(f"{name}: recorded {len(rec_cell)} replayed {len(rep)} matched {matched} exact {a == b} "
              f"dump {len(dump)} tied {tie_n} tie_ok {result[-1]['dump_tie_ok']} jobs "
              + "; ".join(f"{j['job']} {j['rows_matched']}/{j['rows_recorded']}" for j in jobs))
    json.dump(result, open(outpath, "w"), indent=1)


if __name__ == "__main__":
    main()
