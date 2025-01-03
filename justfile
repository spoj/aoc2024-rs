alias w := watch
alias wr := watch-release
alias c := clean
alias r := run
alias rr := run-release
alias b := build-all


watch:
  cargo watch -x run

watch-release:
  cargo watch -x "run -r"

clean:
  cargo clean

run:
  cargo run

run-release:
  cargo run -r

build-all:
  cargo build
  cargo build -r
