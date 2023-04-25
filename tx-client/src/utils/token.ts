import BigNumber from "bignumber.js";
import { TransferParams } from "@taquito/taquito";

import { Contract } from "../types";

export interface ApproveFA12Options {
  spender: string;
  value: BigNumber | number | string;
}

export interface OperatorKey {
  owner: string;
  operator: string;
  token_id: number;
}

export type UpdateOperatorFA2Options = Array<{ [key: string]: OperatorKey }>;

export abstract class Token {
  static approveFA12(token: Contract, options: ApproveFA12Options): TransferParams {
    try {
      return token.methodsObject.approve(options).toTransferParams();
    } catch (err) {
      throw err;
    }
  }

  static updateOperatorsFA2(token: Contract, options: UpdateOperatorFA2Options): TransferParams {
    try {
      return token.methodsObject.update_operators(options).toTransferParams();
    } catch (err) {
      throw err;
    }
  }
}
