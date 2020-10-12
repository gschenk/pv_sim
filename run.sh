#!/bin/sh

config="config.toml"
default_output="results.dat"

# check if localhost is configured (not checking for 127..)
if grep -q localhost $config
then

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
fi

output=${1:-"$default_output"}

echo "#time, meter [kW], PV [kW], new P [kW]" > $output

# start pv
cargo run --manifest-path=pv/Cargo.toml -- -q config.toml >> $output &
pid=$!

cargo run --manifest-path=meter/Cargo.toml config.toml

# kill PV after meter is done
kill $pid

# returns power consumption to STDOUT
cat $output
