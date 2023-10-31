#!/bin/bash

set -ueo pipefail

name=debian
container_id=999
template="mergerfs:vztmpl/debian-12-standard_12.2-1_amd64.tar.zst"
hostname=mvp
cores=2   
memory=2048
swap=2048
storage=mergerfs
net0="name=eth0,bridge=vmbr0,ip=dhcp"

pct create $container_id $template -hostname $hostname -cores $cores -memory $memory -swap $swap -net0 $net0 -storage $storage

script=$(cat ../install/$name.sh)
pct exec $container_id -- bash -c "$script"