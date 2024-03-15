export DB_USER=scheduler
export DB_PASSWORD=scheduler
export DB_HOST=localhost
export DB_PORT=5432
export DB_NAME=schedulerdb
export RUST_LOG=debug

default:
	echo $(DB_USER)
	cargo run
