#!/bin/sh

# check if RabbitMQ is available
if ! command -v rabbitmqctl >/dev/null
then
  echo "rabitmqctl is not available. RabitMQ seems not to be installed."
  exit 1
fi

# test if systemd is available
if command -v systemctl >/dev/null
then
  # check if rabbitmq service is running
  if ! systemctl is-active --quiet rabbitmq.service
  then
    echo "RabitMQ service is not running. 'systemctl start rabitmq.service' may fix it."
    exit 2
  fi
fi
# some distros (eg Alpine, slackware) do not use systemd, put a different check here

echo "it seems up and running"

# start pv

# start meter

# returns power consumption to STDOUT
