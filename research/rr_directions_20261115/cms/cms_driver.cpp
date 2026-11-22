// Minimal command-line driver around the CryptoMiniSat 5.11.21 library (pycryptosat sdist).
// The sdist ships no main.cpp; this driver stands in for cryptominisat5.cpp: it reads
// DIMACS+XOR with the library's own DimacsParser and solves with SATSolver.
//   cms_driver [--verb N] [--maxtime S] [--gauss 0|1] [--seed N] FILE
// --gauss 0 sets gaussconf.max_num_matrices = 0 (same field as upstream --maxnummatrices 0).
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <string>
#include <vector>
#include <chrono>
#include "cryptominisat.h"
#include "dimacsparser.h"
#include "streambuffer.h"
namespace CMSat { void shim_set_max_matrices(SATSolver* s, uint32_t n); void shim_print_gauss_conf(SATSolver* s); }
using namespace CMSat;
int main(int argc, char** argv) {
    int verb = 1; double maxtime = 600; int gauss = 1; unsigned seed = 0; const char* file = nullptr;
    for (int i = 1; i < argc; i++) {
        if (!strcmp(argv[i], "--verb")) verb = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--maxtime")) maxtime = atof(argv[++i]);
        else if (!strcmp(argv[i], "--gauss")) gauss = atoi(argv[++i]);
        else if (!strcmp(argv[i], "--seed")) seed = atoi(argv[++i]);
        else file = argv[i];
    }
    if (!file) { fprintf(stderr, "usage: cms_driver [--verb N] [--maxtime S] [--gauss 0|1] FILE\n"); return 2; }
    SATSolver solver;
    solver.set_num_threads(1);
    solver.set_verbosity(verb);
    solver.set_seed(seed);
    if (!gauss) shim_set_max_matrices(&solver, 0);
    shim_print_gauss_conf(&solver);
    FILE* in = fopen(file, "rb");
    if (!in) { perror("open"); return 2; }
    DimacsParser<StreamBuffer<FILE*, FN>, SATSolver> parser(&solver, nullptr, verb);
    if (!parser.parse_DIMACS(in, false)) { printf("c PARSE ERROR\n"); return 2; }
    fclose(in);
    solver.set_max_time(maxtime);
    auto t0 = std::chrono::steady_clock::now();
    struct timespec c0, c1; clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &c0);
    lbool ret = solver.solve();
    clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &c1);
    double wall = std::chrono::duration<double>(std::chrono::steady_clock::now() - t0).count();
    double cpu = (c1.tv_sec - c0.tv_sec) + (c1.tv_nsec - c0.tv_nsec) * 1e-9;
    if (verb >= 1) solver.print_stats();
    printf("c DRIVER conflicts: %llu\n", (unsigned long long)solver.get_sum_conflicts());
    printf("c DRIVER decisions: %llu\n", (unsigned long long)solver.get_sum_decisions());
    printf("c DRIVER propagations: %llu\n", (unsigned long long)solver.get_sum_propagations());
    printf("c DRIVER solve_cpu_s: %.6f\n", cpu);
    printf("c DRIVER solve_wall_s: %.6f\n", wall);
    if (ret == l_True) {
        printf("s SATISFIABLE\nv");
        const auto& m = solver.get_model();
        for (size_t i = 0; i < m.size(); i++) if (m[i] != l_Undef) printf(" %s%zu", m[i] == l_True ? "" : "-", i + 1);
        printf(" 0\n");
        return 10;
    } else if (ret == l_False) { printf("s UNSATISFIABLE\n"); return 20; }
    printf("s INDETERMINATE\n"); return 0;
}
