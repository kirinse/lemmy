use crate::{
  newtypes::LocalUserId,
  schema::password_reset_request::dsl::*,
  source::password_reset_request::*,
  traits::Crud,
};
use diesel::{dsl::*, result::Error, PgConnection, *};
use sha2::{Digest, Sha256};

impl Crud for PasswordResetRequest {
  type InsertForm = PasswordResetRequestForm;
  type UpdateForm = PasswordResetRequestForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, password_reset_request_id: i32) -> Result<Self, Error> {
    password_reset_request
      .find(password_reset_request_id)
      .first::<Self>(conn)
  }
  fn create(conn: &mut PgConnection, form: &PasswordResetRequestForm) -> Result<Self, Error> {
    insert_into(password_reset_request)
      .values(form)
      .get_result::<Self>(conn)
  }
  fn update(
    conn: &mut PgConnection,
    password_reset_request_id: i32,
    form: &PasswordResetRequestForm,
  ) -> Result<Self, Error> {
    diesel::update(password_reset_request.find(password_reset_request_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl PasswordResetRequest {
  pub fn create_token(
    conn: &mut PgConnection,
    from_local_user_id: LocalUserId,
    token: &str,
  ) -> Result<PasswordResetRequest, Error> {
    let mut hasher = Sha256::new();
    hasher.update(token);
    let token_hash: String = bytes_to_hex(hasher.finalize().to_vec());

    let form = PasswordResetRequestForm {
      local_user_id: from_local_user_id,
      token_encrypted: token_hash,
    };

    Self::create(conn, &form)
  }
  pub fn read_from_token(
    conn: &mut PgConnection,
    token: &str,
  ) -> Result<PasswordResetRequest, Error> {
    let mut hasher = Sha256::new();
    hasher.update(token);
    let token_hash: String = bytes_to_hex(hasher.finalize().to_vec());
    password_reset_request
      .filter(token_encrypted.eq(token_hash))
      .filter(published.gt(now - 1.days()))
      .first::<Self>(conn)
  }
}

fn bytes_to_hex(bytes: Vec<u8>) -> String {
  let mut str = String::new();
  for byte in bytes {
    str = format!("{}{:02x}", str, byte);
  }
  str
}

#[cfg(test)]
mod tests {
  use crate::{
    source::{
      instance::Instance,
      local_user::{LocalUser, LocalUserInsertForm},
      password_reset_request::PasswordResetRequest,
      person::*,
    },
    traits::Crud,
    utils::establish_unpooled_connection,
  };
  use serial_test::serial;

  #[test]
  #[serial]
  fn test_crud() {
    let conn = &mut establish_unpooled_connection();

    let inserted_instance = Instance::create(conn, "my_domain.tld").unwrap();

    let new_person = PersonInsertForm::builder()
      .name("thommy prw".into())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_person = Person::create(conn, &new_person).unwrap();

    let new_local_user = LocalUserInsertForm::builder()
      .person_id(inserted_person.id)
      .password_encrypted("pass".to_string())
      .build();

    let inserted_local_user = LocalUser::create(conn, &new_local_user).unwrap();

    let token = "nope";
    let token_encrypted_ = "ca3704aa0b06f5954c79ee837faa152d84d6b2d42838f0637a15eda8337dbdce";

    let inserted_password_reset_request =
      PasswordResetRequest::create_token(conn, inserted_local_user.id, token).unwrap();

    let expected_password_reset_request = PasswordResetRequest {
      id: inserted_password_reset_request.id,
      local_user_id: inserted_local_user.id,
      token_encrypted: token_encrypted_.to_string(),
      published: inserted_password_reset_request.published,
    };

    let read_password_reset_request = PasswordResetRequest::read_from_token(conn, token).unwrap();
    let num_deleted = Person::delete(conn, inserted_person.id).unwrap();
    Instance::delete(conn, inserted_instance.id).unwrap();

    assert_eq!(expected_password_reset_request, read_password_reset_request);
    assert_eq!(
      expected_password_reset_request,
      inserted_password_reset_request
    );
    assert_eq!(1, num_deleted);
  }
}
