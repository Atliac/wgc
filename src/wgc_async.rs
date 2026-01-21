use windows::Graphics::Capture::GraphicsCaptureItem;

use crate::*;
pub struct WgcAsync {
    item: GraphicsCaptureItem,
    settings: WgcSettings,
}

impl WgcAsync {
    pub async fn new_with_picker(settings: WgcSettings) -> Result<Self> {
        let item = crate::utils::picker::capture_picker().await?;
        Ok(Self { item, settings })
    }
}
