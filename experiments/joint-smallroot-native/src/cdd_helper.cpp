// SPDX-License-Identifier: GPL-2.0-only
// Standalone generic exact rational vertex-to-halfspace converter.
// No cuso or Sage-derived source is linked into this separate program.
#include <cstdio>
#include <gmpxx.h>
#include <cddlib/setoper.h>
#define GMPRATIONAL
#include <cddlib/cdd.h>
#include <fstream>
#include <iostream>
#include <stdexcept>
int main(int argc, char **argv) {
  try {
    if (argc != 2)
      throw std::runtime_error("usage cdd-helper vertices-file");
    std::ifstream in(argv[1]);
    long rows = 0, dimension = 0;
    in >> rows >> dimension;
    if (!in || rows <= 0 || dimension <= 0)
      throw std::runtime_error("invalid vertex header");
    dd_set_global_constants();
    auto V = dd_CreateMatrix(rows, dimension + 1);
    V->representation = dd_Generator;
    V->numbtype = dd_Rational;
    for (long i = 0; i < rows; ++i) {
      mpq_set_si(V->matrix[i][0], 1, 1);
      for (long j = 1; j <= dimension; ++j) {
        mpq_class q;
        in >> q;
        if (!in)
          throw std::runtime_error("invalid rational vertex");
        q.canonicalize();
        mpq_set(V->matrix[i][j], q.get_mpq_t());
      }
    }
    std::string extra;
    if (in >> extra)
      throw std::runtime_error("trailing vertex input");
    dd_ErrorType error = dd_NoError;
    auto P = dd_DDMatrix2Poly(V, &error);
    dd_FreeMatrix(V);
    if (error != dd_NoError || !P)
      throw std::runtime_error("cdd error=" + std::to_string(error));
    auto H = dd_CopyInequalities(P);
    dd_FreePolyhedra(P);
    if (!H)
      throw std::runtime_error("missing H representation");
    std::cout << H->rowsize << " " << dimension << "\n";
    for (long i = 0; i < H->rowsize; ++i) {
      std::cout << (set_member(i + 1, H->linset) ? 1 : 0);
      for (long j = 0; j < H->colsize; ++j) {
        mpq_class q(H->matrix[i][j]);
        std::cout << " " << q.get_num() << "/" << q.get_den();
      }
      std::cout << "\n";
    }
    dd_FreeMatrix(H);
    dd_free_global_constants();
    return 0;
  } catch (const std::exception &e) {
    std::cerr << e.what() << "\n";
    return 1;
  }
}
