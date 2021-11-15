import { PublicKey } from "@solana/web3.js";

//@ts-expect-error missing types
import * as BufferLayout from "buffer-layout";

export const CustomAccountLayout = BufferLayout.struct([
  BufferLayout.u8("isInitialized"), //1byte
  BufferLayout.blob(32, "walletPubkey"), //pubkey(32byte)
  BufferLayout.blob(32, "customAccountPubkey"), //pubkey(32byte)
  BufferLayout.blob(8, "data"), //8byte
]);

export interface CustomAccountLayoutInterface {
  [index: string]: number | Uint8Array;
  isInitialized: number;
  walletPubkey: Uint8Array;
  customAccountPubkey: Uint8Array;
  data: Uint8Array;
}

export interface ExpectedCustomAccountLayoutInterface {
  [index: string]: number | PublicKey;
  isInitialized: number;
  walletPubkey: PublicKey;
  customAccountPubkey: PublicKey;
  data: number;
}
