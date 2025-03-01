use crate::*;
use subtle::CtOption;

/// The ciphertext output from time lock encryption
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeCryptCiphertext<C: BlsSignatureImpl> {
    /// The `u` component
    #[serde(serialize_with = "traits::public_key::serialize::<C, _>")]
    #[serde(deserialize_with = "traits::public_key::deserialize::<C, _>")]
    pub u: <C as Pairing>::PublicKey,
    /// The `v` component
    pub v: [u8; 32],
    /// The `w` component
    pub w: Vec<u8>,
    /// The signature scheme used to generate this ciphertext
    pub scheme: SignatureSchemes,
}

impl<C: BlsSignatureImpl> From<&TimeCryptCiphertext<C>> for Vec<u8> {
    fn from(value: &TimeCryptCiphertext<C>) -> Self {
        serde_bare::to_vec(value).expect("failed to serialize time crypt ciphertext")
    }
}

impl<C: BlsSignatureImpl> TryFrom<&[u8]> for TimeCryptCiphertext<C> {
    type Error = BlsError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let output = serde_bare::from_slice(value)?;
        Ok(output)
    }
}

impl_from_derivatives!(TimeCryptCiphertext);

impl<C: BlsSignatureImpl> TimeCryptCiphertext<C> {
    /// Decrypt the time lock ciphertext using a signature over an identifier
    pub fn decrypt(&self, sig: &Signature<C>) -> CtOption<Vec<u8>> {
        let (s, valid) = match (sig, self.scheme) {
            (Signature::Basic(s), SignatureSchemes::Basic) => (*s, 1u8.into()),
            (Signature::MessageAugmentation(s), SignatureSchemes::MessageAugmentation) => {
                (*s, 1u8.into())
            }
            (Signature::ProofOfPossession(s), SignatureSchemes::ProofOfPossession) => {
                (*s, 1u8.into())
            }
            (_, _) => (<C as Pairing>::Signature::default(), 0u8.into()),
        };
        <C as BlsTimeCrypt>::unseal(self.u, &self.v, &self.w, s, valid)
    }
}
