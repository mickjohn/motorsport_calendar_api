#!/bin/bash

USERNAME="admin"
PASSWORD="qwerty"

DB_PATH="sqlite"
DB_NAME="test.db"

if [[ ! -f "${DB_PATH}/${DB_NAME}" ]];then
  echo "Building database..."
  mkdir "${DB_PATH}"
  DATABASE_URL="${DB_PATH}/${DB_NAME}" diesel database setup

  echo "Adding admin user to database. Username = 'admin', Password = 'qwerty'"

  HASHED_PASS=$(echo -n "${PASSWORD}" | ./target/release/bcrypt_helper --gen)
  echo "INSERT INTO users(user_name, hashed_password) VALUES (\"${USERNAME}\", \"${HASHED_PASS}\");" | sqlite3 sqlite/ms_api.db
  echo "Inserted admin user into database."

  echo "Adding test data"
  sqlite3 "${DB_PATH}/${DB_NAME}" < test_data.sql
  echo "All done, database '${DB_PATH}/${DB_NAME}' built"
else
  echo "DB already exists, not doing anything"
fi
