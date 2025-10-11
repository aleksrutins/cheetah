package platform:
    rattler-build build -r boa --target-platform {{platform}}

publish api_key:
    #!/usr/bin/env bash
    for pkg in $(find output -type f \( -name "*.conda" -o -name "*.tar.bz2" \) ); do
        if ! rattler-build upload prefix -c cheetah "${pkg}" --api-key={{api_key}}; then
        fi
    done
    exit 0

build-and-publish:
    #!/usr/bin/env bash
    if ../cicada/cicada commit '\[publish\]'; then
      for target in linux-64 linux-aarch64 win-64; do
        just package $target
      done
      just publish $(cat ~/.prefix_key)
    fi
