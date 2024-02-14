import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58';

// Now we're going to create a new Keypair, like so:
//Generate a new keypair
let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);

const encodedPrivate = bs58.encode(kp.secretKey);
console.log(encodedPrivate);