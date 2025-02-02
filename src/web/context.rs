use crate::web::auth::WebUser;
use crate::web::models::Device;
use crate::web::models::Key;

#[derive(Serialize, Debug)]
pub struct PossibleIntegration {
    pub id: Option<i32>,
    pub name: &'static str,
    pub display_name: &'static str,
    pub connected: bool,
}

#[derive(Serialize, Default, Debug)]
pub struct Context {
    pub user: Option<WebUser>,
    pub signin_error: Option<String>,
    pub integrations: Vec<PossibleIntegration>,
    pub devices: Vec<Device>,
    pub keys: Vec<Key>,
    pub integration_message: Option<(String, String)>,
}

impl Context {
    pub fn set_signin_error(mut self, signin_error: Option<String>) -> Self {
        self.signin_error = signin_error;
        self
    }

    pub fn set_user(mut self, user: Option<WebUser>) -> Self {
        self.user = user;
        self
    }

    pub fn set_integrations(mut self, integrations: Vec<PossibleIntegration>) -> Self {
        self.integrations = integrations;
        self
    }

    pub fn set_devices(mut self, devices: Vec<Device>) -> Self {
        self.devices = devices;
        self
    }

    pub fn set_keys(mut self, keys: Vec<Key>) -> Self {
        self.keys = keys;
        self
    }

    pub fn set_integration_message(
        mut self,
        integration_message: Option<(String, String)>,
    ) -> Self {
        self.integration_message = integration_message;
        self
    }
}
