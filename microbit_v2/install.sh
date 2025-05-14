#! /bin/bash

cargo build || exit -1

export WAITING='Waiting for media'
while [[ ! -w /media/$USER/MAINTENANCE/ ]] ;
do
    echo -n $WAITING
    sleep 2;
    export WAITING=''
done
echo arm-none-eabi-objcopy target/thumbv6m-none-eabi/debug/hellomicrobit /media/$USER/MAINTENANCE/out.hex -O ihex
arm-none-eabi-objcopy target/thumbv7em-none-eabihf/debug/hellomicrobit /media/$USER/MAINTENANCE/out.hex -O ihex
