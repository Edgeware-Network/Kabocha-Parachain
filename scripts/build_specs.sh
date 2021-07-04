./target/release/kabocha-collator build-spec --chain=kabocha-config > kabocha.json
./target/release/kabocha-collator build-spec --chain=kabocha.json --raw > kabocha.chainspec.json
./target/release/kabocha-collator export-genesis-wasm --chain=kabocha.chainspec.json > genesis-wasm
./target/release/kabocha-collator export-genesis-state --chain=kabocha.chainspec.json --parachain-id 2000 > genesis-state
