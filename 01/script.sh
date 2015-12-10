#!/bin/bash

if [ $# -eq 0 ]
  then
	echo "usage: script.sh [file]";
	exit;
fi

grep -c -E -x '^[a-z]{0,3}[0-9]{2,8}[A-Z]{3,}$' $1