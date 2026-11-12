use anyhow::Result;
use gpui::{AssetSource, SharedString};
use std::{
    borrow::Cow,
};

pub struct Assets;

impl Assets {
    pub fn from_repository() -> Self {
        Self
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let data: &[u8] = match path {
            "icons/avatar.svg" => include_bytes!("../../../resources/icons/avatar.svg"),
            "icons/compose.svg" => include_bytes!("../../../resources/icons/compose.svg"),
            "icons/penta-band.svg" => include_bytes!("../../../resources/icons/penta-band.svg"),
            "icons/record.svg" => include_bytes!("../../../resources/icons/record.svg"),
            "icons/search.svg" => include_bytes!("../../../resources/icons/search.svg"),
            "icons/write.svg" => include_bytes!("../../../resources/icons/write.svg"),
            _ => return Ok(None),
        };

        Ok(Some(Cow::Borrowed(data)))
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![
            "avatar.svg".into(),
            "compose.svg".into(),
            "penta-band.svg".into(),
            "record.svg".into(),
            "search.svg".into(),
            "write.svg".into(),
        ])
    }
}
