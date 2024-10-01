use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

/// Event output model
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::events)]
pub struct EventOutput {
  /// The event id
  pub id: String,
  /// The event description
  pub description:Option<String>,
  /// The event completed status
  pub completed: bool,

  // /// Amount of groups
  // pub total_groups: i32,
  // /// Amount of participants
  // pub total_participants: i32
}

/// Event detail output model
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct EventDetailOutput {
  /// The event id
  pub id: i32,
  /// The event description
  pub description: String,
  /// The event completed status
  pub completed: bool,
  /// Amount of groups
  pub total_groups: i32,
  /// Amount of participants
  pub total_participants: i32,
  /// RSA public key
  pub public_key: String,
  /// AES-Encrypted PKCS#8 RSA private key
  pub encrypted_private_key: String
}