## Escrow Pallet
The Escrow pallet handles logic for holding balance of another user in PalletId.
This pallet exposes the following extrinsic calls:
### Sent Tips
```rust
pub fn send_tip(
  origin: OriginFor<T>,
  post: Post,
  currency_id: CurrencyId,
  amount: u128,
) -> DispatchResult
```
