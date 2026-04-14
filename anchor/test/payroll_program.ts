// Import Anchor framework and web3.js libraries
import * as anchor from '@coral-xyz/anchor'
import { Program, BN } from '@coral-xyz/anchor'
import { PayrollProgram } from '../target/types/payroll_program'
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js'
import { assert } from 'chai'

describe('Payroll Program - Comprehensive Tests', () => {
    // Provider connects us to the blockchain (local or devnet)
    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider)

    // Get the program instance to call its methods
    const program = anchor.workspace.PayrollProgram as Program<PayrollProgram>

    // The authority is whoever is running the tests (wallet owner)
    const authority = provider.wallet as anchor.Wallet

    // Organization name and its derived account address (PDA)
    const orgName = 'TechCorp'
    let orgPda: PublicKey
    let orgBump: number

    // Create three workers for testing
    const worker1 = Keypair.generate()
    const worker2 = Keypair.generate()
    const worker3 = Keypair.generate()

    // PDAs (Program Derived Addresses) for each worker
    let worker1Pda: PublicKey
    let worker2Pda: PublicKey
    let worker3Pda: PublicKey

    // Salary amounts in lamports (1 SOL = 1 billion lamports)
    const salary1 = new BN(1 * LAMPORTS_PER_SOL)
    const salary2 = new BN(1.5 * LAMPORTS_PER_SOL)
    const salary3 = new BN(2 * LAMPORTS_PER_SOL)

    // Get the SOL balance of any account
    async function getBalance(pubkey: PublicKey): Promise<number> {
        return await provider.connection.getBalance(pubkey)
    }

    // Airdrop SOL to an account for test
    async function airdrop(pubkey: PublicKey, amount: number = 2 * LAMPORTS_PER_SOL) {
        const sig = await provider.connection.requestAirdrop(pubkey, amount)
        await provider.connection.confirmTransaction(sig)
    }

    before('Setup test accounts', async() => {
        // Derive the organization PDA using program-specific seeds
        // Seeds: 'org' + authority wallet + organization name
        
        ;[orgPda, orgBump] =    // Derive each worker's PDA
        // Seeds: 'worker' + org address + worker public key
        PublicKey.findProgramAddressSync(
            [
                Buffer.from('org'),
                authority.publicKey.toBuffer(),
                Buffer.from(orgName),
            ],
            program.programId
        )

        // Derive each worker's PDA
        // Seeds: 'worker' + org address + worker public key
        ;[worker1Pda] = PublicKey.findProgramAddressSync(
            [Buffer.from('worker'), orgPda.toBuffer(), worker1.publicKey.toBuffer()],
            program.programId
        )
        ;[worker2Pda] = PublicKey.findProgramAddressSync(
            [Buffer.from('worker'), orgPda.toBuffer(), worker2.publicKey.toBuffer()],
            program.programId
        )
        ;[worker3Pda] = PublicKey.findProgramAddressSync(
            [Buffer.from('worker'), orgPda.toBuffer(), worker3.publicKey.toBuffer()],
            program.programId
        )
    })
})