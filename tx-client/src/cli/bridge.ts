import BigNumber from "bignumber.js";
import { InMemorySigner } from "@taquito/signer";
import { OpKind, TezosToolkit } from "@taquito/taquito";

import { Token } from "../utils/token";
import { Bridge } from "../utils/bridge";

const tezos = new TezosToolkit(`https://${process.argv[2]}.smartpy.io`);

tezos.setProvider({
  signer: new InMemorySigner(process.env.PRIVATE_KEY as string),
});

(async () => {
  try {
    const bridgeInstance = await tezos.contract.at(process.argv[3]);
    const tokenStandard = process.argv[4];

    console.log(`> Transferring to L2`);

    switch (tokenStandard) {
      case "fa12": {
        const tokenAddress = process.argv[5];
        const amount = new BigNumber(process.argv[6]);
        const tokenInstance = await tezos.contract.at(tokenAddress);
        const op = await tezos.contract
          .batch([
            {
              kind: OpKind.TRANSACTION,
              ...Token.approveFA12(tokenInstance, {
                spender: bridgeInstance.address,
                value: amount,
              }),
            },
            {
              kind: OpKind.TRANSACTION,
              ...Bridge.deposit(bridgeInstance, { 0: { fa12: tokenAddress }, 1: amount }),
            },
          ])
          .send();
        await op.confirmation();
        console.log(`>> Operation hash: ${op.hash}`);
        break;
      }
      case "fa2": {
        const tokenAddress = process.argv[5];
        const tokenId = parseInt(process.argv[6]);
        const amount = new BigNumber(process.argv[7]);
        const tokenInstance = await tezos.contract.at(tokenAddress);

        const op = await tezos.contract
          .batch([
            {
              kind: OpKind.TRANSACTION,
              ...Token.updateOperatorsFA2(tokenInstance, [
                {
                  add_operator: {
                    owner: await tezos.wallet.pkh(),
                    token_id: tokenId,
                    operator: bridgeInstance.address,
                  },
                },
              ]),
            },
            {
              kind: OpKind.TRANSACTION,
              ...Bridge.deposit(bridgeInstance, {
                0: { fa2: { 1: tokenAddress, 2: tokenId } },
                1: amount,
              }),
            },
          ])
          .send();
        await op.confirmation();
        console.log(`>> Operation hash: ${op.hash}`);
        break;
      }
      default: {
        throw "Invalid cli arguments";
      }
    }
  } catch (err) {
    console.error(err);
  }
})();
