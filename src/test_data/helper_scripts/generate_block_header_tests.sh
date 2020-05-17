#!/bin/bash

if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

#from the point of view of the test itself (parsers dir for example)
test_data_dir="../test_data/"
file=$1
json=$(cat $file | grep -v '^#')

function generate_block_header_tests(){
  local version=$1
  local prev_block_hash=$2
  local merkle_root_hash=$3
  local time=$4
  local bits=$5
  local nonce=$6
  local hash=$7

  echo -en "let data = include_bytes!(\"$test_data_dir$(echo $file|sed 's/^..\///;s/\.rpc/\.bin/g')\");\n"
  echo -en "let (_, header) = parse_block_header(data).unwrap();\n"
  echo -en "assert_eq!(header.version, $version);\n"
  echo -en "assert_eq!(header.prev_block_hash, Hash256::new(&hex::decode(\"$(echo $prev_block_hash|./endian.sh)\").unwrap()));\n"
  echo -en "assert_eq!(header.merkle_root_hash, Hash256::new(&hex::decode(\"$(echo $merkle_root_hash|./endian.sh)\").unwrap()));\n"
  echo -en "assert_eq!(header.time.0, $time);\n"
  echo -en "assert_eq!(header.bits, Bytes::new(&hex::decode(\"$(echo $bits|./endian.sh)\").unwrap()));\n"
  echo -en "assert_eq!(header.nonce, Bytes::new(&hex::decode(\"$(echo $nonce|./endian.sh)\").unwrap()));\n"
  echo -en "assert_eq!(header.hash, Hash256::new(&hex::decode(\"$(echo $hash|./endian.sh)\").unwrap()));\n"
  echo ""
}

version=$(echo $json | jq '.version')
prev_block_hash=$(echo $json | jq '.previousblockhash' | tr -d '"')
[ "$prev_block_hash" == "null" ] && prev_block_hash="0000000000000000000000000000000000000000000000000000000000000000"
merkle_root_hash=$(echo $json | jq '.merkleroot' | tr -d '"')
time=$(echo $json | jq '.time')
bits=$(echo $json | jq '.bits' | tr -d '"')
nonce=$(printf %2X $(echo $json | jq '.nonce'))
hash=$(echo $json | jq '.hash' | tr -d '"')

generate_block_header_tests $version $prev_block_hash $merkle_root_hash $time $bits $nonce $hash
