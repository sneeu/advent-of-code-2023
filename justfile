create day:
    cp -r day-template {{day}}

test-all:
    cargo test

test day:
    cargo test --bin {{day}}

run day:
    cargo run --bin {{day}}
