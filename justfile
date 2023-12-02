create day:
    cp -r day-template {{day}}

test day:
    cargo test --bin {{day}}

run day:
    cargo run --bin {{day}}
