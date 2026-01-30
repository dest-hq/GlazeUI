use std::sync::Arc;
use std::{marker::PhantomData, path::Path};

use image::imageops::FilterType;
use vello::peniko::{Blob, ImageBrush, ImageData, ImageFormat};

use crate::Widget;
use crate::id::next_id;

pub struct ImageWidget<App> {
    pub image: Option<ImageBrush>,
    pub width: u32,
    pub height: u32,
    _marker: PhantomData<App>,
}

impl<App> ImageWidget<App> {
    pub fn new() -> Self {
        Self {
            image: None,
            width: 0,
            height: 0,
            _marker: PhantomData,
        }
    }

    fn decode_image(
        &mut self,
        data: &[u8],
        width: Option<u32>,
        height: Option<u32>,
    ) -> std::io::Result<ImageData> {
        let mut image = image::ImageReader::new(std::io::Cursor::new(data))
            .with_guessed_format()?
            .decode()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let target_width = width.unwrap_or(image.width());
        let target_height = height.unwrap_or(image.height());

        if image.width() != target_width || image.height() != target_height {
            image = image.resize_exact(target_width, target_height, FilterType::Triangle);
        }

        self.width = target_width;
        self.height = target_height;

        let data = Arc::new(image.into_rgba8().into_vec());
        let blob = Blob::new(data);

        Ok(ImageData {
            data: blob,
            format: ImageFormat::Rgba8,
            width: target_width,
            height: target_height,
            alpha_type: vello::peniko::ImageAlphaType::Alpha,
        })
    }

    pub fn from_file(
        mut self,
        path: impl AsRef<Path>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Self, std::io::Error> {
        let path = path.as_ref();
        let data = std::fs::read(path)?;
        let image = Self::decode_image(&mut self, &data, width, height)?;
        self.image = Some(image.into());
        Ok(self)
    }

    pub fn from_bytes(
        mut self,
        bytes: &[u8],
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Self, std::io::Error> {
        let image = Self::decode_image(&mut self, bytes, width, height)?;
        self.image = Some(image.into());
        Ok(self)
    }

    pub fn build(self) -> Widget<App> {
        return Widget::new(
            next_id(),
            crate::WidgetElement::Image {
                image: self.image.unwrap(),
                width: self.width,
                height: self.height,
            },
            None,
        );
    }
}
