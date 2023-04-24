import { TezosToolkit } from "@taquito/taquito";
import { InMemorySigner } from "@taquito/signer";

const tezos = new TezosToolkit(`https://${process.argv[2]}.smartpy.io`);

tezos.setProvider({
  signer: new InMemorySigner(process.env.PRIVATE_KEY as string),
});

(async () => {
  try {
    console.log("> Initialising smart rollup bridge...");

    // Bridge instance
    const instance = await tezos.contract.at(process.argv[3]);

    // Init
    const op = await instance.methods.initialise(process.argv[4]).send();
    await op.confirmation(1);

    console.log(`>> Operation hash: ${op.hash}`);
  } catch (err) {
    console.error(err);
  }
})();
