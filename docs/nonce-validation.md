# Mempool nonce ordering and recovery

The investigation of issue #1400 identified two paths that could package a
transaction before its sender's preceding nonce: WAL recovery treated every
transaction as immediately executable, and the fee/nonce comparator was not
transitive across senders. The previous cursor rebuild fix only updated accounts
affected by committed transactions; it could not repair an incorrectly initialized
cursor after an empty block. These defects are covered independently; WAL recovery
has not been established as the cause of the production incident.

## Recovery and packaging

WAL recovery uses the same authorization and transaction checks as admission,
including the current chain transaction size limit and committed-transaction check.
It retains future nonces until their predecessors arrive and restores system
transactions to their own bucket.

Packaging compares the fees of each sender's next transaction. A successor becomes
eligible only after its predecessor has been selected, including when the block
limit truncates the package. It defers ordinary transactions when a system
transaction in the same package may change that account's nonce.

Admission and flush share an asynchronous read/write lock so a completed flush
cannot be followed by insertion using an older authorization result. Parallel
validation tasks retain the lock even if their caller is cancelled. Flush refreshes
nonces from committed account state, including accounts affected only by system
transactions. NativeToken transactions affect the calldata target; other system
transactions affect the sender.

Fetching full transactions preserves the requested hash order when some
transactions are in the mempool and others must be loaded from storage. Missing
transactions, unexpected response lengths, and hash mismatches return an error.

## Deployment and compatibility

These changes take effect when each node starts the new binary. Upgrade all
validators to cover every normal proposer path. No new hardfork feature or common
activation height is required, and Medusa does not need an executor change for
this mempool upgrade.

Block acceptance, execution, historical replay, genesis flags, and the existing
Andromeda announcement behavior retain their previous rules. The patch does not
add consensus-level nonce enforcement, rewrite historical blocks, or repair
transactions that have already executed. An invalid proposal produced outside
the corrected mempool path is still subject to the existing validation rules.

## Validation

Regression tests cover future/stale/invalid WAL entries, system WAL routing,
mixed-fee sender ordering at every block limit, system nonce target refresh,
concurrent first admission and flush, cancellation of parallel validation,
chain transaction size limits during startup, and mixed mempool/storage reads.

Use `make check-fmt`, `make clippy`, and `make test-in-separate-processes`.
The separate-process test target avoids interference from process-global executor
system databases.
