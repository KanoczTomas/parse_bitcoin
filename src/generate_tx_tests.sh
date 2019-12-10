#!/bin/bash

#!/bin/bash

if [[ $# -ne 1 ]];then
  echo "usage: $0 <file.rpc>"
  exit 1
fi

file=$1
jq="cat $file | grep -v getraw | jq "

echo -en "let data = include_bytes!(\"$(echo $file|sed 's/\.rpc/\.bin/g')\");\n"
echo "let (_, tx) = parse_transaction(data).unwrap();"
echo  "assert_eq!(tx.version, $(bash -c "$jq '.version'"));"
echo "assert_eq!(tx.lock_time, $(bash -c "$jq '.locktime'"));"
echo "assert_eq!(tx.inputs.len(), $(bash -c "$jq '.vin | length'"));"
echo "assert_eq!(tx.outputs.len(), $(bash -c "$jq '.vout | length'"));"
echo "let witnesses = match tx.witnesses {"
echo -en "\tSome(witnesses) => witnesses,\n"
echo -en "\tNone => vec![]\n"
echo "};"
echo "assert_eq!(witnesses.len(), $(cat $file | grep -v getraw | jq '.vin | map(select(has("txinwitness"))) | length'));"
echo "let inputs = tx.inputs;"
./generate_input_tests.sh $file
echo ""
echo "let outputs = tx.outputs;"
./generate_output_tests.sh $file
echo ""
./generate_witnesses.tests.sh $file
echo ""
