import { Wallet, WalletError } from '../src/index';
import * as fs from 'fs';
import { Keypair } from '@solana/web3.js';
import bs58 from 'bs58';

// Mock environment variables
const mockPrivateKey = bs58.encode(Keypair.generate().secretKey);
process.env.TEST_PRIVATE_KEY = mockPrivateKey;
process.env.EMPTY_PRIVATE_KEY = '';

describe('Wallet', () => {
  describe('constructor', () => {
    it('should create a new wallet with valid keypair', () => {
      const wallet = new Wallet();
      
      expect(wallet.keypair).toBeInstanceOf(Keypair);
      expect(wallet.pubkey).toMatch(/^[1-9A-HJ-NP-Za-km-z]{32,44}$/); // Base58 regex
    });
  });

  describe('fromEnv', () => {
    it('should create wallet from valid environment variable', () => {
      const wallet = Wallet.fromEnv('TEST_PRIVATE_KEY');
      
      expect(wallet.pubkey).toBeDefined();
      expect(wallet.toBase58()).toBe(mockPrivateKey);
    });

    it('should throw when environment variable is missing', () => {
      expect(() => Wallet.fromEnv('NON_EXISTENT_VAR')).toThrow(
        new WalletError("Environment variable 'NON_EXISTENT_VAR' not found")
      );
    });

    it('should throw when environment variable is empty', () => {
      expect(() => Wallet.fromEnv('EMPTY_PRIVATE_KEY')).toThrow(
        new WalletError("Environment variable 'EMPTY_PRIVATE_KEY' not found")
      );
    });
  });

  describe('fromBase58', () => {
    it('should create wallet from valid base58 private key', () => {
      const testKeypair = Keypair.generate();
      const testPrivateKey = bs58.encode(testKeypair.secretKey);
      
      const wallet = Wallet.fromBase58(testPrivateKey);
      
      expect(wallet.pubkey).toBe(testKeypair.publicKey.toBase58());
      expect(wallet.toBase58()).toBe(testPrivateKey);
    });

    it('should throw when private key is invalid', () => {
      const invalidKey = 'not_a_valid_base58_key';
      
      expect(() => Wallet.fromBase58(invalidKey)).toThrow(
        new WalletError('Invalid private key: Non-base58 character')
      );
    });

    it('should throw when private key has wrong length', () => {
      const shortKey = bs58.encode(Buffer.from('tooshort'));
      
      expect(() => Wallet.fromBase58(shortKey)).toThrow(
        /Invalid private key/ // Now matches any error containing this phrase
      );
    });
  });

  describe('toBase58', () => {
    it('should return valid base58 private key', () => {
      const wallet = new Wallet();
      const privateKey = wallet.toBase58();
      
      // Verify it's valid base58
      expect(() => bs58.decode(privateKey)).not.toThrow();
      
      // Verify round-trip conversion
      const wallet2 = Wallet.fromBase58(privateKey);
      expect(wallet2.pubkey).toBe(wallet.pubkey);
    });
  });

  describe('saveToFile and fromFile', () => {
    const testFilePath = 'test-wallet.key';
    
    afterEach(() => {
      if (fs.existsSync(testFilePath)) {
        fs.unlinkSync(testFilePath);
      }
    });

    it('should save and load wallet from file', () => {
      const wallet1 = new Wallet();
      wallet1.saveToFile(testFilePath);
      
      expect(fs.existsSync(testFilePath)).toBe(true);
      
      const wallet2 = Wallet.fromFile(testFilePath);
      
      expect(wallet2.pubkey).toBe(wallet1.pubkey);
      expect(wallet2.toBase58()).toBe(wallet1.toBase58());
    });

    it('should throw when saving to invalid path', () => {
      const wallet = new Wallet();
      const invalidPath = '/nonexistent/path/wallet.key';
      
      expect(() => wallet.saveToFile(invalidPath)).toThrow(
        new WalletError(`Failed to save wallet to file: ${invalidPath}`)
      );
    });

    it('should throw when loading from nonexistent file', () => {
      expect(() => Wallet.fromFile('nonexistent.key')).toThrow(
        new WalletError('Failed to read wallet file: nonexistent.key')
      );
    });

    it('should throw when file contains invalid key', () => {
      fs.writeFileSync(testFilePath, 'invalid_key_content');
      
      expect(() => Wallet.fromFile(testFilePath)).toThrow(
        new WalletError(`Invalid key in file: ${testFilePath}`)
      );
    });
  });

  describe('WalletError', () => {
    it('should maintain proper error type', () => {
      const error = new WalletError('Test error');
      
      expect(error).toBeInstanceOf(Error);
      expect(error).toBeInstanceOf(WalletError);
      expect(error.name).toBe('WalletError');
      expect(error.message).toBe('Test error');
    });
  });
});