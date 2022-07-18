# Fill Assets storage.
cargo run --release -- master tx assets create -a alice -i 0 -m 1
cargo run --release -- master tx assets create -a alice -i 1 -m 1
cargo run --release -- master tx assets create -a alice -i 2 -m 1
cargo run --release -- master tx assets create -a alice -i 3 -m 1
cargo run --release -- master tx assets create -a alice -i 4 -m 1

# Check Assets storage.

# Runtime upgrade.
cargo run --release -- master tx sudo sudo-unchecked-weight -a alice

# Check Assets storage.
cargo run --release -- develop storage assets asset -k 0
cargo run --release -- develop storage assets asset -k 1
cargo run --release -- develop storage assets asset -k 2
cargo run --release -- develop storage assets asset -k 3
cargo run --release -- develop storage assets asset -k 4

# Check NextFTokenId storage.
cargo run --release -- develop storage deip-fnft next_ftoken_id -k 5