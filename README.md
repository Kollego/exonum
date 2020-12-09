Forked from [exomun](https://github.com/exonum/exonum) for hse assignment
Author: Kolesnikov Egor BSE171 + Iskhakova Lilia BSE172

What has beed done:
- First task (added `create_transfer_with_approve` that freezes some amount of balance of sender)
- Second task (added `finish_transfer_with_approve` that frees freezed balance of sender's balance and add amount to receiver's balance)
- Third task (added `wallet_history_approve` at SchemaImpl and changed logic of approved transactions. + Added endpoint to get approved transactions)