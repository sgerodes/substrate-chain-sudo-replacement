SOLO_CHAIN_NODE_EXECUTABLE="./target/release/solochain-template-node"
SOLO_CHAIN_STATE_DATA_BASE_FOLDER="./data"


SOLO_CHAIN_STATE_DATA_BASE_FOLDER=${SOLO_CHAIN_STATE_DATA_BASE_FOLDER}/dev
CHAIN_BASE_PORT=30300
RPC_BASE_PORT=9944

$SOLO_CHAIN_NODE_EXECUTABLE purge-chain -y \
    --dev \
    --base-path "${SOLO_CHAIN_STATE_DATA_BASE_FOLDER}"

rm -rf "${SOLO_CHAIN_STATE_DATA_BASE_FOLDER}"

$SOLO_CHAIN_NODE_EXECUTABLE \
    --dev \
    --base-path "${SOLO_CHAIN_STATE_DATA_BASE_FOLDER}" \
    --port $CHAIN_BASE_PORT \
    --rpc-port $RPC_BASE_PORT \
    --pruning archive-canonical \
    --validator &

trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

while true; do read; done
