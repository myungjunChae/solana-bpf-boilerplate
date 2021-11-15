/* eslint-disable @typescript-eslint/no-non-null-assertion */
import * as dotenv from "dotenv";
dotenv.config();

import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import BN = require("bn.js");
import { checkAccountInitialized, checkAccountDataIsValid } from "./utils";

import {
  CustomAccountLayout,
  CustomAccountLayoutInterface,
  ExpectedCustomAccountLayoutInterface,
} from "./account";

type InstructionNumber = 0 | 1;

const transaction = async () => {
  //phase1 (setup Transaction & send Transaction)
  console.log("Setup Transaction");
  const connection = new Connection("http://localhost:8899", "confirmed");
  const programId = new PublicKey(process.env.CUSTOM_PROGRAM_ID!);
  const userPubkey = new PublicKey(process.env.USER_PUBLIC_KEY!);
  const userPrivateKey = Uint8Array.from(JSON.parse(process.env.USER_PRIVATE_KEY!));
  const userWallet = new Keypair({
    publicKey: userPubkey.toBytes(),
    secretKey: userPrivateKey,
  });
  const instruction: InstructionNumber = 0;
  const data = 10;

  const customAccountKeypair = new Keypair();
  const createCustomAccountIx = SystemProgram.createAccount({
    space: CustomAccountLayout.span,
    lamports: await connection.getMinimumBalanceForRentExemption(CustomAccountLayout.span),
    fromPubkey: userWallet.publicKey,
    newAccountPubkey: customAccountKeypair.publicKey,
    programId: programId,
  });

  const initializeCustomProgramIx = new TransactionInstruction({
    programId: programId,
    keys: [
      { pubkey: userWallet.publicKey, isSigner: true, isWritable: false },
      { pubkey: customAccountKeypair.publicKey, isSigner: false, isWritable: true },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    data: Buffer.from(Uint8Array.of(instruction, ...new BN(data).toArray("le", 8))),
  });

  //make transaction with several instructions(ix)
  console.log("Send transaction... [createCustomAccount, initializeCustomProgram]\n");
  const tx = new Transaction().add(createCustomAccountIx, initializeCustomProgramIx);

  await connection.sendTransaction(tx, [userWallet, customAccountKeypair], {
    skipPreflight: false,
    preflightCommitment: "confirmed",
  });
  //phase1 end

  //wait block update
  await new Promise((resolve) => setTimeout(resolve, 1000));

  //phase2 (check Transaction result is valid)
  const customAccount = await checkAccountInitialized(connection, customAccountKeypair.publicKey);

  const encodedCustomAccountData = customAccount.data;
  const decodedCustomAccountData = CustomAccountLayout.decode(
    encodedCustomAccountData
  ) as CustomAccountLayoutInterface;

  const expectedCustomAccountData: ExpectedCustomAccountLayoutInterface = {
    isInitialized: 1,
    walletPubkey: userWallet.publicKey,
    customAccountPubkey: customAccountKeypair.publicKey,
    data: data,
  };

  console.log("Current AccountData");
  checkAccountDataIsValid(decodedCustomAccountData, expectedCustomAccountData);

  //#phase2 end
  console.log(`✨TX successfully finished✨\n`);
};

transaction();
