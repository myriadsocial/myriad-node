# Description of Myriad Pallets
Myriad blockchain runtime uses the following custom pallets to handle its business logic

## Currency Pallet
The Currency pallet handles logic for create new currency, update balance, and transfer.
This pallet exposes the following extrinsic calls:
### Create Currency
```rust
pub fn add_currency(
  origin: OriginFor<T>,
	currency_id: Vec<u8>,
	decimal: u16,
	rpc_url: Vec<u8>,
	native: bool
) -> DispatchResult
```
### Update Balance
```rust
pub fn update_balance(
  origin: OriginFor<T>,
  to: T::AccountId,
  currency_id: CurrencyId,
  amount: u128,
) -> DispatchResult
```
### Transfer
```rust
pub fn transfer(
  origin: OriginFor<T>,
  to: T::AccountId,
  currency_id: CurrencyId,
  amount: u128,
) -> DispatchResult
```

## Platform Pallet
The Platform pallet handles logic for updating platform collection in Myriad.
This pallet exposes the following extrinsic calls:
### Add Platform
```rust
pub fn add_platform(origin: OriginFor<T>, platform: Platform) -> DispatchResult
```

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

## Post Pallet
The Post pallet handles logic for mapping each post and people with balance.
This pallet expoeses the following extrinsic calls:
### Insert Balance
```rust
pub fn insert_balance(
  origin: OriginFor<T>,
	post_info: PostInfo,
) -> DispatchResultWithPostInfo
```

