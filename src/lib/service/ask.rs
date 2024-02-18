use crate::domain::clip::field::{self, Password};
use crate::ShortCode;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub shotrcode: ShortCode,
    pub password: field::Password,
}

impl  GetClip  {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shotrcode: ShortCode::from(shortcode),
            password: field::Password::default()
        }
    }
}




impl From<ShortCode> for GetClip {
    fn from(shotrcode: ShortCode) -> Self {
            Self {
                shotrcode,
                password: field::Password::default()
            }
    }
}

impl From<&str> for GetClip {
    fn from(raw: &str) -> Self {
            Self::from_raw(raw)
}
}
