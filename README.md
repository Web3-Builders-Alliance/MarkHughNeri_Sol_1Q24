# Web3 Builders Alliance - Solana Cohort Pre-req

## Required files
These two files contains private keys

- dev-wallet.json
- wba-wallet.json

## Install dependencies
```sh
npm install
# OR
yarn
```

## Running the scripts
```sh
# generate keypair
yarn keygen

# request test SOL from test/devnet
yarn airdrop

# perform transfer of SOL from one address to another
yarn transfer

# execute WBA program to enroll and complete pre-req
yarn enroll
```

## Result
Enrollment Transaction: (Wrong wallet used)
https://explorer.solana.com/tx/5g3eieR94nQNHsPM2YtuWJETp4pAXLc8EZMgFw1ZVzFq62YNqFpBKg4H8FV7AyBdqtU1VVz8TztJMAp9yMZmKLfm?cluster=devnet

Enrollment Transaction (Re-ran: Corrected wallet)
https://explorer.solana.com/tx/32YvVSnzZibW6wCuTpm4oi5ToYyTxMZWfsggCx567vRgw392xqYsFCd7JRXXbsCDvCGYqyxBHgeMsZrkJPDu4RLT?cluster=devnet