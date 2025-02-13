'use client'

import { getMeetupsdappProgram, getMeetupsdappProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

export function useMeetupsdappProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getMeetupsdappProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getMeetupsdappProgram(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['meetupsdapp', 'all', { cluster }],
    queryFn: () => program.account.meetupsdapp.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initialize = useMutation({
    mutationKey: ['meetupsdapp', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ meetupsdapp: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
      return accounts.refetch()
    },
    onError: () => toast.error('Failed to initialize account'),
  })

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    initialize,
  }
}

export function useMeetupsdappProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useMeetupsdappProgram()

  const accountQuery = useQuery({
    queryKey: ['meetupsdapp', 'fetch', { cluster, account }],
    queryFn: () => program.account.meetupsdapp.fetch(account),
  })

  const closeMutation = useMutation({
    mutationKey: ['meetupsdapp', 'close', { cluster, account }],
    mutationFn: () => program.methods.close().accounts({ meetupsdapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accounts.refetch()
    },
  })

  const decrementMutation = useMutation({
    mutationKey: ['meetupsdapp', 'decrement', { cluster, account }],
    mutationFn: () => program.methods.decrement().accounts({ meetupsdapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const incrementMutation = useMutation({
    mutationKey: ['meetupsdapp', 'increment', { cluster, account }],
    mutationFn: () => program.methods.increment().accounts({ meetupsdapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const setMutation = useMutation({
    mutationKey: ['meetupsdapp', 'set', { cluster, account }],
    mutationFn: (value: number) => program.methods.set(value).accounts({ meetupsdapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  return {
    accountQuery,
    closeMutation,
    decrementMutation,
    incrementMutation,
    setMutation,
  }
}
