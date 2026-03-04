NODE="https://rpc-lb.neutron.org:443"
NEXT_KEY=""
ALL=0
WASMD=~/go/bin/wasmd

while : ; do
  if [ -z "$NEXT_KEY" ]; then
    RESPONSE=$($WASMD query wasm list-code --node "$NODE" -o json)
  else
    RESPONSE=$($WASMD query wasm list-code --node "$NODE" --page-key "$NEXT_KEY" -o json)
  fi

  COUNT=$(echo "$RESPONSE" | jq '.code_infos | length')
  ALL=$((ALL + COUNT))

  NEXT_KEY=$(echo "$RESPONSE" | jq -r '.pagination.next_key')
  if [[ "$NEXT_KEY" == "null" || -z "$NEXT_KEY" ]]; then
    break
  fi

  echo "Count = $COUNT, next-key = $NEXT_KEY"
done

echo "Total stored WASM code entries: $ALL"
