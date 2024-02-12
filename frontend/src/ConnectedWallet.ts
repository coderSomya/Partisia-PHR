import { ShardPutTransactionResponse } from "./client/ShardedClient";
import { Rpc, TransactionPayload } from "./client/TransactionData";

/**
 * Interface for a connected MPC wallet.
 */
export interface ConnectedWallet {
  readonly address: string;
  readonly signAndSendTransaction: (
    payload: TransactionPayload<Rpc>,
    cost?: string | number
  ) => Promise<ShardPutTransactionResponse>;
}
