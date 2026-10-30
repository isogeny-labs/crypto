#!/usr/bin/env bash
# Paste this into the cloud environment setup field as:
#   bash /workspace/scripts/setup-cloud-environment.sh
# Installs system math/build tools plus the repository's Conductor and cairn
# binaries. Do not put tokens or credentials in this file.
set -Eeuo pipefail

case "${WORKSPACE_OFFLINE:-0}" in
  0|'') ;;
  1)
    offline_repo="${CRYPTO_REPO_DIR:-$(cd "$(dirname "$0")/.." && pwd)}"
    exec bash "$offline_repo/scripts/private-workspace.sh" check
    ;;
  *) echo "WORKSPACE_OFFLINE must be 0 or 1" >&2; exit 2 ;;
esac

repo_dir="${CRYPTO_REPO_DIR:-/workspace}"
bin_dir="$HOME/.local/bin"
mkdir -p "$bin_dir"
export PATH="$bin_dir:$HOME/.cargo/bin:$PATH"

run_root() {
  if [[ "$(id -u)" -eq 0 ]]; then
    "$@"
  else
    if ! command -v sudo >/dev/null 2>&1; then
      echo "sudo is required to install system packages" >&2
      return 1
    fi
    sudo "$@"
  fi
}

if ! command -v apt-get >/dev/null 2>&1; then
  echo "This setup script requires a Debian or Ubuntu image with apt-get" >&2
  exit 1
fi

packages=(
  build-essential bzip2 ca-certificates clang cmake curl git jq
  libgmp-dev libmpfr-dev libpq-dev libssl-dev pkg-config ripgrep
  python3 python3-pip python3-venv python3-sympy python3-pytest
  pari-gp shellcheck git-lfs
)
if ! command -v go >/dev/null 2>&1; then
  packages+=(golang-go)
fi

missing=()
for package in "${packages[@]}"; do
  if ! dpkg-query -W -f='${Status}' "$package" 2>/dev/null |
       grep -qx 'install ok installed'; then
    missing+=("$package")
  fi
done
if (("${#missing[@]}" > 0)); then
  run_root env DEBIAN_FRONTEND=noninteractive apt-get update
  available=()
  for package in "${missing[@]}"; do
    if apt-cache show "$package" >/dev/null 2>&1; then
      available+=("$package")
    else
      echo "APT package unavailable, skipping: $package" >&2
    fi
  done
  if (("${#available[@]}" > 0)); then
    run_root env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "${available[@]}"
  fi
fi

# Some minimal images do not expose the APT sagemath package. Prefer it when
# available, then fall back to the Sage project's conda-forge installation.
sage_works() {
  command -v sage >/dev/null 2>&1 &&
    sage -c 'assert GF(256).cardinality() == 256' >/dev/null 2>&1
}

if ! sage_works; then
  if run_root env DEBIAN_FRONTEND=noninteractive apt-get update &&
     apt-cache show sagemath >/dev/null 2>&1; then
    if ! run_root env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends sagemath; then
      echo "APT Sage install failed; trying conda-forge" >&2
    fi
  fi
fi

