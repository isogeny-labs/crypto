// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include "exact.hpp"
#include "usage.hpp"
#include <fcntl.h>
#include <filesystem>
#include <fstream>
#include <sys/wait.h>
#include <unistd.h>
namespace jsr {
namespace fs = std::filesystem;
inline void write(const fs::path &p, const std::string &s) {
  std::ofstream o(p);
  require(bool(o), "open " + p.string());
  o << s;
  require(bool(o), "write " + p.string());
}
inline std::string read(const fs::path &p) {
  std::ifstream i(p);
  require(bool(i), "read " + p.string());
  return {std::istreambuf_iterator<char>(i), {}};
}
struct Backend {
  fs::path executable, archive;
  Order order;
  std::vector<std::string> names;
  unsigned serial = 0;
  std::vector<Poly> basis;
  std::string header() const {
    std::string s = "ring R=0,(";
    for (size_t i = 0; i < names.size(); ++i) {
      if (i)
        s += ",";
      s += names[i];
    }
    s += "),Wp(" + estr(order.w) + ");\noption(redSB); option(redTail);\n";
    s += "proc emit(poly f) { while(f!=0) { "
         "print(string(leadcoef(f))+\":\"+string(leadexp(f))); f=f-lead(f); } "
         "print(\"ENDPOLY\"); }\n";
    return s;
  }
  std::vector<Poly> call(const std::string &body, const std::string &phase) {
    fs::create_directories(archive);
    std::string name = std::to_string(serial++) + "-" + phase;
    auto script = archive / (name + ".sing");
    auto out = archive / (name + ".stdout");
    auto err = archive / (name + ".stderr");
    write(script, header() + body + "\nprint(\"SUCCESS-END\"); quit;\n");
    pid_t p = fork();
    require(p >= 0, "fork Singular");
    if (p == 0) {
      int a = open(out.c_str(), O_WRONLY | O_CREAT | O_EXCL, 0600),
          b = open(err.c_str(), O_WRONLY | O_CREAT | O_EXCL, 0600);
      if (a < 0 || b < 0)
        _exit(126);
      dup2(a, 1);
      dup2(b, 2);
      close(a);
      close(b);
      execl(executable.c_str(), executable.c_str(), "-q", script.c_str(),
            static_cast<char *>(nullptr));
      _exit(127);
    }
    const auto waited = wait_child_usage(p);
    write(archive / (name + ".wait-status"),
          waited.reaped ? std::to_string(waited.status) + "\n"
                        : "unavailable\n");
    write(archive / (name + ".cpu-receipt"), wait_usage_fields(waited));
    require(waited.reaped,
            "wait Singular errno=" + std::to_string(waited.error));
    const int st = waited.status;
    require(WIFEXITED(st) && WEXITSTATUS(st) == 0, "Singular process failure");
    std::istringstream in(read(out));
    std::string line, codec;
    std::vector<Poly> v;
    bool done = false;
    while (std::getline(in, line)) {
      line.erase(std::remove_if(line.begin(), line.end(),
                                [](unsigned char c) {
                                  return c == ' ' || c == '\r' || c == '\t';
                                }),
                 line.end());
      if (line == "SUCCESS-END") {
        done = true;
        continue;
      }
      if (line == "ENDPOLY") {
        v.push_back(parse_codec(codec, names.size()));
        codec.clear();
        continue;
      }
      if (line.empty() || line.rfind("//", 0) == 0)
        continue;
      require(line.find(':') != std::string::npos,
              "unexpected Singular stdout: " + line);
      codec += line + "\n";
    }
    require(done && codec.empty(), "incomplete Singular protocol");
    return v;
  }
  std::string ideal_text(const std::vector<Poly> &g,
                         const std::string &name) const {
    std::string s = "ideal " + name + "=";
    if (g.empty())
      s += "0";
    for (size_t i = 0; i < g.size(); ++i) {
      if (i)
        s += ",";
      s += g[i].singular(names, order);
    }
    return s + ";\n";
  }
  std::vector<Poly> groebner(const std::vector<Poly> &generators) {
    auto v = call("LIB \"standard.lib\";\n" + ideal_text(generators, "J") +
                      "ideal G=groebner(J);\nint i; "
                      "for(i=1;i<=size(G);i++){emit(G[i]);}\n",
                  "groebner");
    std::stable_sort(v.begin(), v.end(), [&](auto &a, auto &b) {
      return compare(a, b, order) > 0;
    });
    for (auto &f : v)
      if (!f.zero())
        f = scale(f, 1 / f.terms(order).front().second);
    basis = v;
    return v;
  }
  Poly normal_form(const Poly &f) {
    auto v =
        call(ideal_text(basis, "G") + "attrib(G,\"isSB\",1);\nemit(reduce(" +
                 f.singular(names, order) + ",G));\n",
             "normal-form");
    require(v.size() == 1, "normal form arity");
    return v[0];
  }
};
inline std::vector<Poly> ideal_power(const std::vector<Poly> &generators,
                                     int mu) {
  require(mu >= 0, "negative ideal power");
  std::vector<Poly> out;
  std::function<void(size_t, int, Poly)> go = [&](size_t i, int left, Poly p) {
    if (i == generators.size()) {
      if (left == 0)
        out.push_back(p);
      return;
    }
    for (int k = 0; k <= left; ++k)
      go(i + 1, left - k, p * power(generators[i], static_cast<unsigned>(k)));
  };
  require(!generators.empty(), "empty ideal generators");
  go(0, mu, Poly::constant(generators[0].n, 1));
  return out;
}
} // namespace jsr
