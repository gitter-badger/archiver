use crate::web::db::DbConn;
use crate::web::auth::WebUser;
use crate::web::context::{Context, PossibleIntegration};

use rocket::request::FlashMessage;
use rocket_contrib::templates::Template;

use crate::messages::Oauth2Provider;

#[get("/")]
pub fn index(user: Option<WebUser>, conn: DbConn, flash: Option<FlashMessage<'_, '_>>) -> Template {
    let mut possible_integrations = vec![];
    let mut devices = vec![];
    let mut keys = vec![];

    if let Some(user) = &user {
        if let Ok(integrations) = user.user.integrations(&*conn) {
            let mut integrations = integrations.iter();

            for provider in Oauth2Provider::providers() {
                let name = provider.name();

                let configured_integration = integrations.find(|ref x| x.provider == name);

                possible_integrations.push(PossibleIntegration {
                    id: configured_integration.map(|i| i.id),
                    name: provider.name(),
                    display_name: provider.display_name(),
                    connected: configured_integration.is_some(),
                });
            }
        }
        devices = user.user.devices(&*conn).unwrap();
        keys = user.user.keys(&*conn).unwrap();
    }

    let context = Context::default()
        .set_user(user)
        .set_integrations(possible_integrations)
        .set_devices(devices)
        .set_keys(keys)
        .set_integration_message(flash.map(|ref msg| (msg.name().into(), msg.msg().into())));
    Template::render("index", context)
}
