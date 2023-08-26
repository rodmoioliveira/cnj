#!/bin/bash

declare TRACE
[[ "${TRACE}" == 1 ]] && set -o xtrace
set -o errexit
set -o nounset
set -o pipefail

bench() {
  hyperfine \
    --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' \
    --warmup 5 \
    --parameter-scan size 0 6 \
    -n 'cnj check [ input_size=10^{size} ]' \
    'for cnj in $(seq 1 $( echo "10 ^ {size}"| bc )); do echo 1234567-38.1011.1.21.3141; done | xargs cnj check' \
    --export-markdown benches/results.md
}

main() {
  bench
}

main
