/// This module contains message types which are shared between web and the client.

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSignIn {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSignInResp {
    pub token: Option<String>,
    pub error: Option<String>,
}
