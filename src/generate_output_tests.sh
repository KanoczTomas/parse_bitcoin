#!/bin/bash

if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

file=$1
json=$(cat $file|grep -v '^#')

function generate_test_output(){
  local index=$1
  local value=$2
  local scriptPubKey=$3
  value=$(echo "$value*100000000"|bc)
  value=${value%.*}
  echo -en "test_output!(outputs[$n], $value, \"$scriptPubKey\");\n"
}

n=0
for output in $(echo $json | jq '.vout[] | [ .value, .scriptPubKey.hex ] | @csv' | sed 's/[\\]*"//g');do
  value=$(echo $output|cut -d, -f1)
  scriptPubKey=$(echo $output|cut -d, -f2)
  generate_test_output $n $value $scriptPubKey
  let n++
done
