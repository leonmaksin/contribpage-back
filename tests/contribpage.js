const anchor = require('@project-serum/anchor');
const BN = require('bn.js');

const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log('🚀 Starting test...')

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Contribpage;
  const baseAccount = anchor.web3.Keypair.generate();

  let tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature:", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Item Count', account.totalItems.toString())
  console.log('💰 Donation Total', account.donationTotal.toString())

  await program.rpc.addItem("sample message", "John Wick", 64, new BN(1), "aliceblue", new BN(2),
                            new BN(3), new BN(new Date().getTime()),
    {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });
  
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Item Count', account.totalItems.toString())
  console.log('👀 Item List', account.itemList)
  console.log('💰 Donation Total', account.donationTotal.toString())

  console.log('🏁 Test finished')
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
