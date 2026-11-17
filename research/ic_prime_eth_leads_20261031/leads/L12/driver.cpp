// L12 driver: CaDiCaL C++ API on inst.cnf + targets.tsv.
// modes: inc = one solver, target as assumptions (learned clauses retained)
//        fresh = new solver per target, same CNF, target as assumptions
//        units = new solver per target, target as unit clauses
// usage: driver MODE inst.cnf targets.tsv NT CONFLICT_CAP > out.tsv
// out: idx kind oracle res(10/20/0) dconf dprop ddec cpu_s
#include <cadical.hpp>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
using namespace std;
static double cpu() { timespec t; clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &t); return t.tv_sec + 1e-9 * t.tv_nsec; }
struct T { string kind; int oracle; long tenum; vector<int> lits; };
static vector<vector<int>> cls;
static void load(CaDiCaL::Solver *s) { for (auto &c : cls) { for (int l : c) s->add(l); s->add(0); } }
int main(int argc, char **argv) {
  string mode = argv[1]; ifstream cf(argv[2]), tf(argv[3]); size_t NT = atol(argv[4]); long cap = atol(argv[5]);
  string line; vector<int> cur;
  while (getline(cf, line)) { if (line.empty() || line[0] == 'p' || line[0] == 'c') continue; istringstream is(line); int l; while (is >> l) { if (l == 0) { cls.push_back(cur); cur.clear(); } else cur.push_back(l); } }
  vector<T> ts;
  while (getline(tf, line)) { if (line.empty() || line[0] == '#') continue; istringstream is(line); T t; string k, o, e, ls; getline(is, k, '\t'); getline(is, o, '\t'); getline(is, e, '\t'); getline(is, ls, '\t'); t.kind = k; t.oracle = atoi(o.c_str()); t.tenum = atol(e.c_str()); istringstream li(ls); int l; while (li >> l) t.lits.push_back(l); ts.push_back(t); }
  if (ts.size() > NT) ts.resize(NT);
  CaDiCaL::Solver *S = nullptr;
  if (mode == "inc") { S = new CaDiCaL::Solver; S->set("quiet", 1); load(S); for (auto &t : ts) for (int l : t.lits) S->freeze(abs(l)); }
  for (size_t i = 0; i < ts.size(); i++) {
    CaDiCaL::Solver *s = S;
    if (mode != "inc") { s = new CaDiCaL::Solver; s->set("quiet", 1); load(s); if (mode == "fresh") for (int l : ts[i].lits) s->freeze(abs(l)); }
    long c0 = s->get_statistic_value("conflicts"), p0 = s->get_statistic_value("propagations"), d0 = s->get_statistic_value("decisions");
    double t0 = cpu();
    if (mode == "units") { for (int l : ts[i].lits) { s->add(l); s->add(0); } }
    else for (int l : ts[i].lits) s->assume(l);
    s->limit("conflicts", cap);
    int r = s->solve();
    double dt = cpu() - t0;
    printf("%zu\t%s\t%d\t%d\t%ld\t%ld\t%ld\t%.4f\n", i, ts[i].kind.c_str(), ts[i].oracle, r, (long)s->get_statistic_value("conflicts") - c0, (long)s->get_statistic_value("propagations") - p0, (long)s->get_statistic_value("decisions") - d0, dt);
    fflush(stdout);
    if (mode != "inc") delete s;
  }
  return 0;
}
