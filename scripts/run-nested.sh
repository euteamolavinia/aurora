#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"
export AURORA_LOG="${AURORA_LOG:-info,aurora_compositor=debug}"

if [ "${AURORA_SOFTWARE_GL:-0}" = "1" ]; then
  export LIBGL_ALWAYS_SOFTWARE=1
  export GALLIUM_DRIVER=llvmpipe
fi

exec cargo run --bin aurora -- "$@"