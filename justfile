
_default:
    just --list

test day="":
    #!/usr/bin/env sh
    if [ "{{day}}" == "" ]; then
        cargo test
    else
        filename=$(printf "day%02d" {{day}})
        cargo test $filename
    fi

