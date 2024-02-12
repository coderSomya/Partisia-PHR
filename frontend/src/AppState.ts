import { ContractAbi } from "@partisiablockchain/abi-client";
import { BlockchainPublicKey } from "@partisiablockchain/zk-client";
import { ShardedClient } from "./client/ShardedClient";
import { TransactionApi } from "./client/TransactionApi";
import { ConnectedWallet } from "./ConnectedWallet";
// import { AverageSalaryApi } from "../playground/average-salary-frontend/src/main/contract/AverageSalaryApi";
import { updateContractState } from "./WalletIntegration";

export const CLIENT = new ShardedClient(
  "https://node1.testnet.partisiablockchain.com",
  ["Shard0", "Shard1", "Shard2"]
);

export let contractAddress: string | undefined;
export let currentAccount: ConnectedWallet | undefined;
export let contractAbi: ContractAbi | undefined;
export let averageApi: AverageSalaryApi | undefined;
export let engineKeys: BlockchainPublicKey[] | undefined;

export const setAccount = (account: ConnectedWallet | undefined) => {
  currentAccount = account;
  setAverageApi();
};

export const resetAccount = () => {
  currentAccount = undefined;
};

export const isConnected = () => {
  return currentAccount != null;
};

export const setContractAbi = (abi: ContractAbi | undefined) => {
  contractAbi = abi;
  setAverageApi();
};

export const getContractAbi = () => {
  return contractAbi;
};

export const setAverageApi = () => {
  if (
    currentAccount != undefined &&
    contractAbi != undefined &&
    engineKeys !== undefined
  ) {
    const transactionApi = new TransactionApi(
      currentAccount,
      updateContractState
    );
    averageApi = new AverageSalaryApi(
      transactionApi,
      currentAccount.address,
      contractAbi,
      engineKeys
    );
  }
};

export const getAverageApi = () => {
  return averageApi;
};

export const getEngineKeys = () => {
  return engineKeys;
};

export const setEngineKeys = (keys: BlockchainPublicKey[] | undefined) => {
  engineKeys = keys;
  setAverageApi();
};

export const getContractAddress = () => {
  return contractAddress;
};

export const setContractAddress = (address: string) => {
  contractAddress = address;
};
