import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CollatMezo } from "../target/types/collat_mezo";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  Account,
} from "@solana/spl-token";

describe("collat-mezo", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.collatMezo as Program<CollatMezo>;
  const wallet = provider.wallet as anchor.Wallet;

  let btcMint: PublicKey;
  let musdMint: PublicKey;
  let vaultAddress: PublicKey;
  let vaultBtcAta: PublicKey;
  let vaultMusdAta: PublicKey;
  let priceFeedAddress: PublicKey;
  let positionAddress: PublicKey;

  const VAULT_SEED = "vault";
  const VAULT_BTC_SEED = "vault_btc";
  const VAULT_MUSD_SEED = "vault_musd";
  const PRICE_FEED_SEED = "price_feed";
  const POSITION_SEED = "position";

  before(async () => {
    // Create BTC and MUSD mints
    btcMint = await createMint(
      provider.connection,
      wallet.payer,
      wallet.publicKey,
      wallet.publicKey,
      8
    );
    musdMint = await createMint(
      provider.connection,
      wallet.payer,
      wallet.publicKey,
      wallet.publicKey,
      6
    );

    // Derive PDAs
    const vaultSeeds = [
      Buffer.from(VAULT_SEED),
      wallet.publicKey.toBuffer(),
    ];
    [vaultAddress] = PublicKey.findProgramAddressSync(
      vaultSeeds,
      program.programId
    );

    const vaultBtcSeeds = [
      Buffer.from(VAULT_BTC_SEED),
      vaultAddress.toBuffer(),
    ];
    [vaultBtcAta] = PublicKey.findProgramAddressSync(
      vaultBtcSeeds,
      program.programId
    );

    const vaultMusdSeeds = [
      Buffer.from(VAULT_MUSD_SEED),
      vaultAddress.toBuffer(),
    ];
    [vaultMusdAta] = PublicKey.findProgramAddressSync(
      vaultMusdSeeds,
      program.programId
    );

    [priceFeedAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from(PRICE_FEED_SEED)],
      program.programId
    );

    const positionSeeds = [
      Buffer.from(POSITION_SEED),
      wallet.publicKey.toBuffer(),
    ];
    [positionAddress] = PublicKey.findProgramAddressSync(
      positionSeeds,
      program.programId
    );

    // Fund wallet
    const sig = await provider.connection.requestAirdrop(
      wallet.publicKey,
      10 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    // Mint BTC and MUSD to user
    const userBtcAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      btcMint,
      wallet.publicKey
    );
    await mintTo(
      provider.connection,
      wallet.payer,
      btcMint,
      userBtcAta.address,
      wallet.publicKey,
      100_000_000_000n // 1000 BTC (8 decimals)
    );

    const userMusdAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      musdMint,
      wallet.publicKey
    );
    await mintTo(
      provider.connection,
      wallet.payer,
      musdMint,
      userMusdAta.address,
      wallet.publicKey,
      10_000_000_000_000n // 10M MUSD (6 decimals)
    );

    // Initialize vault
    const vaultBtcAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      btcMint,
      vaultAddress,
      true
    );

    const vaultMusdAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      musdMint,
      vaultAddress,
      true
    );

    await program.methods
      .initializeVault(new anchor.BN(1_000_000), new anchor.BN(500))
      .accounts({
        admin: wallet.publicKey,
        vault: vaultAddress,
        btcMint: btcMint,
        musdMint: musdMint,
        vaultBtcAta: vaultBtcAta.address,
        vaultMusdAta: vaultMusdAta.address,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    // Set price (BTC = $60,000 USD)
    await program.methods
      .setPrice(new anchor.BN(60_000_000_000))
      .accounts({
        oracle: wallet.publicKey,
        priceFeed: priceFeedAddress,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });

  it("deposits BTC collateral", async () => {
    const userBtcAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      btcMint,
      wallet.publicKey
    );

    const vaultBtcAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      btcMint,
      vaultAddress,
      true
    );

    await program.methods
      .depositCollateral(new anchor.BN(10_000_000_000)) // 100 BTC
      .accounts({
        user: wallet.publicKey,
        vault: vaultAddress,
        position: positionAddress,
        userBtcAta: userBtcAta.address,
        vaultBtcAta: vaultBtcAta.address,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const position = await program.account.userPosition.fetch(
      positionAddress
    );
    console.log("BTC deposited:", position.btcDeposited.toString());
  });

  it("borrows MUSD against BTC", async () => {
    const userMusdAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      musdMint,
      wallet.publicKey
    );

    const vaultMusdAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      musdMint,
      vaultAddress,
      true
    );

    // Borrow 300,000 MUSD (60% LTV of 100 BTC * $60k = $6M, 60% = $3.6M)
    // Mint enough MUSD to vault first
    await mintTo(
      provider.connection,
      wallet.payer,
      musdMint,
      vaultMusdAta.address,
      wallet.publicKey,
      10_000_000_000_000n
    );

    await program.methods
      .borrow(new anchor.BN(300_000_000_000)) // 300,000 MUSD
      .accounts({
        owner: wallet.publicKey,
        vault: vaultAddress,
        position: positionAddress,
        priceFeed: priceFeedAddress,
        userMusdAta: userMusdAta.address,
        vaultMusdAta: vaultMusdAta.address,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const position = await program.account.userPosition.fetch(
      positionAddress
    );
    console.log("MUSD borrowed:", position.musdBorrowed.toString());
  });

  it("repays MUSD loan", async () => {
    const userMusdAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      musdMint,
      wallet.publicKey
    );

    const vaultMusdAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      musdMint,
      vaultAddress,
      true
    );

    await program.methods
      .repay(new anchor.BN(100_000_000_000)) // repay 100,000 MUSD
      .accounts({
        owner: wallet.publicKey,
        vault: vaultAddress,
        position: positionAddress,
        userMusdAta: userMusdAta.address,
        vaultMusdAta: vaultMusdAta.address,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const position = await program.account.userPosition.fetch(
      positionAddress
    );
    console.log("Remaining MUSD debt:", position.musdBorrowed.toString());
  });

  it("withdraws BTC after full repayment", async () => {
    const userMusdAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      musdMint,
      wallet.publicKey
    );

    const vaultMusdAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      musdMint,
      vaultAddress,
      true
    );

    const vaultBtcAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      btcMint,
      vaultAddress,
      true
    );

    const userBtcAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      btcMint,
      wallet.publicKey
    );

    const position = await program.account.userPosition.fetch(
      positionAddress
    );

    // Repay remaining 200k MUSD
    await program.methods
      .repay(new anchor.BN(200_000_000_000))
      .accounts({
        owner: wallet.publicKey,
        vault: vaultAddress,
        position: positionAddress,
        userMusdAta: userMusdAta.address,
        vaultMusdAta: vaultMusdAta.address,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    // Withdraw all BTC
    await program.methods
      .withdrawCollateral(new anchor.BN(10_000_000_000))
      .accounts({
        owner: wallet.publicKey,
        vault: vaultAddress,
        position: positionAddress,
        userBtcAta: userBtcAta.address,
        vaultBtcAta: vaultBtcAta.address,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    const updatedPosition = await program.account.userPosition.fetch(
      positionAddress
    );
    console.log("BTC remaining:", updatedPosition.btcDeposited.toString());
    console.log("MUSD debt remaining:", updatedPosition.musdBorrowed.toString());
  });
});
