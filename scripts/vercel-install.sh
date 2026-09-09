#!/usr/bin/env sh

set -eu

trunk_version="0.21.14"
trunk_sha256="f2b4680cd239693a646a2795e4633c625328d7b2a044fbe749fa3a2fe9e7036b"
trunk_dir=".vercel/cache/trunk/${trunk_version}"
trunk_bin="${trunk_dir}/trunk"

rustup target add wasm32-unknown-unknown

if [ -x "${trunk_bin}" ]; then
    echo "Using cached Trunk ${trunk_version}"
    exit 0
fi

archive_dir="$(mktemp -d)"
trap 'rm -rf "${archive_dir}"' EXIT HUP INT TERM
archive="${archive_dir}/trunk.tar.gz"

curl --fail --location --retry 3 --silent --show-error \
    "https://github.com/trunk-rs/trunk/releases/download/v${trunk_version}/trunk-x86_64-unknown-linux-gnu.tar.gz" \
    --output "${archive}"

echo "${trunk_sha256}  ${archive}" | sha256sum --check --status
mkdir -p "${trunk_dir}"
tar --extract --gzip --file "${archive}" --directory "${trunk_dir}" trunk
chmod +x "${trunk_bin}"

echo "Installed prebuilt Trunk ${trunk_version}"
