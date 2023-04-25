import { TezosToolkit } from "@taquito/taquito";
import { InMemorySigner } from "@taquito/signer";

import { Transfer } from "../utils/transfer";
import { RollupClient } from "../utils/rollup";

const tezos = new TezosToolkit(`https://${process.argv[2]}.smartpy.io`);

tezos.setProvider({
  signer: new InMemorySigner(process.env.PRIVATE_KEY as string),
});

(async () => {
  try {
    const across = process.argv[3]; // Send the transfer across the sequencer or the L1 inbox
    const nonce = parseInt(process.argv[4]); // TODO: fetch directly from rollup/sequencer state
    const tokenStandard = process.argv[5];

    console.log(`> Sending transfer request`);

    let transferContent, signature;

    switch (tokenStandard) {
      case "fa12": {
        const tokenAddress = process.argv[6];
        const amount = process.argv[7];
        const destination = process.argv[8];

        transferContent = await Transfer.getTransferContent(tezos, {
          token: { fa12: tokenAddress },
          destination,
          amount,
        });

        const hash = Transfer.getHashToSign(nonce, transferContent);
        signature = (await tezos.signer.sign(hash)).prefixSig;
        break;
      }
      case "fa2": {
        const tokenAddress = process.argv[6];
        const tokenId = parseInt(process.argv[7]);
        const amount = process.argv[8];
        const destination = process.argv[9];

        transferContent = await Transfer.getTransferContent(tezos, {
          token: { fa2: { 1: tokenAddress, 2: tokenId } },
          destination,
          amount,
        });

        const hash = Transfer.getHashToSign(nonce, transferContent);
        signature = (await tezos.signer.sign(hash)).prefixSig;
        break;
      }
      default: {
        throw "Invalid cli arguments";
      }
    }

    const transferBytes = Transfer.getTransferMessageBytes({
      pkey: await tezos.signer.publicKey(),
      nonce,
      signature,
      transferContent,
    });

    const op = await RollupClient.send(tezos, { message: transferBytes, specialByte: "55" });
    op.confirmation();

    console.log(`>> Message sent to ${across}: ${op.hash}`);
  } catch (err) {
    console.error(err);
  }
})();
