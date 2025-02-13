import { Program, BN } from "@coral-xyz/anchor";
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

const getFutureDate = (days: number) => {
  const futureDate = new Date();
  futureDate.setDate(futureDate.getDate() + days);

  const year = futureDate.getFullYear();
  const month = futureDate.getMonth() + 1;
  const day = futureDate.getDate();

  return [year, month, day];
};

describe("meetups", () => {
  let context;
  let provider: BankrunProvider;
  let meetupsProgram: Program<Meetups>;
  let eventsManagerAddress: PublicKey;
  let eventsManagerStateAddress: PublicKey;
  let identityProfileAddress: PublicKey;
  let futureDate: number[];

  beforeAll(async () => {
    context = await startAnchor(
      "",
      [{ name: "meetups", programId: meetupsAddress }],
      []
    );

    provider = new BankrunProvider(context);
    meetupsProgram = new Program<Meetups>(IDL, provider);

    [eventsManagerAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("manager"), provider.wallet.publicKey.toBuffer()],
      meetupsAddress
    );

    [eventsManagerStateAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("state"), provider.wallet.publicKey.toBuffer()],
      meetupsAddress
    );

    [identityProfileAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("identity"), provider.wallet.publicKey.toBuffer()],
      meetupsAddress
    );

    futureDate = getFutureDate(3);
  });

  it("Initialize Events Manager", async () => {
    await meetupsProgram.methods
      .initEventManager(atlasMintAddress, polisMintAddress, usdcMintAddress)
      .rpc();

    const eventsManager = await meetupsProgram.account.eventsManager.fetch(
      eventsManagerAddress
    );
    // console.log(eventsManager);

    expect(eventsManager.mints.atlas).toEqual(atlasMintAddress);
    expect(eventsManager.mints.polis).toEqual(polisMintAddress);
    expect(eventsManager.mints.usdc).toEqual(usdcMintAddress);

    const eventsManagerState =
      await meetupsProgram.account.eventsManagerState.fetch(
        eventsManagerStateAddress
      );
    // console.log(eventsManagerState);

    expect(eventsManagerState.authority).toEqual(provider.wallet.publicKey);
    expect(eventsManagerState.eventsManager).toEqual(eventsManagerAddress);

    expect(eventsManagerState.vaultFeeInfo.daoVaultFee).toEqual(0);
    expect(eventsManagerState.vaultFeeInfo.devVaultFee).toEqual(0);
    expect(eventsManagerState.vaultFeeInfo.opsVaultFee).toEqual(0);
    expect(eventsManagerState.vaultFeeInfo.hostProfileFee).toEqual(0);

    expect(eventsManagerState.vaultOwnerInfo.daoVaultOwner).toEqual(
      provider.wallet.publicKey
    );
    expect(eventsManagerState.vaultOwnerInfo.devVaultOwner).toEqual(
      provider.wallet.publicKey
    );
    expect(eventsManagerState.vaultOwnerInfo.opsVaultOwner).toEqual(
      provider.wallet.publicKey
    );
  });

  // it("Updates Events Manager Vault Fee Info", async () => {
  //   // todo!
  // });

  // it("Updates Events Manager Vault Owner Info", async () => {
  //   // todo!
  // });

  it("Initialize Identity Profile", async () => {
    await meetupsProgram.methods.initIdentityProfile("Space Cadet").rpc();

    const identityProfile = await meetupsProgram.account.identityProfile.fetch(
      identityProfileAddress
    );
    // console.log(identityProfile);

    expect(identityProfile.owner).toEqual(provider.wallet.publicKey);
    expect(identityProfile.name).toEqual("Space Cadet");
  });

  it("Create an Event", async () => {
    const [year, month, day] = futureDate;

    // Get the host profile PDA
    const [hostProfileAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("host"),
        eventsManagerAddress.toBuffer(),
        identityProfileAddress.toBuffer(),
      ],
      meetupsAddress
    );

    // Get the event PDA
    const [eventAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("event"),
        eventsManagerAddress.toBuffer(),
        hostProfileAddress.toBuffer(),
        new BN(year).toArrayLike(Buffer, "le", 2), // Buffer.from([2025 & 0xff, (2025 >> 8) & 0xff]), // year (u16) as le bytes
        new BN(month).toArrayLike(Buffer, "le", 1), // Buffer.from([2]), // month (u8)
        new BN(day).toArrayLike(Buffer, "le", 1), // Buffer.from([1]), // day (u8)
      ],
      meetupsAddress
    );

    await meetupsProgram.methods
      .createEvent(
        eventsManagerAddress,
        year,
        month,
        day,
        "PENDING: Star Explorer's Night Out"
      )
      .rpc();

    const event = await meetupsProgram.account.eventEntry.fetch(eventAddress);
    // console.log(event);

    expect(event.name).toEqual("PENDING: Star Explorer's Night Out");
    expect(event.location).toEqual("");
    expect(event.mappableAddress).toEqual("");
    expect(event.startTimeAt.toString()).toEqual("0");
    expect(event.endTimeAt.toString()).toEqual("0");
    expect(event.status).toEqual({ pending: {} });
  });

  it("Update an Event", async () => {
    const [year, month, day] = futureDate;

    // Get the host profile PDA
    const [hostProfileAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("host"),
        eventsManagerAddress.toBuffer(),
        identityProfileAddress.toBuffer(),
      ],
      meetupsAddress
    );

    // Get the event PDA
    const [eventAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("event"),
        eventsManagerAddress.toBuffer(),
        hostProfileAddress.toBuffer(),
        new BN(year).toArrayLike(Buffer, "le", 2), // Buffer.from([2025 & 0xff, (2025 >> 8) & 0xff]), // year (u16) as le bytes
        new BN(month).toArrayLike(Buffer, "le", 1), // Buffer.from([2]), // month (u8)
        new BN(day).toArrayLike(Buffer, "le", 1), // Buffer.from([1]), // day (u8)
      ],
      meetupsAddress
    );

    await meetupsProgram.methods
      .updateEvent(
        eventsManagerAddress,
        year,
        month,
        day,
        "Star Explorer's Night Out",
        "21st Amendment Brewery",
        "563 2nd St, San Francisco, CA 94107",
        new BN(1),
        new BN(2)
      )
      .rpc();

    const event = await meetupsProgram.account.eventEntry.fetch(eventAddress);
    // console.log(event);

    expect(event.name).toEqual("Star Explorer's Night Out");
    expect(event.location).toEqual("21st Amendment Brewery");
    expect(event.mappableAddress).toEqual(
      "563 2nd St, San Francisco, CA 94107"
    );
    expect(event.startTimeAt.toString()).toEqual("1");
    expect(event.endTimeAt.toString()).toEqual("2");
    expect(event.status).toEqual({ pending: {} });
  });

  it("Open (and Close) an Event", async () => {
    const [year, month, day] = futureDate;

    // Get the host profile PDA
    const [hostProfileAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("host"),
        eventsManagerAddress.toBuffer(),
        identityProfileAddress.toBuffer(),
      ],
      meetupsAddress
    );

    // Get the event PDA
    const [eventAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("event"),
        eventsManagerAddress.toBuffer(),
        hostProfileAddress.toBuffer(),
        new BN(year).toArrayLike(Buffer, "le", 2), // Buffer.from([2025 & 0xff, (2025 >> 8) & 0xff]), // year (u16) as le bytes
        new BN(month).toArrayLike(Buffer, "le", 1), // Buffer.from([2]), // month (u8)
        new BN(day).toArrayLike(Buffer, "le", 1), // Buffer.from([1]), // day (u8)
      ],
      meetupsAddress
    );

    await meetupsProgram.methods
      .openEvent(eventsManagerAddress, year, month, day)
      .rpc();

    let event = await meetupsProgram.account.eventEntry.fetch(eventAddress);
    // console.log(event);
    expect(event.status).toEqual({ open: {} });

    await meetupsProgram.methods
      .closeEvent(eventsManagerAddress, year, month, day)
      .rpc();

    event = await meetupsProgram.account.eventEntry.fetch(eventAddress);
    // console.log(event);
    expect(event.status).toEqual({ closed: {} });

    // todo!: re-open event
  });
});
