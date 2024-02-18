//use crate::data::model::Clip;
use crate::data::{query, Transaction, DatabasePool};
use crate::service::ask;
use crate::{ShortCode, ServiceError, Clip};
use std::convert::TryInto;

pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool)
    -> Result<Clip, ServiceError> {
        let user_password = req.password.clone();
        let clip: Clip = query::get_clip(req, pool).await?.try_into()?;

        if  clip.password.has_password() {
            if clip.password == user_password {
                Ok(clip)
            } else {
                Err(ServiceError::PermissionError("xeta oldu permission".to_owned()))
            }
        } else {
            Ok(clip)
        }
    }
