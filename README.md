# MobileCoin Incinerator

Burn MobileCoin by sending it to an unspendable address.

## Summary

This script generates a public address without corresponding private keys. It does this by
hashing a known string into a Ristretto curve point, which it then uses as the public key.
Because the address was not generated via the normal key derivation path, which is a
one-way function, it is not possible to reverse the derivation to recover the private key.
This means that all funds sent to this address can never be spent.

For the known string "burn address", the public address is:
o21C75ybsJVwBfH79SEYxZxFhtUF2VmyjbX22PsHYmJvZ1hPNGoSUZehPPMFWQDwcCzGe3itrjecPhQrKExFjjDwMUNjao2TYRJPDqupyn


## How to burn funds

First install the [MobileCoin CLI](https://github.com/mobilecoinofficial/full-service/tree/main/cli).
```sh
$ mobcli send --build-only a7ddc0 0.001 o21C75ybsJVwBfH79SEYxZxFhtUF2VmyjbX22PsHYmJvZ1hPNGoSUZehPPMFWQDwcCzGe3itrjecPhQrKExFjjDwMUNjao2TYRJPDqupyn
Building transaction for 0.001 MOB from account a7ddc0
to address o21C75ybsJVwBfH79SEYxZxFhtUF2VmyjbX22PsHYmJvZ1hPNGoSUZehPPMFWQDwcCzGe3itrjecPhQrKExFjjDwMUNjao2TYRJPDqupyn
Fee is 0.0004 MOB, for a total amount of 0.0014 MOB.
Wrote tx_proposal.json

$ mobcli submit --receipt tx_proposal.json
Wrote receipt.json
This transaction will not be logged, because an account id was not provided.
Submit this transaction proposal for 0.001 MOB? (Y/N) y
Submitted. The file tx_proposal.json is now unusable for sending transactions.
```

## Proof of burn
As currently written, in order to prove to others that funds were burned, you have to share the view private key of the sending account. This is not easily supported by our tooling currently.

A better version of burn addresses could be created in the future, like this: Generate a burn address with a known view private key, but an unknown spend private key, and have the sender generate a receipt for the burn transaction. The receipt and the view key together can be used to verify the funds have been sent to the burn address.

