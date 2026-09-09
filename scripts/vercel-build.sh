#!/usr/bin/env sh

set -eu

trunk_version="0.21.14"
trunk_target="x86_64-unknown-linux-musl"
trunk_bin=".vercel/cache/trunk/${trunk_version}/${trunk_target}/trunk"
output_dir=".vercel/output"

if [ ! -x "${trunk_bin}" ]; then
    echo "Trunk is missing; run ./scripts/vercel-install.sh first" >&2
    exit 1
fi

rm -rf "${output_dir}"
mkdir -p "${output_dir}/static"

cat > "${output_dir}/config.json" <<'EOF'
{
  "version": 3,
  "cache": [
    ".vercel/cache/cargo-home/registry/cache/**",
    ".vercel/cache/cargo-home/registry/index/**",
    ".vercel/cache/cargo-target/**",
    ".vercel/cache/trunk/**"
  ]
}
EOF

CARGO_HOME="${PWD}/.vercel/cache/cargo-home" \
CARGO_TARGET_DIR="${PWD}/.vercel/cache/cargo-target" \
BEVY_ASSET_PATH="${PWD}/assets" \
NO_COLOR=true \
    "${trunk_bin}" build --release --locked --dist "${output_dir}/static"
