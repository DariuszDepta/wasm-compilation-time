NODE="https://rpc-lb.neutron.org:443"
OUT_DIR="wasm_binaries"
WASMD=~/go/bin/wasmd
COUNT=5200

mkdir -p "$OUT_DIR"

for CODE_ID in $(seq 1 $COUNT); do
  echo "Downloading code id: $CODE_ID"

  $WASMD q wasm code "$CODE_ID" "${OUT_DIR}/${CODE_ID}.wasm" --node "$NODE" 2>/dev/null

  if [ $? -ne 0 ]; then
    echo "  -> Code $CODE_ID not found or failed"
    rm -f "${OUT_DIR}/${CODE_ID}.wasm"
  fi
done

echo "Done."
