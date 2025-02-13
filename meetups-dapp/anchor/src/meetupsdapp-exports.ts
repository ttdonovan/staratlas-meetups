// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import MeetupsdappIDL from '../target/idl/meetupsdapp.json'
import type { Meetupsdapp } from '../target/types/meetupsdapp'

// Re-export the generated IDL and type
export { Meetupsdapp, MeetupsdappIDL }

// The programId is imported from the program IDL.
export const MEETUPSDAPP_PROGRAM_ID = new PublicKey(MeetupsdappIDL.address)

// This is a helper function to get the Meetupsdapp Anchor program.
export function getMeetupsdappProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...MeetupsdappIDL, address: address ? address.toBase58() : MeetupsdappIDL.address } as Meetupsdapp, provider)
}

// This is a helper function to get the program ID for the Meetupsdapp program depending on the cluster.
export function getMeetupsdappProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Meetupsdapp program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return MEETUPSDAPP_PROGRAM_ID
  }
}
