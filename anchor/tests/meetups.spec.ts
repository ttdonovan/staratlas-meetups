import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

import { BankrunProvider, startAnchor } from "anchor-bankrun";

// cp target/deploy/meetups.so tests/fixtures/.
import type { Meetups } from "../target/types/meetups";
const IDL = require("../target/idl/meetups.json");

const meetupsAddress = new PublicKey(
  "C14GzDxp9S1UfZk1BR1BPwHK1erFoPWkvjjFRtyf4B7L"
);

const atlasMintAddress = new PublicKey(
  "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx"
);
const polisMintAddress = new PublicKey(
  "poLisWXnNRwC6oBu1vHiuKQzFjGL4XDSu4g9qjz9qVk"
);
const usdcMintAddress = new PublicKey(
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
);

describe("meetups", () => {
  it("Initialize Event Manager", async () => {
    const context = await startAnchor(
      "",
      [{ name: "meetups", programId: meetupsAddress }],
      []
    );
    const provider = new BankrunProvider(context);

    const meetupsProgram = new Program<Meetups>(IDL, provider);

    await meetupsProgram.methods
      .initEventManager(atlasMintAddress, polisMintAddress, usdcMintAddress)
      .rpc();
  });
});
