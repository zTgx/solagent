import { Connection } from '@solana/web3.js';
import { Wallet } from '@solagent/wallet';
import { Config } from './types';

export class SolanaAgentKit {
  constructor(
    public readonly wallet: Wallet,
    public readonly connection: Connection,
    public readonly config: Config = {}
  ) {}

  static create(
    wallet: Wallet,
    rpcUrl: string,
    config: Config = {}
  ): SolanaAgentKit {
    const connection = new Connection(rpcUrl);
    return new SolanaAgentKit(wallet, connection, config);
  }

  // Example utility method
  async getBalance(): Promise<number> {
    return this.connection.getBalance(this.wallet.keypair.publicKey);
  }
}