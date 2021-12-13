#!/usr/bin/env bash

set -e

if [ -z "$1" ]; then
	echo "Please provide the number of initial validators!"
	exit 1
fi

if [ ! -z "$2" ]; then
  SECRET=$2
fi

generate_account_id() {
	subkey inspect ${2:-} ${3:-} "$SECRET//$1" | grep "Public key (hex)" | awk '{ print $4 }'
}

generate_address() {
	subkey inspect ${2:-} ${3:-} "$SECRET//$1" | grep "Public key (SS58)" | awk '{ print $4 }'
}

generate_address_and_account_id() {
	ACCOUNT=$(generate_account_id $1 $2)
	ADDRESS=$(generate_address $1 $2)
	if ${3:-false}; then
		INTO="unchecked_into"
	else
		INTO="into"
	fi

	printf "// $ADDRESS\nhex![\"${ACCOUNT#'0x'}\"].$INTO(),"
}

V_NUM=$1

AUTHORITIES=""

for i in $(seq 1 $V_NUM); do
	AUTHORITIES+="(\n"
	AUTHORITIES+="$(generate_address_and_account_id $i '--scheme sr25519' false)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i '--scheme sr25519' true)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i '--scheme ed25519' true)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i '--scheme sr25519' true)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i '--scheme ecdsa' true)\n"
  AUTHORITIES+="$(generate_address_and_account_id $i '--scheme sr25519' true)\n"
	AUTHORITIES+="),\n"
done

printf "$AUTHORITIES"
