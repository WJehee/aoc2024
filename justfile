
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

new day:
    #!/usr/bin/env sh
    filename=$(printf "day%02d" {{day}})
    touch "examples/${filename}.txt"
    touch "puzzle_inputs/${filename}.txt"
    touch "src/${filename}.rs"
    echo "mod ${filename};" >> src/lib.rs

