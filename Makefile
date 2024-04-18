login:
	cargo login
	# please paste the token found on https://crates.io/me below
    # token

config_token:
	code ~/.cargo/credentials.toml
	code ~/.zshrc
	# export CARGO_REGISTRY_TOKEN=token

publish:
	cargo publish --dry-run

# make bench
bench:
	cargo bench

# make bench_report
bench_report:
	open target/criterion/report/index.html