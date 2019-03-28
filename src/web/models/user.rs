use bcrypt;
use diesel::prelude::*;

use super::*;
use crate::web::schema::users;
use crate::web::routes::settings::SettingsForm;
use crate::config::{MountableDeviceLocation, StagingConfig};

use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Identifiable, Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub notify_email: Option<String>,
    pub notify_pushover: Option<String>,
    pub staging_type: StagingKind,
    pub staging_location: Option<String>,
}

#[derive(Debug, DbEnum, Serialize)]
// We can't reuse this directly, without pulling all of the web stuff into the client, so instead
// we're going to have a mirror struct and some smoke unit tests that break if they're not kept in
// sync
pub enum StagingKind {
    Device,
    Directory,
}

impl<'v> FromFormValue<'v> for StagingKind {
    type Error = String;

    fn from_form_value(form_value: &'v RawStr) -> Result<StagingKind, Self::Error> {
        let decoded = form_value.url_decode();
        match decoded {
            Ok(ref kind) if kind == "device" => Ok(StagingKind::Device),
            Ok(ref kind) if kind == "directory" => Ok(StagingKind::Directory),
            _ => Err(format!("unknown staging_kind {}", form_value)),
        }
    }
}

impl User {
    pub fn by_credentials(conn: &PgConnection, email: &str, password: &str) -> Option<User> {
        use crate::web::schema::users::dsl::{email as user_email, users};

        if let Ok(user) = users.filter(user_email.eq(email)).get_result::<User>(conn) {
            if bcrypt::verify(password, &user.password).unwrap() {
                Some(user)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn integrations(&self, conn: &PgConnection) -> QueryResult<Vec<Integration>> {
        use crate::web::schema::integrations::dsl::*;

        integrations
            .filter(user_id.eq(self.id))
            .load::<Integration>(conn)
    }

    pub fn devices(&self, conn: &PgConnection) -> QueryResult<Vec<Device>> {
        use crate::web::schema::devices::dsl::*;

        devices.filter(user_id.eq(self.id)).load::<Device>(conn)
    }

    pub fn keys(&self, conn: &PgConnection) -> QueryResult<Vec<Key>> {
        use crate::web::schema::keys::dsl::*;

        keys.filter(user_id.eq(self.id)).load::<Key>(conn)
    }

    pub fn integration_by_id(
        &self,
        integration_id: i32,
        conn: &PgConnection,
    ) -> QueryResult<Integration> {
        use crate::web::schema::integrations::dsl::*;

        integrations
            .filter(user_id.eq(self.id).and(id.eq(integration_id)))
            .get_result(conn)
    }

    pub fn device_by_id(&self, device_id: i32, conn: &PgConnection) -> QueryResult<Device> {
        use crate::web::schema::devices::dsl::*;

        devices
            .filter(user_id.eq(self.id).and(id.eq(device_id)))
            .get_result(conn)
    }

    pub fn key_by_id(&self, key_id: i32, conn: &PgConnection) -> QueryResult<Key> {
        use crate::web::schema::keys::dsl::*;

        keys.filter(user_id.eq(self.id).and(id.eq(key_id)))
            .get_result(conn)
    }

    pub fn staging(&self) -> Option<StagingConfig> {
        let loc = match &self.staging_location {
            Some(loc) => loc,
            None => return None,
        };
        let location = match &self.staging_type {
            StagingKind::Device => MountableDeviceLocation::Label(loc.to_owned()),
            StagingKind::Directory => MountableDeviceLocation::Mountpoint(loc.into()),
        };
        Some(StagingConfig {
            location,
        })
    }

    pub fn update_settings(&self, settings: &SettingsForm, conn: &PgConnection) -> QueryResult<usize> {
        use diesel::update;
        use crate::web::schema::users::dsl::*;

        let staging = settings.staging();
        update(self)
            .set((
                    notify_email.eq(settings.notification_email()),
                    notify_pushover.eq(settings.notification_pushover()),
                    staging_type.eq(staging.kind_for_db()),
                    staging_location.eq(staging.location_for_db())
            ))
            .execute(conn)
    }

    pub fn update_staging(&self, staging: &StagingConfig, conn: &PgConnection) -> QueryResult<usize> {
        use diesel::update;
        use crate::web::schema::users::dsl::*;

        update(self)
            .set((
                    staging_type.eq(staging.kind_for_db()),
                    staging_location.eq(staging.location_for_db())
            ))
            .execute(conn)
    }
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: String,
}

impl<'a> NewUser<'a> {
    pub fn new(email: &'a str, password: &'a str) -> Self {
        let hashed_password = bcrypt::hash(&password, bcrypt::DEFAULT_COST).unwrap();

        NewUser {
            email: email,
            password: hashed_password,
        }
    }

    pub fn create(&self, conn: &PgConnection) -> QueryResult<User> {
        use diesel::insert_into;

        insert_into(users::table)
            .values(self)
            .get_result::<User>(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MountableDeviceLocation;

    #[test]
    fn test_staging_kinds_are_in_sync() {
        // These don't have to run, we just want the definitions
        fn one_way(sk: StagingKind) {
            match sk {
                StagingKind::Device => {},
                StagingKind::Directory => {},
            }
        }

        fn other_way(ml: MountableDeviceLocation) {
            match ml {
                MountableDeviceLocation::Label(_) => {},
                MountableDeviceLocation::Mountpoint(_) => {},
            }
        }
        // If you find yourself looking at this test, it's because one of those enums was updated
        // without the other.
    }
}
