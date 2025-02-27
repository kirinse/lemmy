use crate::structs::RegistrationApplicationView;
use diesel::{dsl::count, result::Error, *};
use lemmy_db_schema::{
  schema::{local_user, person, registration_application},
  source::{
    local_user::{LocalUser, LocalUserSettings},
    person::{Person, PersonSafe},
    registration_application::RegistrationApplication,
  },
  traits::{ToSafe, ToSafeSettings, ViewToVec},
  utils::limit_and_offset,
};
use typed_builder::TypedBuilder;

type RegistrationApplicationViewTuple = (
  RegistrationApplication,
  LocalUserSettings,
  PersonSafe,
  Option<PersonSafe>,
);

impl RegistrationApplicationView {
  pub fn read(conn: &mut PgConnection, registration_application_id: i32) -> Result<Self, Error> {
    let person_alias_1 = diesel::alias!(person as person1);

    let (registration_application, creator_local_user, creator, admin) =
      registration_application::table
        .find(registration_application_id)
        .inner_join(
          local_user::table.on(registration_application::local_user_id.eq(local_user::id)),
        )
        .inner_join(person::table.on(local_user::person_id.eq(person::id)))
        .left_join(
          person_alias_1
            .on(registration_application::admin_id.eq(person_alias_1.field(person::id).nullable())),
        )
        .order_by(registration_application::published.desc())
        .select((
          registration_application::all_columns,
          LocalUser::safe_settings_columns_tuple(),
          Person::safe_columns_tuple(),
          person_alias_1
            .fields(Person::safe_columns_tuple())
            .nullable(),
        ))
        .first::<RegistrationApplicationViewTuple>(conn)?;

    Ok(RegistrationApplicationView {
      registration_application,
      creator_local_user,
      creator,
      admin,
    })
  }

  /// Returns the current unread registration_application count
  pub fn get_unread_count(
    conn: &mut PgConnection,
    verified_email_only: bool,
  ) -> Result<i64, Error> {
    let person_alias_1 = diesel::alias!(person as person1);

    let mut query = registration_application::table
      .inner_join(local_user::table.on(registration_application::local_user_id.eq(local_user::id)))
      .inner_join(person::table.on(local_user::person_id.eq(person::id)))
      .left_join(
        person_alias_1
          .on(registration_application::admin_id.eq(person_alias_1.field(person::id).nullable())),
      )
      .filter(registration_application::admin_id.is_null())
      .into_boxed();

    if verified_email_only {
      query = query.filter(local_user::email_verified.eq(true))
    }

    query
      .select(count(registration_application::id))
      .first::<i64>(conn)
  }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(default))]
pub struct RegistrationApplicationQuery<'a> {
  #[builder(!default)]
  conn: &'a mut PgConnection,
  unread_only: Option<bool>,
  verified_email_only: Option<bool>,
  page: Option<i64>,
  limit: Option<i64>,
}

impl<'a> RegistrationApplicationQuery<'a> {
  pub fn list(self) -> Result<Vec<RegistrationApplicationView>, Error> {
    let person_alias_1 = diesel::alias!(person as person1);

    let mut query = registration_application::table
      .inner_join(local_user::table.on(registration_application::local_user_id.eq(local_user::id)))
      .inner_join(person::table.on(local_user::person_id.eq(person::id)))
      .left_join(
        person_alias_1
          .on(registration_application::admin_id.eq(person_alias_1.field(person::id).nullable())),
      )
      .order_by(registration_application::published.desc())
      .select((
        registration_application::all_columns,
        LocalUser::safe_settings_columns_tuple(),
        Person::safe_columns_tuple(),
        person_alias_1
          .fields(Person::safe_columns_tuple())
          .nullable(),
      ))
      .into_boxed();

    if self.unread_only.unwrap_or(false) {
      query = query.filter(registration_application::admin_id.is_null())
    }

    if self.verified_email_only.unwrap_or(false) {
      query = query.filter(local_user::email_verified.eq(true))
    }

    let (limit, offset) = limit_and_offset(self.page, self.limit)?;

    query = query
      .limit(limit)
      .offset(offset)
      .order_by(registration_application::published.desc());

    let res = query.load::<RegistrationApplicationViewTuple>(self.conn)?;

    Ok(RegistrationApplicationView::from_tuple_to_vec(res))
  }
}

