use persistence::input_models::parents::NewParent;
use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
pub(super) struct ParentRegister {
    pub name: String,
    pub id: String,
    pub password: String,
    pub secret: String,
}

impl Into<NewParent> for ParentRegister {
    fn into(self) -> NewParent {
        let ParentRegister {
            name,
            id,
            password,
            secret,
        } = self;
        NewParent::builder()
            .name(name)
            .identity(id)
            .password(password)
            .secret(secret)
            .build()
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct ParentLogin {
    pub unique_id: String,
    pub pwd: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct ParentSecret {
    pub secret: String,
}
