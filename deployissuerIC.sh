dfx canister create issuer
dfx build issuer

candid-extractor target/wasm32-unknown-unknown/release/issuer.wasm > issuer/issuer.did

dfx generate issuer

rootkey_did=$(dfx ping ic \
    | sed -n 's/.*"root_key": \[\(.*\)\].*/{\1}/p' \
    | sed 's/\([0-9][0-9]*\)/\1:nat8/g' \
    | sed 's/,/;/g')
    
echo "Public key: ${rootkey_did}"

II_CANISTER_ID="rdmx6-jaaaa-aaaaa-aaadq-cai"

echo "${II_CANISTER_ID}"

dfx deploy issuer --network ic --argument "( \
    record { \
        ii_canister_id = principal \"${II_CANISTER_ID}\"; \
        ic_root_key_der = vec ${rootkey_did}; \
    } \
)"