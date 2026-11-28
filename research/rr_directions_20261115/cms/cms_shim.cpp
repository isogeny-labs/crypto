// Compiled in place of src/cryptominisat.cpp: includes it unchanged, then adds two helpers that
// reach the private solver config (the public SATSolver API has no setter for the Gauss matrix cap).
#define private public
#include "cryptominisat.h"
#undef private
#include "cryptominisat.cpp"
namespace CMSat {
void shim_set_max_matrices(SATSolver* s, uint32_t n) {
    for (auto* sv : s->data->solvers) sv->conf.gaussconf.max_num_matrices = n;
}
void shim_print_gauss_conf(SATSolver* s) {
    for (auto* sv : s->data->solvers) {
        const auto& g = sv->conf.gaussconf;
        printf("c GAUSSCONF max_num_matrices=%u autodisable=%d max_matrix_rows=%u max_matrix_columns=%u min_gauss_xor_clauses=%u\n",
               g.max_num_matrices, (int)g.autodisable, g.max_matrix_rows, g.max_matrix_columns, g.min_gauss_xor_clauses);
    }
}
}
