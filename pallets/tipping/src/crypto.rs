use frame_support::sp_runtime::{
	app_crypto::{app_crypto, sr25519},
	traits::Verify,
	MultiSignature, MultiSigner,
};
use sp_core::{crypto::KeyTypeId, sr25519::Signature as Sr25519Signature};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");

app_crypto!(sr25519, KEY_TYPE);

pub struct TestAuthId;

impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
	type RuntimeAppPublic = Public;
	type GenericSignature = sp_core::sr25519::Signature;
	type GenericPublic = sp_core::sr25519::Public;
}

// implemented for mock runtime in test
impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
	for TestAuthId
{
	type RuntimeAppPublic = Public;
	type GenericSignature = sp_core::sr25519::Signature;
	type GenericPublic = sp_core::sr25519::Public;
}
