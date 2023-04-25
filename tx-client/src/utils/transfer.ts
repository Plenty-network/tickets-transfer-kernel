import { blake2bHex } from "blakejs";
import { Parser } from "@taquito/michel-codec";
import { TezosToolkit } from "@taquito/taquito";
import { Schema } from "@taquito/michelson-encoder";

import { Token, TransferContent, TransferMessage } from "../types";

export interface GetTransferContentOptions {
  token: Token;
  destination: string;
  amount: string;
}

export interface GetTransferMessageBytesOptions {
  pkey: string;
  nonce: number;
  signature: string;
  transferContent: TransferContent;
}

export abstract class Transfer {
  static getTransferMessageBytes(options: GetTransferMessageBytesOptions): string {
    const transferMessage: TransferMessage = {
      Transfer: {
        pkey: {
          Ed25519: options.pkey,
        },
        signature: {
          Ed25519: options.signature,
        },
        inner: {
          nonce: options.nonce,
          content: options.transferContent,
        },
      },
    };

    return Buffer.from(JSON.stringify(transferMessage)).toString("hex");
  }

  static async getTransferContent(
    tezos: TezosToolkit,
    options: GetTransferContentOptions
  ): Promise<TransferContent> {
    const michelineTokenType = JSON.parse(
      JSON.stringify(
        new Parser().parseMichelineExpression(`(or (address %fa12) (pair %fa2 address nat))`)
      )
    );

    const schema = new Schema(michelineTokenType);

    const michelineTokenData = schema.Encode(options.token);

    const tokenBytes = await tezos.rpc.packData({
      data: michelineTokenData,
      type: michelineTokenType,
    });

    return {
      token: Array.from(Buffer.from(tokenBytes.packed, "hex")),
      destination: {
        Tz1: options.destination,
      },
      amount: options.amount,
    };
  }

  static getHashToSign(nonce: number, transferContent: TransferContent): string {
    const _nonce = nonce.toString(18).padStart(8, "0").toUpperCase();
    return blake2bHex(
      `${_nonce}${transferContent.token}${transferContent.destination}${transferContent.amount}`,
      undefined,
      32
    );
  }
}
