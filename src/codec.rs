//! Codec implementations for Malachite consensus messages

use crate::context::{BaseProposalPart, BaseValue, MalachiteContext};
use crate::proto;
use bytes::Bytes;
use malachitebft_app::engine::util::streaming::StreamMessage;
use malachitebft_codec::Codec;
use malachitebft_core_consensus::{LivenessMsg, ProposedValue, SignedConsensusMsg};
use malachitebft_core_types::VoteType;
use malachitebft_proto::Error as ProtoError;
use malachitebft_signing_ed25519::Signature;
use malachitebft_sync as sync;
use prost::Message;

/// Protobuf codec for Malachite messages
#[derive(Copy, Clone, Debug)]
pub struct ProtoCodec;

// Helper functions for encoding/decoding
#[allow(dead_code)]
fn encode_signature(signature: &Signature) -> proto::Signature {
    proto::Signature {
        bytes: Bytes::copy_from_slice(signature.to_bytes().as_ref()),
    }
}

#[allow(dead_code)]
fn decode_signature(signature: proto::Signature) -> Result<Signature, ProtoError> {
    let bytes = <[u8; 64]>::try_from(signature.bytes.as_ref())
        .map_err(|_| ProtoError::Other("Invalid signature length".to_string()))?;
    Ok(Signature::from_bytes(bytes))
}

#[allow(dead_code)]
fn encode_votetype(vote_type: VoteType) -> proto::VoteType {
    match vote_type {
        VoteType::Prevote => proto::VoteType::Prevote,
        VoteType::Precommit => proto::VoteType::Precommit,
    }
}

#[allow(dead_code)]
fn decode_votetype(vote_type: i32) -> VoteType {
    match proto::VoteType::try_from(vote_type) {
        Ok(proto::VoteType::Prevote) => VoteType::Prevote,
        Ok(proto::VoteType::Precommit) => VoteType::Precommit,
        Err(_) => VoteType::Prevote, // Default fallback
    }
}

// For now, we'll implement only the essential codecs needed to compile
// In production, all of these would have proper implementations

impl Codec<BaseValue> for ProtoCodec {
    type Error = ProtoError;

    fn decode(&self, bytes: Bytes) -> Result<BaseValue, Self::Error> {
        let proto = proto::Value::decode(bytes.as_ref())?;
        Ok(BaseValue {
            data: proto.value.unwrap_or_default().to_vec(),
        })
    }

    fn encode(&self, msg: &BaseValue) -> Result<Bytes, Self::Error> {
        let proto = proto::Value {
            value: Some(Bytes::from(msg.data.clone())),
        };
        Ok(Bytes::from(proto.encode_to_vec()))
    }
}

impl Codec<BaseProposalPart> for ProtoCodec {
    type Error = ProtoError;

    fn decode(&self, _bytes: Bytes) -> Result<BaseProposalPart, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }

    fn encode(&self, _msg: &BaseProposalPart) -> Result<Bytes, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }
}

impl Codec<SignedConsensusMsg<MalachiteContext>> for ProtoCodec {
    type Error = ProtoError;

    fn decode(&self, _bytes: Bytes) -> Result<SignedConsensusMsg<MalachiteContext>, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }

    fn encode(&self, _msg: &SignedConsensusMsg<MalachiteContext>) -> Result<Bytes, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }
}

impl Codec<ProposedValue<MalachiteContext>> for ProtoCodec {
    type Error = ProtoError;

    fn decode(&self, _bytes: Bytes) -> Result<ProposedValue<MalachiteContext>, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }

    fn encode(&self, _msg: &ProposedValue<MalachiteContext>) -> Result<Bytes, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }
}

impl Codec<LivenessMsg<MalachiteContext>> for ProtoCodec {
    type Error = ProtoError;

    fn decode(&self, _bytes: Bytes) -> Result<LivenessMsg<MalachiteContext>, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }

    fn encode(&self, _msg: &LivenessMsg<MalachiteContext>) -> Result<Bytes, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }
}

impl Codec<StreamMessage<BaseProposalPart>> for ProtoCodec {
    type Error = ProtoError;

    fn decode(&self, _bytes: Bytes) -> Result<StreamMessage<BaseProposalPart>, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }

    fn encode(&self, _msg: &StreamMessage<BaseProposalPart>) -> Result<Bytes, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }
}

impl Codec<sync::Status<MalachiteContext>> for ProtoCodec {
    type Error = ProtoError;

    fn decode(&self, _bytes: Bytes) -> Result<sync::Status<MalachiteContext>, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }

    fn encode(&self, _msg: &sync::Status<MalachiteContext>) -> Result<Bytes, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }
}

impl Codec<sync::Request<MalachiteContext>> for ProtoCodec {
    type Error = ProtoError;

    fn decode(&self, _bytes: Bytes) -> Result<sync::Request<MalachiteContext>, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }

    fn encode(&self, _msg: &sync::Request<MalachiteContext>) -> Result<Bytes, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }
}

impl Codec<sync::Response<MalachiteContext>> for ProtoCodec {
    type Error = ProtoError;

    fn decode(&self, _bytes: Bytes) -> Result<sync::Response<MalachiteContext>, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }

    fn encode(&self, _msg: &sync::Response<MalachiteContext>) -> Result<Bytes, Self::Error> {
        // Placeholder implementation
        Err(ProtoError::Other("Not implemented".to_string()))
    }
}

