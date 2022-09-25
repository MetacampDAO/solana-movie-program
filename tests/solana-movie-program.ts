import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { SolanaMovieProgram } from "../target/types/solana_movie_program";

describe("solana-movie-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .SolanaMovieProgram as Program<SolanaMovieProgram>;
  const user = anchor.web3.Keypair.generate();
  const movieTitle = "Batman";

  it("initializes user account", async () => {
    // Add your test here.

    const airdropSellerSig = await provider.connection.requestAirdrop(
      user.publicKey,
      2e9
    );
    const latestSellerBlockhash =
      await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestSellerBlockhash.blockhash,
      lastValidBlockHeight: latestSellerBlockhash.lastValidBlockHeight,
      signature: airdropSellerSig,
    });
  });

  it("Adds review", async () => {
    // Add your test here.

    const [pda] = await web3.PublicKey.findProgramAddress(
      [
        user.publicKey.toBuffer(),
        Buffer.from(anchor.utils.bytes.utf8.encode(movieTitle)),
      ],
      new web3.PublicKey(program.programId)
    );
    const tx = await program.methods
      .addOrUpdateReview(0, movieTitle, 5, "Really good!")
      .accounts({
        initializer: user.publicKey,
        pdaAccount: pda,
      })
      .signers([user])
      .rpc();
    
    const initializedPdaAccount = await program.account.movieAccountState.fetch(pda);

    assert.ok(initializedPdaAccount.title === movieTitle)
    assert.ok(initializedPdaAccount.rating === 5)
    assert.ok(initializedPdaAccount.description === "Really good!")
  });

  it("Updates review", async () => {
    // Add your test here.

    const [pda] = await web3.PublicKey.findProgramAddress(
      [
        user.publicKey.toBuffer(),
        Buffer.from(anchor.utils.bytes.utf8.encode(movieTitle)),
      ],
      new web3.PublicKey(program.programId)
    );
    const tx = await program.methods
      .addOrUpdateReview(1, movieTitle, 1, "Really bad!")
      .accounts({
        initializer: user.publicKey,
        pdaAccount: pda,
      })
      .signers([user])
      .rpc();
    
    const initializedPdaAccount = await program.account.movieAccountState.fetch(pda);

    assert.ok(initializedPdaAccount.title === movieTitle)
    assert.ok(initializedPdaAccount.rating === 1)
    assert.ok(initializedPdaAccount.description === "Really bad!")
  });
});
