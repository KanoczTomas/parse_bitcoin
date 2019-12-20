#!/bin/bash

if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

#from the point of view of the test itself (parsers dir for example)
test_data_dir="../test_data/"
file=$1
json=$(cat $file | grep -v '^#')
coinbase=$(echo $json | jq '.vin[0].coinbase'|tr -d '"')

function count_witnesses(){
  local version=$1
  local inputs_count=$2
  witnesses_count=$(echo $json | jq '.vin | map(select(has("txinwitness"))) | length')
  if [ "$coinbase" != "null" ] && [ $version -eq 2 ]; then #version 2 coinbase has a witness of all 0
     echo 1
  else
    [ $witnesses_count -eq 0 ] && echo 0 || echo $inputs_count
  fi
}

function generate_tx_tests(){
  local file=$1
  local version=$2
  local locktime=$3
  local inputs_count=$4
  local outputs_count=$5
  echo -en "let data = include_bytes!(\"$test_data_dir$(echo $file|sed 's/\.rpc/\.bin/g')\");\n"
  echo "let (_, tx) = parse_transaction(data).unwrap();"
  echo "assert_eq!(tx.version, $version);"
  echo "assert_eq!(tx.lock_time, $locktime);"
  echo "assert_eq!(tx.inputs.len(), $inputs_count);"
  echo "assert_eq!(tx.outputs.len(), $outputs_count);"
  echo "let witnesses = match tx.witnesses {"
  echo -en "\tSome(witnesses) => witnesses,\n"
  echo -en "\tNone => vec![]\n"
  echo "};"

  witnesses_count=$(count_witnesses $version $inputs_count)
  echo "assert_eq!(witnesses.len(), $witnesses_count);"
  echo "let inputs = tx.inputs;"
  ./generate_input_tests.sh $file
  echo ""
  echo "let outputs = tx.outputs;"
  ./generate_output_tests.sh $file
  echo ""
  [ $witnesses_count -ne 0 ] && ./generate_witnesses.tests.sh $file
  echo ""
}

version=$(echo $json | jq '.version')
locktime=$(echo $json | jq '.locktime')
inputs_count=$(echo $json | jq '.vin | length')
outputs_count=$(echo $json | jq '.vout | length')

generate_tx_tests $file $version $locktime $inputs_count $outputs_count
