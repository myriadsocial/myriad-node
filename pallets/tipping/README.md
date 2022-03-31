## Tipping Pallet
The Tipping pallet handles logic for holding balance of another user in PalletId.
This pallet exposes the following extrinsic calls:
### Send Tip
```rust
pub fn send_tip(
  origin: OriginFor<T>,
  tip_balance_info: TipsBalanceInfo<T>,
  amount: BalanceOf<T>,
) -> DispatchResultWithPostInfo
```
### Claim Tip
```rust
pub fn claim_tip(
  origin: OriginFor<T>,
  tip_balance_info: TipsBalanceInfo<T>,
) -> DispatchResultWithPostInfo
```

### Claim Reference
```rust
  pub fn claim_reference(
    origin: OriginFor<T>,
    tips_balance_info: TipsBalanceInfoOf<T>,
    reference_id: ReferenceId,
    reference_type: ReferenceType,
    account_id: Option<AccountId>,
  ) -> DispatchResultWithPostInfo
```

