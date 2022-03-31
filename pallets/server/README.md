## Server Pallet
The Server pallet handles logic for registering new server
This pallet exposes the following extrinsic calls:
### Register (Admin Only)
```rust
  pub fn register(
    origin: OriginFor<T>,
    account_id: AccountIdOf<T>,
    name: Vec<u8>,
  ) -> DispatchResultWithPostInfo
```
### Transfer Owner (Admin Only)
```rust
pub fn transfer_owner(
  origin: OriginFor<T>,
  account_id: AccountIdOf<T>,
  server_id: HashOf<T>,
  new_owner: AccountIdOf<T>,
) -> DispatchResultWithPostInfo
```
### Update name (Admin Only)
```rust
pub fn update_name(
  origin: OriginFor<T>,
  account_id: AccountIdOf<T>,
  server_id: HashOf<T>,
  new_name: Vec<u8>,
) -> DispatchResultWithPostInfo
```
### Unregister (Admin Only)
```rust
pub fn unregister(
  origin: OriginFor<T>,
  account_id: AccountIdOf<T>,
  server_id: ServerIdOf<T>,
) -> DispatchResultWithPostInfo
```
### Transfer Admin Key (Admin Only)
```rust
pub fn transfer_admin_key(
  origin: OriginFor<T>,
  account_id: AccountIdOf<T>,
) -> DispatchResultWithPostInfo
```
