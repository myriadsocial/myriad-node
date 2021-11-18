
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
