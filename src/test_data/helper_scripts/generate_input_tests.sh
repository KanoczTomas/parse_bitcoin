#!/bin/bash

if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

file=$1
json=$(cat $file|grep -v '^#')


#is_coinbase=$(echo $json | jq '.vin | map(select(has("coinbase"))) | length')
#if [ $is_coinbase -eq 1 ];then
#  is_coinbase="yes"
#  coinbase=$(echo $json | jq '.vin[0].coinbase'|tr -d '"')
#else
#  is_coinbase="no"
#fi

coinbase=$(echo $json | jq '.vin[0].coinbase'|tr -d '"')

function generate_test_input(){
  local index=$1
  local txid=$2
  local vout=$3
  local scriptSig=$4
  local sequence=$5

  echo -en "test_input!("
  echo -en "&inputs[$index], "
  echo -en "\"$(echo $txid | tr -d '"'|./endian.sh)\", "
  echo -en "$vout, "
  echo -en "$scriptSig, "
  echo -en "$sequence"
  echo -en ");\n"

}

n=0
for input in $(echo $json |jq '.vin[] | [ .txid, .vout, .scriptSig.hex, .sequence ] | @csv' |  sed 's/^"//;s/"$//;s/\\//g');do
  txid=$(echo $input|cut -d, -f1)
  vout=$(echo $input|cut -d, -f2)
  scriptSig=$(echo $input|cut -d, -f3)
  [ "$scriptSig" == "" ] && scriptSig="\"\""
  sequence=$(echo $input|cut -d, -f4)
  #if coinbase
  if [ "$coinbase" != "null" ];then
    txid="\"0000000000000000000000000000000000000000000000000000000000000000\""
    scriptSig="\"$coinbase\""
    vout=4294967295
  fi
  generate_test_input $n $txid $vout $scriptSig $sequence
  let n++
done
