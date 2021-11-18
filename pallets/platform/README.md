## Platform Pallet
The Platform pallet handles logic for updating platform collection in Myriad.
This pallet exposes the following extrinsic calls:
### Add Platform
```rust
pub fn add_platform(origin: OriginFor<T>, platform: Platform) -> DispatchResult
```
