#!/usr/bin/env sh

set -eu

trunk_version="0.21.14"
trunk_target="x86_64-unknown-linux-musl"
trunk_sha256="a67f4054b249fe9acc5fabc25de1aebf19783aca3ad6ff64bf34d7da44d0ea20"
trunk_dir=".vercel/cache/trunk/${trunk_version}/${trunk_target}"
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
    "https://github.com/trunk-rs/trunk/releases/download/v${trunk_version}/trunk-${trunk_target}.tar.gz" \
    --output "${archive}"

echo "${trunk_sha256}  ${archive}" | sha256sum --check --status
mkdir -p "${trunk_dir}"
tar --extract --gzip --file "${archive}" --directory "${trunk_dir}" trunk
chmod +x "${trunk_bin}"

echo "Installed prebuilt Trunk ${trunk_version}"
