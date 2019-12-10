#!/bin/bash

if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

file=$1

n=0
for input in $(cat $file |grep -v '^#' |jq '.vin[] | [ .txid, .vout, .scriptSig.hex, .sequence ] | @csv' | sed 's/[\\]*"//g');do 
  txid=$(echo $input|cut -d, -f1)
  vout=$(echo $input|cut -d, -f2)
  scriptSig=$(echo $input|cut -d, -f3)
  sequence=$(echo $input|cut -d, -f4)
  echo -en "test_input!("
  echo -en "&inputs[$n], "
  echo -en "\"$(echo $txid|./endian.sh)\", "
  echo -en "$vout, "
  echo -en "\"$scriptSig\", "
  echo -en "$sequence"
  echo -en ");\n"
  let n++
done
