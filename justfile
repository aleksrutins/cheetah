package platform:
    pixi build -t {{platform}}

publish api_key:
    #!/usr/bin/env bash
    pixi auth login repo.prefix.dev --token {{api_key}}
    for pkg in *.conda; do
        if ! pixi upload https://prefix.dev/api/v1/upload/cheetah "${pkg}"; then
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
