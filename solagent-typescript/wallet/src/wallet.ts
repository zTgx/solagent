import * as fs from 'fs';
import * as dotenv from 'dotenv';
import { Keypair } from '@solana/web3.js';
import bs58 from 'bs58';

dotenv.config();

export class WalletError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'WalletError';
    }
}

/**
 * Represents a wallet containing a keypair and its corresponding public key.
 */
export class Wallet {
    /** The keypair associated with the wallet. This contains the private key. */
    public keypair: Keypair;
    /** The public key associated with the wallet. */
    public pubkey: string;

    /**
     * Creates a new wallet with a randomly generated keypair.
     */
    constructor() {
        this.keypair = Keypair.generate();
        this.pubkey = this.keypair.publicKey.toBase58();
    }

    /**
     * Creates a wallet from a private key stored in an environment variable.
     * @param variableName The name of the environment variable containing the private key.
     * @returns A Wallet instance.
     * @throws WalletError If the environment variable is not found or the private key is invalid.
     */
    static fromEnv(variableName: string): Wallet {
        const privateKey = process.env[variableName];
        if (!privateKey) {
            throw new WalletError(`Environment variable '${variableName}' not found`);
        }
        return Wallet.fromBase58(privateKey);
    }

    /**
     * Creates a wallet from a base58 encoded private key.
     * @param privateKey The base58 encoded private key.
     * @returns A Wallet instance.
     * @throws WalletError If the private key is invalid or not properly encoded.
     */
    static fromBase58(privateKey: string): Wallet {
        try {
            const secretKey = bs58.decode(privateKey);
            const keypair = Keypair.fromSecretKey(secretKey);
            const wallet = new Wallet();
            wallet.keypair = keypair;
            wallet.pubkey = keypair.publicKey.toBase58();
            return wallet;
        } catch (error) {
            if (error instanceof Error) {
                throw new WalletError(`Invalid private key: ${error.message}`);
            }
            throw new WalletError('Invalid private key');
        }
    }

    /**
     * Returns the base58 encoded private key of the wallet.
     * @returns The base58 encoded private key.
     */
    toBase58(): string {
        return bs58.encode(this.keypair.secretKey);
    }

    /**
     * Saves the wallet's private key to a file.
     * @param filePath The path to the file where the private key will be saved.
     * @throws WalletError If the file cannot be written.
     */
    saveToFile(filePath: string): void {
        try {
            const privateKey = this.toBase58();
            fs.writeFileSync(filePath, privateKey);
        } catch (error) {
            throw new WalletError(`Failed to save wallet to file: ${filePath}`);
        }
    }

    /**
     * Loads a wallet from a private key file.
     * @param filePath The path to the file containing the private key.
     * @returns A Wallet instance.
     * @throws WalletError If the file cannot be read or the private key is invalid.
     */
    static fromFile(filePath: string): Wallet {
        try {
          const privateKey = fs.readFileSync(filePath, 'utf-8').trim();
          try {
            return Wallet.fromBase58(privateKey);
          } catch (error) {
            throw new WalletError(`Invalid key in file: ${filePath}`);
          }
        } catch (error) {
          if (error instanceof WalletError) {
            throw error;
          }
          throw new WalletError(`Failed to read wallet file: ${filePath}`);
        }
      }
}