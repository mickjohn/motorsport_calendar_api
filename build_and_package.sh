#!/bin/bash
set -e

MAJOR_VERSION="1"
MINOR_VERSION="$(($(cat minor_version.txt) + 1))"
echo $MINOR_VERSION > minor_version.txt
VERSION="${MAJOR_VERSION}.${MINOR_VERSION}"

NAME="mscapi"
FULL_NAME="motorsport_calendar_api"

WORKING_DIR="/home/mick/Programs/rust/motorsport_calendar_api/PACKAGE"

if [ -d "$WORKING_DIR" ];then
  echo "$WORKING_DIR already exists."
  echo "Delete it? y/n"
  read DELETE_DIR
  if [ "${DELETE_DIR}x" = "yx" ];then
    echo "Deleting $WORKING_DIR"
    rm -rf "${WORKING_DIR}"
  else
    exit 1
  fi
fi

echo "Creating files..."

mkdir $WORKING_DIR
cd $WORKING_DIR

PACKAGE="${NAME}_${VERSION}"
mkdir "${PACKAGE}"
cd "$PACKAGE"

CONF_DIR="etc/${FULL_NAME}"
mkdir -p "${CONF_DIR}"
echo "---
database_url: "/var/lib/motorsport_calendar_api/database/ms_api.db"" > "${CONF_DIR}/conf.yml"
cp "../../log4rs.yml" "${CONF_DIR}"

DATA_DIR="var/lib/${FULL_NAME}/database"
mkdir -p "${DATA_DIR}"
cp "../../sqlite/ms_api.db" "${DATA_DIR}"

BIN_DIR="usr/local/bin/"
mkdir -p "${BIN_DIR}"
cp "../../target/release/motorsport_calendar_api" "${BIN_DIR}"

SERVICE_DIR="lib/systemd/system/"
mkdir -p "${SERVICE_DIR}"
echo "[Unit]
Description=${FULL_NAME}
[Service]
Type=simple
ExecStart=/${BIN_DIR}/${FULL_NAME} -c /${CONF_DIR}/conf.yml -l /${CONF_DIR}/log4rs.yml
[Install]
WantedBy=multi-user.target" > "${SERVICE_DIR}${FULL_NAME}d.service"

CONTROL_DIR="DEBIAN/"
mkdir -p "${CONTROL_DIR}"
chmod -R 0755 "${CONTROL_DIR}"
echo "Package: ${NAME}
Version: ${VERSION}
Section: base
Priority: optional
Architecture: i386
Depends: sqlite3
Maintainer: Your Name <you@email.com>
Description: Hello World
 When you need some sunshine, just run this
 small program!" > "${CONTROL_DIR}control"

echo "Building package..."
cd ..
dpkg-deb --build "${PACKAGE}"

echo "Moving package up a directory..."
mv "${PACKAGE}.deb" ..

echo "Deleting ${WORKING_DIR}"
cd ..
rm -rf "${WORKING_DIR}"
echo "Finished :D"
