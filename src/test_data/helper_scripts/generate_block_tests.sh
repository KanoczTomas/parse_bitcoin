#!/bin/bash

if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

#from the point of view of the test itself (parsers dir for example)
test_data_dir="../test_data/"
file=$1
json=$(cat $file | grep -v '^#')

function generate_block_tests(){
  local file=$1
  local tx_len=$2

  echo -en "let data = include_bytes!(\"$test_data_dir$(echo $file|sed 's/^..\///;s/\.rpc/\.bin/g')\");\n"
  echo "let (_, block) = parse_block(data).unwrap();"
  echo "assert_eq!(block.transactions.len(),$tx_len);"
  txid=$(echo $json | jq ".tx[0]" | tr -d '"' | ./endian.sh)
  echo "assert_eq!(block.transactions[0].txid, Hash256::new(&hex::decode(\"$txid\").unwrap()));"
  if [ $tx_len -ne 1 ];then
    txid=$(echo $json | jq ".tx[$(($tx_len-1))]" | tr -d '"' | ./endian.sh)
    echo "assert_eq!(block.transactions[$(($tx_len-1))].txid, Hash256::new(&hex::decode(\"$txid\").unwrap()));"
  fi
  echo ""

}

tx_len=$(echo $json | jq '.tx | length')

generate_block_tests $file $tx_len
