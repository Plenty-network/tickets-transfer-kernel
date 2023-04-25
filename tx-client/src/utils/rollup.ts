import { TezosToolkit } from "@taquito/taquito";
import { SmartRollupAddMessagesOperation } from "@taquito/taquito/dist/types/operations/smart-rollup-add-messages-operation";

interface SendOptions {
  message: string;
  specialByte: string;
}

export abstract class RollupClient {
  static async send(
    tezos: TezosToolkit,
    options: SendOptions
  ): Promise<SmartRollupAddMessagesOperation> {
    return await tezos.contract.smartRollupAddMessages({
      message: [options.specialByte.concat(options.message)],
    });
  }
}
