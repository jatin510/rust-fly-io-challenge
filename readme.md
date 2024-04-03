# Flyio Challenge

- Live reloading: `cargo watch -x 'run'`

- Command :

  - echo: `./maelstrom test -w echo --bin /Users/work/Documents/practice/rust/flyio/target/debug/flyio --node-count 1 --time-limit 10`

  - unique ids : `./maelstrom test -w unique-ids --bin /Users/work/Documents/practice/rust/flyio/target/debug/flyio --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition`

  - broadcast

    - single node: `./maelstrom test -w broadcast --bin /Users/work/Documents/practice/rust/flyio/target/debug/flyio --node-count 1 --time-limit 20 --rate 10`

    - multi node: `./maelstrom test -w broadcast --bin /Users/work/Documents/practice/rust/flyio/target/debug/flyio --node-count 5 --time-limit 20 --rate 10`
