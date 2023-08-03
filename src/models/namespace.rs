use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateParams {
    pub name: String,
}
