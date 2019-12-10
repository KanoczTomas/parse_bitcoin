#!/bin/bash

if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

file=$1

witness_input_count=$(cat $file |grep -v getrawtransaction|jq '.vin | map(select(has("txinwitness"))) | length')

for n in $(seq 0 $(($witness_input_count-1)));do

  witness_count=$(cat $file |grep -v getrawtransaction|jq ".vin[$n].txinwitness | length")

  echo "let witnesses_n = &witnesses[$n];"
  for i in $(seq 0 $((witness_count-1)));do
    witness=$(cat $file |grep -v getrawtransaction|jq ".vin[$n].txinwitness[$i]")
    echo -en "test_witness!(&witnesses_n[$i], $witness);\n"
  done

done