impl ViewToVec for RegistrationApplicationView {
  type DbTuple = RegistrationApplicationViewTuple;
  fn from_tuple_to_vec(items: Vec<Self::DbTuple>) -> Vec<Self> {
    items
      .into_iter()
      .map(|a| Self {
        registration_application: a.0,
        creator_local_user: a.1,
        creator: a.2,
        admin: a.3,
      })
      .collect::<Vec<Self>>()
  }
}

#[cfg(test)]
mod tests {
  use crate::registration_application_view::{
    RegistrationApplicationQuery,
    RegistrationApplicationView,
  };
  use lemmy_db_schema::{
    source::{
      instance::Instance,
      local_user::{LocalUser, LocalUserInsertForm, LocalUserSettings, LocalUserUpdateForm},
      person::*,
      registration_application::{
        RegistrationApplication,
        RegistrationApplicationInsertForm,
        RegistrationApplicationUpdateForm,
      },
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

    let timmy_person_form = PersonInsertForm::builder()
      .name("timmy_rav".into())
      .admin(Some(true))
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_timmy_person = Person::create(conn, &timmy_person_form).unwrap();

    let timmy_local_user_form = LocalUserInsertForm::builder()
      .person_id(inserted_timmy_person.id)
      .password_encrypted("nada".to_string())
      .build();

    let _inserted_timmy_local_user = LocalUser::create(conn, &timmy_local_user_form).unwrap();

    let sara_person_form = PersonInsertForm::builder()
      .name("sara_rav".into())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_sara_person = Person::create(conn, &sara_person_form).unwrap();

    let sara_local_user_form = LocalUserInsertForm::builder()
      .person_id(inserted_sara_person.id)
      .password_encrypted("nada".to_string())
      .build();

    let inserted_sara_local_user = LocalUser::create(conn, &sara_local_user_form).unwrap();

    // Sara creates an application
    let sara_app_form = RegistrationApplicationInsertForm {
      local_user_id: inserted_sara_local_user.id,
      answer: "LET ME IIIIINN".to_string(),
    };

    let sara_app = RegistrationApplication::create(conn, &sara_app_form).unwrap();

    let read_sara_app_view = RegistrationApplicationView::read(conn, sara_app.id).unwrap();

    let jess_person_form = PersonInsertForm::builder()
      .name("jess_rav".into())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_jess_person = Person::create(conn, &jess_person_form).unwrap();

    let jess_local_user_form = LocalUserInsertForm::builder()
      .person_id(inserted_jess_person.id)
      .password_encrypted("nada".to_string())
      .build();

    let inserted_jess_local_user = LocalUser::create(conn, &jess_local_user_form).unwrap();

    // Sara creates an application
    let jess_app_form = RegistrationApplicationInsertForm {
      local_user_id: inserted_jess_local_user.id,
      answer: "LET ME IIIIINN".to_string(),
    };

    let jess_app = RegistrationApplication::create(conn, &jess_app_form).unwrap();

    let read_jess_app_view = RegistrationApplicationView::read(conn, jess_app.id).unwrap();

    let mut expected_sara_app_view = RegistrationApplicationView {
      registration_application: sara_app.to_owned(),
      creator_local_user: LocalUserSettings {
        id: inserted_sara_local_user.id,
        person_id: inserted_sara_local_user.person_id,
        email: inserted_sara_local_user.email,
        show_nsfw: inserted_sara_local_user.show_nsfw,
        theme: inserted_sara_local_user.theme,
        default_sort_type: inserted_sara_local_user.default_sort_type,
        default_listing_type: inserted_sara_local_user.default_listing_type,
        interface_language: inserted_sara_local_user.interface_language,
        show_avatars: inserted_sara_local_user.show_avatars,
        send_notifications_to_email: inserted_sara_local_user.send_notifications_to_email,
        validator_time: inserted_sara_local_user.validator_time,
        show_bot_accounts: inserted_sara_local_user.show_bot_accounts,
        show_scores: inserted_sara_local_user.show_scores,
        show_read_posts: inserted_sara_local_user.show_read_posts,
        show_new_post_notifs: inserted_sara_local_user.show_new_post_notifs,
        email_verified: inserted_sara_local_user.email_verified,
        accepted_application: inserted_sara_local_user.accepted_application,
      },
      creator: PersonSafe {
        id: inserted_sara_person.id,
        name: inserted_sara_person.name.to_owned(),
        display_name: None,
        published: inserted_sara_person.published,
        avatar: None,
        actor_id: inserted_sara_person.actor_id.to_owned(),
        local: true,
        banned: false,
        ban_expires: None,
        deleted: false,
        admin: false,
        bot_account: false,
        bio: None,
        banner: None,
        updated: None,
        inbox_url: inserted_sara_person.inbox_url.to_owned(),
        shared_inbox_url: None,
        matrix_user_id: None,
        instance_id: inserted_instance.id,
      },
      admin: None,
    };

    assert_eq!(read_sara_app_view, expected_sara_app_view);

    // Do a batch read of the applications
    let apps = RegistrationApplicationQuery::builder()
      .conn(conn)
      .unread_only(Some(true))
      .build()
      .list()
      .unwrap();

    assert_eq!(
      apps,
      [
        read_jess_app_view.to_owned(),
        expected_sara_app_view.to_owned()
      ]
    );

    // Make sure the counts are correct
    let unread_count = RegistrationApplicationView::get_unread_count(conn, false).unwrap();
    assert_eq!(unread_count, 2);

    // Approve the application
    let approve_form = RegistrationApplicationUpdateForm {
      admin_id: Some(Some(inserted_timmy_person.id)),
      deny_reason: None,
    };

    RegistrationApplication::update(conn, sara_app.id, &approve_form).unwrap();

    // Update the local_user row
    let approve_local_user_form = LocalUserUpdateForm::builder()
      .accepted_application(Some(true))
      .build();

    LocalUser::update(conn, inserted_sara_local_user.id, &approve_local_user_form).unwrap();

    let read_sara_app_view_after_approve =
      RegistrationApplicationView::read(conn, sara_app.id).unwrap();

    // Make sure the columns changed
    expected_sara_app_view
      .creator_local_user
      .accepted_application = true;
    expected_sara_app_view.registration_application.admin_id = Some(inserted_timmy_person.id);

    expected_sara_app_view.admin = Some(PersonSafe {
      id: inserted_timmy_person.id,
      name: inserted_timmy_person.name.to_owned(),
      display_name: None,
      published: inserted_timmy_person.published,
      avatar: None,
      actor_id: inserted_timmy_person.actor_id.to_owned(),
      local: true,
      banned: false,
      ban_expires: None,
      deleted: false,
      admin: true,
      bot_account: false,
      bio: None,
      banner: None,
      updated: None,
      inbox_url: inserted_timmy_person.inbox_url.to_owned(),
      shared_inbox_url: None,
      matrix_user_id: None,
      instance_id: inserted_instance.id,
    });
    assert_eq!(read_sara_app_view_after_approve, expected_sara_app_view);

    // Do a batch read of apps again
    // It should show only jessicas which is unresolved
    let apps_after_resolve = RegistrationApplicationQuery::builder()
      .conn(conn)
      .unread_only(Some(true))
      .build()
      .list()
      .unwrap();
    assert_eq!(apps_after_resolve, vec![read_jess_app_view]);

    // Make sure the counts are correct
    let unread_count_after_approve =
      RegistrationApplicationView::get_unread_count(conn, false).unwrap();
    assert_eq!(unread_count_after_approve, 1);

    // Make sure the not undenied_only has all the apps
    let all_apps = RegistrationApplicationQuery::builder()
      .conn(conn)
      .build()
      .list()
      .unwrap();
    assert_eq!(all_apps.len(), 2);

    Person::delete(conn, inserted_timmy_person.id).unwrap();
    Person::delete(conn, inserted_sara_person.id).unwrap();
    Person::delete(conn, inserted_jess_person.id).unwrap();
    Instance::delete(conn, inserted_instance.id).unwrap();
  }
}
