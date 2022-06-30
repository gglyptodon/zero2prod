#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then 
  echo >&2 "not installed: `psql`. Install with your OS's pacakge manager. E.g. on ubuntu do: "; 
echo >&2 "sudo apt install postgresql-client"
  exit 1 
fi

if ! [ -x "$(command -v sqlx)" ]; then 
  echo >&2 "not installed: `sqlx`. Install via cargo install:";
  echo >&2 "    cargo install sqlx-cli --no-default-features --features native-tls,postgres";
 
  exit 1 
fi



DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"


docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DBE=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d postgres postgres -N 1000

export PGPASSWORD="${DB_PASSWORD}"

echo "psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" "
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q' ; do >&2 echo "Postgres not available yet - sleeping"; sleep 2; done
>&2 echo "Postgres running on port ${DB_PORT}"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
