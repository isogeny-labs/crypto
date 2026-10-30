// SPDX-License-Identifier: LGPL-3.0-or-later
// Finite predeclared immutable files only. --seal never advises; --advise needs
// separate external admission and an exact sealed index. No file-content
// writes.
#include <array>
#include <cerrno>
#include <fcntl.h>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <openssl/evp.h>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <sys/stat.h>
#include <unistd.h>
namespace fs = std::filesystem;
void require(bool b, const char *why) {
  if (!b)
    throw std::runtime_error(why);
}
std::string identity(const struct stat &s) {
  return std::to_string(s.st_uid) + "\t" + std::to_string(s.st_dev) + "\t" +
         std::to_string(s.st_ino) + "\t" + std::to_string(s.st_size) + "\t" +
         std::to_string(s.st_mtim.tv_sec) + "\t" +
         std::to_string(s.st_mtim.tv_nsec);
}
std::string sha256(int fd) {
  EVP_MD_CTX *ctx = EVP_MD_CTX_new();
  require(ctx != nullptr, "digest context");
  require(EVP_DigestInit_ex(ctx, EVP_sha256(), nullptr) == 1, "digest init");
  std::array<unsigned char, 1048576> buffer{};
  for (;;) {
    const auto got = read(fd, buffer.data(), buffer.size());
    if (got < 0 && errno == EINTR)
      continue;
    require(got >= 0, "digest read");
    if (got == 0)
      break;
    require(EVP_DigestUpdate(ctx, buffer.data(), static_cast<size_t>(got)) == 1,
            "digest update");
  }
  unsigned char hash[EVP_MAX_MD_SIZE];
  unsigned n = 0;
  require(EVP_DigestFinal_ex(ctx, hash, &n) == 1, "digest final");
  EVP_MD_CTX_free(ctx);
  std::ostringstream result;
  result << std::hex << std::setfill('0');
  for (unsigned i = 0; i < n; ++i)
    result << std::setw(2) << static_cast<unsigned>(hash[i]);
  return result.str();
}
struct Entry {
  std::string path, metadata, hash;
};
Entry parse(const std::string &line, bool advice) {
  auto tab = line.find('\t');
  require(tab != std::string::npos, "missing path delimiter");
  Entry e{line.substr(0, tab), line.substr(tab + 1), {}};
  require(e.path.rfind("/workspace/", 0) == 0 &&
              e.path.find("..") == std::string::npos,
          "path outside finite workspace inventory");
  if (advice) {
    tab = e.metadata.rfind('\t');
    require(tab != std::string::npos, "missing digest delimiter");
    e.hash = e.metadata.substr(tab + 1);
    e.metadata.resize(tab);
    require(e.hash.size() == 64 && e.hash.find_first_not_of(
                                       "0123456789abcdef") == std::string::npos,
            "invalid sealed SHA256");
  }
  return e;
}
int main(int argc, char **argv) {
  try {
    require(argc == 4, "--seal|--advise exact-parts-index fresh-output");
    const bool advice = std::string(argv[1]) == "--advise";
    require(advice || std::string(argv[1]) == "--seal", "unknown mode");
    const fs::path out = argv[3];
    require(fs::create_directory(out), "fresh output required; no second pass");
    std::ofstream started(out / "started");
    started << "mode=" << argv[1] << "\nuid=" << getuid()
            << "\nscience_calls=0\n";
    started.close();
    std::ifstream parts(argv[2]);
    require(bool(parts), "index open");
    std::ofstream output_index(out / "eligible-parts.txt");
    std::set<std::string> paths, identities;
    unsigned count = 0, accepted = 0, excluded = 0, failures = 0, part = 0;
    unsigned long long bytes = 0;
    std::string part_path;
    while (std::getline(parts, part_path)) {
      require(!part_path.empty(), "empty index path");
      std::ifstream input(part_path);
      require(bool(input), "part open");
      const std::string suffix = std::to_string(part++);
      std::ofstream records(out / ("records-" + suffix + ".tsv"));
      const auto eligible_path = out / ("eligible-" + suffix + ".tsv");
      std::ofstream eligible(eligible_path);
      bool nonempty = false;
      std::string line;
      while (std::getline(input, line)) {
        if (line.empty())
          continue;
        const auto entry = parse(line, advice);
        require(paths.insert(entry.path).second, "duplicate inventory path");
        ++count;
        int fd = open(entry.path.c_str(),
                      O_RDONLY | O_CLOEXEC | O_NOFOLLOW | O_NOATIME);
        if (fd < 0) {
          records << entry.path << "\texcluded=open_errno_" << errno << '\n';
          ++excluded;
          if (advice)
            ++failures;
          continue;
        }
        try {
          struct stat before{}, after{};
          require(fstat(fd, &before) == 0 && S_ISREG(before.st_mode),
                  "not stable regular file");
          require(before.st_uid == 1000 && identity(before) == entry.metadata,
                  "inventory metadata changed or unowned");
          const auto key = std::to_string(before.st_dev) + ":" +
                           std::to_string(before.st_ino);
          require(identities.insert(key).second, "duplicate inode");
          const auto hash = sha256(fd);
          require(fstat(fd, &after) == 0 && identity(before) == identity(after),
                  "metadata changed during hash");
          if (advice) {
            require(hash == entry.hash, "sealed SHA256 mismatch");
            const int rc = posix_fadvise(fd, 0, 0, POSIX_FADV_DONTNEED);
            struct stat final_stat{};
            const bool stable = fstat(fd, &final_stat) == 0 &&
                                identity(before) == identity(final_stat);
            records << entry.path << "\tsha256=" << hash
                    << "\tfadvise_rc=" << rc << "\tstable_after=" << stable
                    << '\n';
            if (rc || !stable)
              ++failures;
          } else {
            eligible << entry.path << '\t' << identity(before) << '\t' << hash
                     << '\n';
            records << entry.path << "\teligible\t" << identity(before)
                    << "\tsha256=" << hash << '\n';
            nonempty = true;
          }
          ++accepted;
          bytes += static_cast<unsigned long long>(before.st_size);
        } catch (const std::exception &e) {
          records << entry.path << "\texcluded=" << e.what() << '\n';
          ++excluded;
          if (advice)
            ++failures;
        }
        close(fd);
        records.flush();
      }
      require(input.eof(), "part read error");
      if (nonempty)
        output_index << eligible_path.string() << '\n';
    }
    require(parts.eof(), "index read error");
    require(count > 0 && count <= 398, "finite candidate count violation");
    std::ofstream result(out / "result");
    result << "mode=" << argv[1] << "\ncandidates=" << count
           << "\naccepted=" << accepted << "\nexcluded=" << excluded
           << "\nfailures=" << failures << "\naccepted_bytes=" << bytes
           << "\nadvice_performed=" << (advice ? "true" : "false")
           << "\nadvice_is_not_reclamation_proof=true\n";
    std::cout << "candidates=" << count << " accepted=" << accepted
              << " excluded=" << excluded << " failures=" << failures << '\n';
    return failures ? 1 : 0;
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
