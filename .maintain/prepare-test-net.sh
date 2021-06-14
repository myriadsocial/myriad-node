#!/usr/bin/env bash

set -e

if [ "$#" -ne 1 ]; then
	echo "Please provide the number of initial collator!"
	exit 1
fi
SECRET="fall deal book genuine tonight chimney angry steak proof wheel bag faith//collator"
generate_account_id() {
	subkey inspect ${2:-} ${3:-} "$SECRET//$1" | grep "Account ID" | awk '{ print $3 }'
}

generate_address() {
	subkey inspect ${2:-} ${3:-} "$SECRET//$1" | grep "SS58 Address" | awk '{ print $3 }'
}

generate_address_and_account_id() {
	ACCOUNT=$(generate_account_id $1 $2)
	ADDRESS=$(generate_address $1 $2)
	if ${3:-false}; then
		INTO="unchecked_into"
	else
		INTO="into"
	fi

	printf "//$ADDRESS\nhex![\"${ACCOUNT#'0x'}\"].$INTO(),"
}

V_NUM=$1

AUTHORITIES=""

for i in $(seq 1 $V_NUM); do
	AUTHORITIES+="(\n"
	AUTHORITIES+="$(generate_address_and_account_id $i '--scheme sr25519' false)\n"
	AUTHORITIES+="$(generate_address_and_account_id $i '--scheme sr25519' true)\n"
	AUTHORITIES+="),\n"
done

printf "$AUTHORITIES"
