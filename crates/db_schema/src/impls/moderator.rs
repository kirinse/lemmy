use crate::{source::moderator::*, traits::Crud};
use diesel::{dsl::*, result::Error, *};

// TODO grep for ..xxxDefault()

impl Crud for ModRemovePost {
  type InsertForm = ModRemovePostForm;
  type UpdateForm = ModRemovePostForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_remove_post::dsl::*;
    mod_remove_post.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModRemovePostForm) -> Result<Self, Error> {
    use crate::schema::mod_remove_post::dsl::*;
    insert_into(mod_remove_post)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &mut PgConnection,
    from_id: i32,
    form: &ModRemovePostForm,
  ) -> Result<Self, Error> {
    use crate::schema::mod_remove_post::dsl::*;
    diesel::update(mod_remove_post.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModLockPost {
  type InsertForm = ModLockPostForm;
  type UpdateForm = ModLockPostForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_lock_post::dsl::*;
    mod_lock_post.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModLockPostForm) -> Result<Self, Error> {
    use crate::schema::mod_lock_post::dsl::*;
    insert_into(mod_lock_post)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &mut PgConnection, from_id: i32, form: &ModLockPostForm) -> Result<Self, Error> {
    use crate::schema::mod_lock_post::dsl::*;
    diesel::update(mod_lock_post.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModStickyPost {
  type InsertForm = ModStickyPostForm;
  type UpdateForm = ModStickyPostForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_sticky_post::dsl::*;
    mod_sticky_post.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModStickyPostForm) -> Result<Self, Error> {
    use crate::schema::mod_sticky_post::dsl::*;
    insert_into(mod_sticky_post)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &mut PgConnection,
    from_id: i32,
    form: &ModStickyPostForm,
  ) -> Result<Self, Error> {
    use crate::schema::mod_sticky_post::dsl::*;
    diesel::update(mod_sticky_post.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModRemoveComment {
  type InsertForm = ModRemoveCommentForm;
  type UpdateForm = ModRemoveCommentForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_remove_comment::dsl::*;
    mod_remove_comment.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModRemoveCommentForm) -> Result<Self, Error> {
    use crate::schema::mod_remove_comment::dsl::*;
    insert_into(mod_remove_comment)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &mut PgConnection,
    from_id: i32,
    form: &ModRemoveCommentForm,
  ) -> Result<Self, Error> {
    use crate::schema::mod_remove_comment::dsl::*;
    diesel::update(mod_remove_comment.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModRemoveCommunity {
  type InsertForm = ModRemoveCommunityForm;
  type UpdateForm = ModRemoveCommunityForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_remove_community::dsl::*;
    mod_remove_community.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModRemoveCommunityForm) -> Result<Self, Error> {
    use crate::schema::mod_remove_community::dsl::*;
    insert_into(mod_remove_community)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &mut PgConnection,
    from_id: i32,
    form: &ModRemoveCommunityForm,
  ) -> Result<Self, Error> {
    use crate::schema::mod_remove_community::dsl::*;
    diesel::update(mod_remove_community.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModBanFromCommunity {
  type InsertForm = ModBanFromCommunityForm;
  type UpdateForm = ModBanFromCommunityForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_ban_from_community::dsl::*;
    mod_ban_from_community.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModBanFromCommunityForm) -> Result<Self, Error> {
    use crate::schema::mod_ban_from_community::dsl::*;
    insert_into(mod_ban_from_community)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &mut PgConnection,
    from_id: i32,
    form: &ModBanFromCommunityForm,
  ) -> Result<Self, Error> {
    use crate::schema::mod_ban_from_community::dsl::*;
    diesel::update(mod_ban_from_community.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModBan {
  type InsertForm = ModBanForm;
  type UpdateForm = ModBanForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_ban::dsl::*;
    mod_ban.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModBanForm) -> Result<Self, Error> {
    use crate::schema::mod_ban::dsl::*;
    insert_into(mod_ban).values(form).get_result::<Self>(conn)
  }

  fn update(conn: &mut PgConnection, from_id: i32, form: &ModBanForm) -> Result<Self, Error> {
    use crate::schema::mod_ban::dsl::*;
    diesel::update(mod_ban.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModHideCommunity {
  type InsertForm = ModHideCommunityForm;
  type UpdateForm = ModHideCommunityForm;
  type IdType = i32;

  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_hide_community::dsl::*;
    mod_hide_community.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModHideCommunityForm) -> Result<Self, Error> {
    use crate::schema::mod_hide_community::dsl::*;
    insert_into(mod_hide_community)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &mut PgConnection,
    from_id: i32,
    form: &ModHideCommunityForm,
  ) -> Result<Self, Error> {
    use crate::schema::mod_hide_community::dsl::*;
    diesel::update(mod_hide_community.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModAddCommunity {
  type InsertForm = ModAddCommunityForm;
  type UpdateForm = ModAddCommunityForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_add_community::dsl::*;
    mod_add_community.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModAddCommunityForm) -> Result<Self, Error> {
    use crate::schema::mod_add_community::dsl::*;
    insert_into(mod_add_community)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &mut PgConnection,
    from_id: i32,
    form: &ModAddCommunityForm,
  ) -> Result<Self, Error> {
    use crate::schema::mod_add_community::dsl::*;
    diesel::update(mod_add_community.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModTransferCommunity {
  type InsertForm = ModTransferCommunityForm;
  type UpdateForm = ModTransferCommunityForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_transfer_community::dsl::*;
    mod_transfer_community.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModTransferCommunityForm) -> Result<Self, Error> {
    use crate::schema::mod_transfer_community::dsl::*;
    insert_into(mod_transfer_community)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(
    conn: &mut PgConnection,
    from_id: i32,
    form: &ModTransferCommunityForm,
  ) -> Result<Self, Error> {
    use crate::schema::mod_transfer_community::dsl::*;
    diesel::update(mod_transfer_community.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for ModAdd {
  type InsertForm = ModAddForm;
  type UpdateForm = ModAddForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::mod_add::dsl::*;
    mod_add.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &ModAddForm) -> Result<Self, Error> {
    use crate::schema::mod_add::dsl::*;
    insert_into(mod_add).values(form).get_result::<Self>(conn)
  }

  fn update(conn: &mut PgConnection, from_id: i32, form: &ModAddForm) -> Result<Self, Error> {
    use crate::schema::mod_add::dsl::*;
    diesel::update(mod_add.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for AdminPurgePerson {
  type InsertForm = AdminPurgePersonForm;
  type UpdateForm = AdminPurgePersonForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::admin_purge_person::dsl::*;
    admin_purge_person.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &Self::InsertForm) -> Result<Self, Error> {
    use crate::schema::admin_purge_person::dsl::*;
    insert_into(admin_purge_person)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &mut PgConnection, from_id: i32, form: &Self::InsertForm) -> Result<Self, Error> {
    use crate::schema::admin_purge_person::dsl::*;
    diesel::update(admin_purge_person.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for AdminPurgeCommunity {
  type InsertForm = AdminPurgeCommunityForm;
  type UpdateForm = AdminPurgeCommunityForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::admin_purge_community::dsl::*;
    admin_purge_community.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &Self::InsertForm) -> Result<Self, Error> {
    use crate::schema::admin_purge_community::dsl::*;
    insert_into(admin_purge_community)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &mut PgConnection, from_id: i32, form: &Self::InsertForm) -> Result<Self, Error> {
    use crate::schema::admin_purge_community::dsl::*;
    diesel::update(admin_purge_community.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for AdminPurgePost {
  type InsertForm = AdminPurgePostForm;
  type UpdateForm = AdminPurgePostForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::admin_purge_post::dsl::*;
    admin_purge_post.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &Self::InsertForm) -> Result<Self, Error> {
    use crate::schema::admin_purge_post::dsl::*;
    insert_into(admin_purge_post)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &mut PgConnection, from_id: i32, form: &Self::InsertForm) -> Result<Self, Error> {
    use crate::schema::admin_purge_post::dsl::*;
    diesel::update(admin_purge_post.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

impl Crud for AdminPurgeComment {
  type InsertForm = AdminPurgeCommentForm;
  type UpdateForm = AdminPurgeCommentForm;
  type IdType = i32;
  fn read(conn: &mut PgConnection, from_id: i32) -> Result<Self, Error> {
    use crate::schema::admin_purge_comment::dsl::*;
    admin_purge_comment.find(from_id).first::<Self>(conn)
  }

  fn create(conn: &mut PgConnection, form: &Self::InsertForm) -> Result<Self, Error> {
    use crate::schema::admin_purge_comment::dsl::*;
    insert_into(admin_purge_comment)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn update(conn: &mut PgConnection, from_id: i32, form: &Self::InsertForm) -> Result<Self, Error> {
    use crate::schema::admin_purge_comment::dsl::*;
    diesel::update(admin_purge_comment.find(from_id))
      .set(form)
      .get_result::<Self>(conn)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    source::{comment::*, community::*, instance::Instance, moderator::*, person::*, post::*},
    traits::Crud,
    utils::establish_unpooled_connection,
  };
  use serial_test::serial;

  // use Crud;
  #[test]
  #[serial]
  fn test_crud() {
    let conn = &mut establish_unpooled_connection();

    let inserted_instance = Instance::create(conn, "my_domain.tld").unwrap();

    let new_mod = PersonInsertForm::builder()
      .name("the mod".into())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_mod = Person::create(conn, &new_mod).unwrap();

    let new_person = PersonInsertForm::builder()
      .name("jim2".into())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_person = Person::create(conn, &new_person).unwrap();

    let new_community = CommunityInsertForm::builder()
      .name("mod_community".to_string())
      .title("nada".to_owned())
      .public_key("pubkey".to_string())
      .instance_id(inserted_instance.id)
      .build();

    let inserted_community = Community::create(conn, &new_community).unwrap();

    let new_post = PostInsertForm::builder()
      .name("A test post thweep".into())
      .creator_id(inserted_person.id)
      .community_id(inserted_community.id)
      .build();

    let inserted_post = Post::create(conn, &new_post).unwrap();

    let comment_form = CommentInsertForm::builder()
      .content("A test comment".into())
      .creator_id(inserted_person.id)
      .post_id(inserted_post.id)
      .build();

    let inserted_comment = Comment::create(conn, &comment_form, None).unwrap();

    // Now the actual tests

    // remove post
    let mod_remove_post_form = ModRemovePostForm {
      mod_person_id: inserted_mod.id,
      post_id: inserted_post.id,
      reason: None,
      removed: None,
    };
    let inserted_mod_remove_post = ModRemovePost::create(conn, &mod_remove_post_form).unwrap();
    let read_mod_remove_post = ModRemovePost::read(conn, inserted_mod_remove_post.id).unwrap();
    let expected_mod_remove_post = ModRemovePost {
      id: inserted_mod_remove_post.id,
      post_id: inserted_post.id,
      mod_person_id: inserted_mod.id,
      reason: None,
      removed: Some(true),
      when_: inserted_mod_remove_post.when_,
    };

    // lock post

    let mod_lock_post_form = ModLockPostForm {
      mod_person_id: inserted_mod.id,
      post_id: inserted_post.id,
      locked: None,
    };
    let inserted_mod_lock_post = ModLockPost::create(conn, &mod_lock_post_form).unwrap();
    let read_mod_lock_post = ModLockPost::read(conn, inserted_mod_lock_post.id).unwrap();
    let expected_mod_lock_post = ModLockPost {
      id: inserted_mod_lock_post.id,
      post_id: inserted_post.id,
      mod_person_id: inserted_mod.id,
      locked: Some(true),
      when_: inserted_mod_lock_post.when_,
    };

    // sticky post

    let mod_sticky_post_form = ModStickyPostForm {
      mod_person_id: inserted_mod.id,
      post_id: inserted_post.id,
      stickied: None,
    };
    let inserted_mod_sticky_post = ModStickyPost::create(conn, &mod_sticky_post_form).unwrap();
    let read_mod_sticky_post = ModStickyPost::read(conn, inserted_mod_sticky_post.id).unwrap();
    let expected_mod_sticky_post = ModStickyPost {
      id: inserted_mod_sticky_post.id,
      post_id: inserted_post.id,
      mod_person_id: inserted_mod.id,
      stickied: Some(true),
      when_: inserted_mod_sticky_post.when_,
    };

    // comment

    let mod_remove_comment_form = ModRemoveCommentForm {
      mod_person_id: inserted_mod.id,
      comment_id: inserted_comment.id,
      reason: None,
      removed: None,
    };
    let inserted_mod_remove_comment =
      ModRemoveComment::create(conn, &mod_remove_comment_form).unwrap();
    let read_mod_remove_comment =
      ModRemoveComment::read(conn, inserted_mod_remove_comment.id).unwrap();
    let expected_mod_remove_comment = ModRemoveComment {
      id: inserted_mod_remove_comment.id,
      comment_id: inserted_comment.id,
      mod_person_id: inserted_mod.id,
      reason: None,
      removed: Some(true),
      when_: inserted_mod_remove_comment.when_,
    };

    // community

    let mod_remove_community_form = ModRemoveCommunityForm {
      mod_person_id: inserted_mod.id,
      community_id: inserted_community.id,
      reason: None,
      removed: None,
      expires: None,
    };
    let inserted_mod_remove_community =
      ModRemoveCommunity::create(conn, &mod_remove_community_form).unwrap();
    let read_mod_remove_community =
      ModRemoveCommunity::read(conn, inserted_mod_remove_community.id).unwrap();
    let expected_mod_remove_community = ModRemoveCommunity {
      id: inserted_mod_remove_community.id,
      community_id: inserted_community.id,
      mod_person_id: inserted_mod.id,
      reason: None,
      removed: Some(true),
      expires: None,
      when_: inserted_mod_remove_community.when_,
    };

    // ban from community

    let mod_ban_from_community_form = ModBanFromCommunityForm {
      mod_person_id: inserted_mod.id,
      other_person_id: inserted_person.id,
      community_id: inserted_community.id,
      reason: None,
      banned: None,
      expires: None,
    };
    let inserted_mod_ban_from_community =
      ModBanFromCommunity::create(conn, &mod_ban_from_community_form).unwrap();
    let read_mod_ban_from_community =
      ModBanFromCommunity::read(conn, inserted_mod_ban_from_community.id).unwrap();
    let expected_mod_ban_from_community = ModBanFromCommunity {
      id: inserted_mod_ban_from_community.id,
      community_id: inserted_community.id,
      mod_person_id: inserted_mod.id,
      other_person_id: inserted_person.id,
      reason: None,
      banned: Some(true),
      expires: None,
      when_: inserted_mod_ban_from_community.when_,
    };

    // ban

    let mod_ban_form = ModBanForm {
      mod_person_id: inserted_mod.id,
      other_person_id: inserted_person.id,
      reason: None,
      banned: None,
      expires: None,
    };
    let inserted_mod_ban = ModBan::create(conn, &mod_ban_form).unwrap();
    let read_mod_ban = ModBan::read(conn, inserted_mod_ban.id).unwrap();
    let expected_mod_ban = ModBan {
      id: inserted_mod_ban.id,
      mod_person_id: inserted_mod.id,
      other_person_id: inserted_person.id,
      reason: None,
      banned: Some(true),
      expires: None,
      when_: inserted_mod_ban.when_,
    };

    // mod add community

    let mod_add_community_form = ModAddCommunityForm {
      mod_person_id: inserted_mod.id,
      other_person_id: inserted_person.id,
      community_id: inserted_community.id,
      removed: None,
    };
    let inserted_mod_add_community =
      ModAddCommunity::create(conn, &mod_add_community_form).unwrap();
    let read_mod_add_community =
      ModAddCommunity::read(conn, inserted_mod_add_community.id).unwrap();
    let expected_mod_add_community = ModAddCommunity {
      id: inserted_mod_add_community.id,
      community_id: inserted_community.id,
      mod_person_id: inserted_mod.id,
      other_person_id: inserted_person.id,
      removed: Some(false),
      when_: inserted_mod_add_community.when_,
    };

    // mod add

    let mod_add_form = ModAddForm {
      mod_person_id: inserted_mod.id,
      other_person_id: inserted_person.id,
      removed: None,
    };
    let inserted_mod_add = ModAdd::create(conn, &mod_add_form).unwrap();
    let read_mod_add = ModAdd::read(conn, inserted_mod_add.id).unwrap();
    let expected_mod_add = ModAdd {
      id: inserted_mod_add.id,
      mod_person_id: inserted_mod.id,
      other_person_id: inserted_person.id,
      removed: Some(false),
      when_: inserted_mod_add.when_,
    };

    Comment::delete(conn, inserted_comment.id).unwrap();
    Post::delete(conn, inserted_post.id).unwrap();
    Community::delete(conn, inserted_community.id).unwrap();
    Person::delete(conn, inserted_person.id).unwrap();
    Person::delete(conn, inserted_mod.id).unwrap();
    Instance::delete(conn, inserted_instance.id).unwrap();

    assert_eq!(expected_mod_remove_post, read_mod_remove_post);
    assert_eq!(expected_mod_lock_post, read_mod_lock_post);
    assert_eq!(expected_mod_sticky_post, read_mod_sticky_post);
    assert_eq!(expected_mod_remove_comment, read_mod_remove_comment);
    assert_eq!(expected_mod_remove_community, read_mod_remove_community);
    assert_eq!(expected_mod_ban_from_community, read_mod_ban_from_community);
    assert_eq!(expected_mod_ban, read_mod_ban);
    assert_eq!(expected_mod_add_community, read_mod_add_community);
    assert_eq!(expected_mod_add, read_mod_add);
  }
}
