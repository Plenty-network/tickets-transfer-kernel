import BigNumber from "bignumber.js";
import { Contract, Token } from "../types";
import { TransferParams } from "@taquito/taquito";

export interface DepositOptions {
  0: Token;
  1: BigNumber;
}

export abstract class Bridge {
  static deposit(bridge: Contract, options: DepositOptions): TransferParams {
    return bridge.methodsObject.deposit(options).toTransferParams();
  }
}
