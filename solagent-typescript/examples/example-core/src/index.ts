import { Wallet } from '@solagent/wallet';
import { SolanaAgentKit } from '@solagent/core';

async function main() {
  const wallet = new Wallet();
  console.log('Wallet address:', wallet.pubkey);

  const agent = SolanaAgentKit.create(wallet, 'https://api.mainnet-beta.solana.com');
  const balance = await agent.getBalance();
  console.log('Balance:', balance);
}

main().catch(console.error);