import { DefaultContractType } from "@taquito/taquito";

export type Contract = DefaultContractType;

export interface FA2 {
  1: string;
  2: number;
}

export type FA12 = string;

export type Token = { fa12: FA12 } | { fa2: FA2 };

export interface TransferContent {
  token: Array<number>;
  destination: {
    Tz1: string;
  };
  amount: string;
}

export interface TransferMessage {
  Transfer: {
    pkey: {
      Ed25519: string;
    };
    signature: {
      Ed25519: string;
    };
    inner: {
      nonce: number;
      content: TransferContent;
    };
  };
}
