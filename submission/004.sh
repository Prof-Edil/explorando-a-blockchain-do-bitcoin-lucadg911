#!/bin/bash
XPUB="xpub6Cx5tvq6nACSLJdra1A6WjqTo1SgeUZRFqsX5ysEtVBMwhCCRa4kfgFqaT2o1kwL3esB1PsYr3CUdfRZYfLHJunNWUABKftK2NjHUtzDms2"
DERIVATION="/100"
USER="-rpcuser=user_240"
PASSWORD="-rpcpassword=JQs8WLu7N4dR"
IP="-rpcconnect=84.247.182.145"

DESCRIPTOR=$(bitcoin-cli $IP $USER $PASSWORD getdescriptorinfo "tr($XPUB$DERIVATION)"| jq -r '.descriptor')
ADDRESS=$(bitcoin-cli $IP $USER $PASSWORD deriveaddresses "$DESCRIPTOR" | jq -r '.[0]')

echo $ADDRESS
