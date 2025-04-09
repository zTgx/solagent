import { SolanaAgentKit } from '../src';
import { Wallet } from '@solagent/wallet';
import { Connection } from '@solana/web3.js';

describe('SolanaAgentKit', () => {
  const mockRpcUrl = 'https://api.mainnet-beta.solana.com';
  const mockConfig = {
    heliusApiKey: 'test-key',
    jupiterFeeBps: 50
  };

  it('should initialize with default config', () => {
    const wallet = new Wallet();
    const agent = SolanaAgentKit.create(wallet, mockRpcUrl);
    
    expect(agent.wallet).toBeInstanceOf(Wallet);
    expect(agent.connection).toBeInstanceOf(Connection);
    expect(agent.config).toEqual({});
  });

  it('should initialize with custom config', () => {
    const wallet = new Wallet();
    const agent = SolanaAgentKit.create(wallet, mockRpcUrl, mockConfig);
    
    expect(agent.config.heliusApiKey).toBe('test-key');
    expect(agent.config.jupiterFeeBps).toBe(50);
  });

  it('should get balance', async () => {
    const wallet = new Wallet();
    const agent = SolanaAgentKit.create(wallet, mockRpcUrl);
    
    // Mock the connection method
    agent.connection.getBalance = jest.fn().mockResolvedValue(1000);
    
    const balance = await agent.getBalance();
    expect(balance).toBe(1000);
    expect(agent.connection.getBalance).toHaveBeenCalledWith(wallet.keypair.publicKey);
  });
});