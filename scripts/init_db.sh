#!/bin/bash

set -x
set -eo pipefail

if ! [ -x "$(command -v mysql)" ]; then
	echo >&2 'Error: mysql is not installed.'
	exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "Error: sqlx-cli is not installed."
	exit 1
fi

# Check if a custom user has been set, otherwise default to root
DB_USER=${DB_USER}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${DB_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'testing'
DB_NAME="${DB_NAME:=testing}"
# Check if a custom port has been set, otherwise default to '3306'
DB_PORT="${DB_PORT:=3306}"

if [[ -z ${SKIP_DOCKER} ]]; then
	# Launch mariadb using Docker
	docker run -d \
		-e MARIADB_USER=${DB_USER} \
		-e MARIADB_PASSWORD=${DB_PASSWORD} \
		-e MARIADB_DATABASE=${DB_NAME} \
		-e MARIADB_ROOT_PASSWORD=password \
		-p "${DB_PORT}":3306 \
		mariadb
fi

# Keep pinging mysql until it's ready to accept commands
export MARIADB_PASSWORD="${DB_PASSWORD}"
until mysql --user "${DB_USER}" -p"${DB_PASSWORD}" -P ${DB_PORT} --protocol=TCP -e 'quit' "${DB_NAME}"; do
	echo >&2 "MariaDB is still unavailable - sleeping"
	sleep 1
done
echo >&2 "MariaDB is up and running on port ${DB_PORT}!"

export DATABASE_URL=mariadb://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

echo >&2 "MariaDB has been migrated, ready to go!"
