package platform:
    pixi build -t {{platform}} -o output/{{platform}}

publish api_key:
    #!/usr/bin/env bash
    pixi auth login repo.prefix.dev --token {{api_key}}
    for pkg in $(find output -type f \( -name "*.conda" -o -name "*.tar.bz2" \) ); do
        pixi upload https://prefix.dev/api/v1/upload/cheetah "${pkg}" || true
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