if ! sage_works; then
  case "$(uname -m)" in
    x86_64) mamba_platform=linux-64 ;;
    aarch64|arm64) mamba_platform=linux-aarch64 ;;
    *) echo "No micromamba build selected for architecture $(uname -m)" >&2; exit 1 ;;
  esac
  if ! command -v micromamba >/dev/null 2>&1; then
    (
      download_dir="$(mktemp -d)"
      trap 'rm -rf "$download_dir"' EXIT
      curl -fsSL "https://micro.mamba.pm/api/micromamba/$mamba_platform/latest" -o "$download_dir/micromamba.tar.bz2"
      tar -xjf "$download_dir/micromamba.tar.bz2" -C "$download_dir" bin/micromamba
      install -m 755 "$download_dir/bin/micromamba" "$bin_dir/micromamba"
    )
  fi
  export MAMBA_ROOT_PREFIX="${MAMBA_ROOT_PREFIX:-$HOME/.local/share/micromamba}"
  sage_prefix="$MAMBA_ROOT_PREFIX/envs/sage"
  # The full `sage` metapackage (386 packages, 1.45 GB to download, several GB
  # extracted: openjdk, jmol, gcc, Jupyter) overflows the cloud disk allowance
  # once the repositories are cloned. `sagelib` plus `maxima` provides bin/sage
  # and the libraries this repository uses (156 packages, about 0.65 GB).
  read -r -a sage_packages <<< "${SAGE_CONDA_PACKAGES:-sagelib maxima}"
  mkdir -p "$MAMBA_ROOT_PREFIX"
  free_kb="$(df -Pk "$MAMBA_ROOT_PREFIX" | awk 'NR == 2 {print $4}')"
  need_kb=$(( ${SAGE_MIN_FREE_GB:-4} * 1024 * 1024 ))
  if (( free_kb < need_kb )); then
    echo "Only $(( free_kb / 1024 )) MiB free; SageMath needs ${SAGE_MIN_FREE_GB:-4} GiB. Skipping it." >&2
  else
    if [[ -d "$sage_prefix/conda-meta" ]]; then
      mamba_action=install
    else
      mamba_action=create
    fi
    if micromamba "$mamba_action" -y -p "$sage_prefix" -c conda-forge "${sage_packages[@]}"; then
      # Installed files are hard links into the package cache, so this only
      # frees the downloaded archives.
      micromamba clean -a -y >/dev/null || true
      # Activate the conda environment for each invocation instead of calling
      # its Sage executable through a bare symlink.
      micromamba_path="$(command -v micromamba)"
      sage_wrapper="$(mktemp "$bin_dir/.sage.XXXXXX")"
      printf '#!/usr/bin/env bash\nexport MAMBA_ROOT_PREFIX=%q\nexec %q run -p %q %q "$@"\n' \
        "$MAMBA_ROOT_PREFIX" "$micromamba_path" "$sage_prefix" "$sage_prefix/bin/sage" > "$sage_wrapper"
      chmod 755 "$sage_wrapper"
      mv -f "$sage_wrapper" "$bin_dir/sage"
      hash -r
    else
      # Free the disk, but keep an environment this run did not create.
      echo "micromamba could not install SageMath; removing the partial download" >&2
      rm -rf "$MAMBA_ROOT_PREFIX/pkgs"
      if [[ "$mamba_action" == create ]]; then
        rm -rf "$sage_prefix"
      fi
    fi
  fi
fi

# Sage is optional for a cloud session: without it the remaining tools still
# install. Set SAGE_REQUIRED=1 to fail setup instead.
if ! sage_works; then
  echo "WARNING: SageMath is unavailable or failed its finite-field check." >&2
  if command -v sage >/dev/null 2>&1; then
    sage -c 'assert GF(256).cardinality() == 256' >&2 || true
  fi
  if [[ "${SAGE_REQUIRED:-0}" == 1 ]]; then
    exit 1
  fi
fi

if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs |
    sh -s -- -y --profile minimal
  export PATH="$HOME/.cargo/bin:$PATH"
fi
if command -v rustup >/dev/null 2>&1; then
  rustup component add rustfmt clippy
fi

if ! command -v conductor >/dev/null 2>&1 ||
   ! command -v conductor-mcp >/dev/null 2>&1 ||
   ! command -v conductord >/dev/null 2>&1; then
  GOBIN="$bin_dir" GOFLAGS=-modcacherw go install \
    "github.com/aburan28/conductor/cmd/...@${CONDUCTOR_VERSION:-main}"
fi

if ! command -v cairn >/dev/null 2>&1; then
  cairn_args=(--locked --root "$HOME/.local" --git https://github.com/aburan28/cairn)
  if [[ -n "${CAIRN_REV:-}" ]]; then
    cairn_args+=(--rev "$CAIRN_REV")
  fi
  cargo install "${cairn_args[@]}" cairn
fi

# Login shells launched after setup can find the user-installed binaries.
profile_line='export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"'
touch "$HOME/.profile"
if ! grep -Fqx "$profile_line" "$HOME/.profile"; then
  printf '%s\n' "$profile_line" >> "$HOME/.profile"
fi

if [[ -f "$repo_dir/Cargo.toml" ]]; then
  (cd "$repo_dir" && cargo fetch --locked)
  # The current main branch has unrelated library compile errors. Enable this
  # after those are fixed to install the repository's Rust executables.
  if [[ "${CRYPTO_BUILD_BINS:-0}" == 1 ]]; then
    cargo install --locked --path "$repo_dir" --bins --root "$HOME/.local"
  fi
fi

echo "Installed tools:"
sage --version || echo "sage: not installed"
go version
cargo --version
conductor version
cairn --version
