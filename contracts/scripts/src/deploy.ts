import fs from "fs";
import { TezosToolkit } from "@taquito/taquito";
import { InMemorySigner } from "@taquito/signer";

const tezos = new TezosToolkit(`https://${process.argv[2]}.smartpy.io`);

tezos.setProvider({
  signer: new InMemorySigner(process.env.PRIVATE_KEY as string),
});

(async () => {
  try {
    console.log("> Deploying smart rollup bridge...");

    // Load code
    const code = fs.readFileSync(`${__dirname}/../../michelson/bridge.tz`).toString();

    // Deploy
    const op = await tezos.contract.originate({
      code,
      storage: "KT1ThEdxfUcWUwqsdergy3QnbCWGHSUHeHJq",
    });
    await op.confirmation(1);

    console.log(`>> Deployed at: ${op.contractAddress}`);
  } catch (err) {
    console.error(err);
  }
})();
