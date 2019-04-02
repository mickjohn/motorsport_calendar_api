FROM ubuntu:18.04

RUN mkdir -p "/var/lib/motorsport_calendar_api/database" \
  && mkdir -p "/etc/motorsport_calendar_api/" \
  && apt-get update \
  && apt-get install libsqlite3-dev -y

WORKDIR /etc/motorsport_calendar_api
COPY prod_conf.yml .
COPY log4rs.yml .

WORKDIR /var/lib/motorsport_calendar_api/database

# This is just the test DB to get the app up and running.The production DB is
# in a docker volume, and is mounted over the  above database path.
COPY sqlite/test.db ./ms_api.db

WORKDIR /usr/bin
COPY target/release/motorsport_calendar_api .

EXPOSE 8000
ENV ROCKET_ADDRESS="0.0.0.0"
CMD ["./motorsport_calendar_api", "-c", "/etc/motorsport_calendar_api/prod_conf.yml", "-l", "/etc/motorsport_calendar_api/log4rs.yml"]
