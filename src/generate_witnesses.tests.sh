#!/bin/bash
if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

file=$1
json=$(cat $file|grep -v '^#')

function generate_let_line() {
  local index=$1
  echo "let witnesses_n = &witnesses[$index];"
}

function generate_test_makro_line(){
  local index=$1
  local string_to_check=$2
  echo -en "test_witness!(&witnesses_n[$index], $string_to_check);\n"
}

function generate_test_makro_line_no_witness() {
  #where is no sub vector in the &witnesses_n var
  echo -en "test_witness!(&witnesses_n[0], \"\");\n"
}

function generate_test() {
  local json=$1
  local index=$2
  local n=0
  generate_let_line $index
  if [ "$json" == "null" ];then # an empty witness (input has no witness)
    generate_test_makro_line_no_witness
  else
    for witness in $(echo $json | jq '.[]');do
      generate_test_makro_line $n $witness
      let n++
    done
  fi
}

is_coinbase=$(echo $json | jq '.vin | map(select(has("coinbase"))) | length')
version=$(echo $json | jq '.version')

if [ $is_coinbase -eq 1 ];then
  if [ $version -eq 2 ];then
    generate_let_line 0
    generate_test_makro_line 0 "\"0000000000000000000000000000000000000000000000000000000000000000\""
    exit 0
  fi
fi

n=0
for witnesses in $(echo $json | jq -c '.vin[].txinwitness');do
  generate_test $witnesses $n
  let n++
done
