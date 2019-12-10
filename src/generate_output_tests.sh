#!/bin/bash

if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

file=$1

n=0
for input in $(cat $file |grep -v getrawtransaction|jq '.vout[] | [ .value, .scriptPubKey.hex ] | @csv' | sed 's/[\\]*"//g');do 
  value=$(echo $input|cut -d, -f1)
  scriptPubKey=$(echo $input|cut -d, -f2)
  value=$(echo "$value*100000000"|bc)
  value=${value%.*}
  echo -en "test_output!(outputs[$n], $value, \"$scriptPubKey\");\n"
  let n++
done
