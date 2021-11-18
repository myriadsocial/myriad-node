
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
