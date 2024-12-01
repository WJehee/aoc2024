
_default:
    just --lust

run:
    cargo run

runday day:
    #!/usr/bin/env sh
    filename=$(printf "day%02d.zig" {{day}})
    cargo run src/$filename

