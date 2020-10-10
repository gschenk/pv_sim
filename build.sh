#!/bin/sh

paths=("pv insolation meter")
config="config.toml"
template="config.template.toml"
debug=true

# build each crate
for p in ${paths}
do
  echo "Build Crate ${p}"
  cargo build --manifest-path=${p}/Cargo.toml
done

if [ ! -f $config ]
then
  echo "Copying config file. You may like to change it."
  cp -nv $template $config
elif [ $debug = true ]
then
  echo "Writing over your config file if the template is newer (debug=true)."
  cp -uv $template $config
fi
