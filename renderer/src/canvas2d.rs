use skia_safe::{
    paint, AlphaType, Color, ColorType, Font, ImageInfo, Paint, PixelGeometry, Surface,
    SurfaceProps, SurfacePropsFlags, TextBlob, Typeface,
};
use std::ffi::c_void;

pub struct SkiaCanvas {
    surface: Surface,
    pub width: i32,
    pub height: i32,
}

impl SkiaCanvas {
    pub fn new(width: i32, height: i32) -> SkiaCanvas {
        let surface = Surface::new_raster(
            &ImageInfo::new(
                (width, height),
                ColorType::RGBA8888,
                AlphaType::Unpremul,
                None,
            ),
            Some(*&[0; 1][0] as usize),
            Some(&SurfaceProps::new(
                SurfacePropsFlags::USE_DEVICE_INDEPENDENT_FONTS,
                PixelGeometry::RGBV,
            )),
        )
        .expect("no surface!");

        SkiaCanvas {
            surface,
            width,
            height,
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;

        self.surface = Surface::new_raster(
            &ImageInfo::new(
                (width, height),
                ColorType::RGBA8888,
                AlphaType::Unpremul,
                None,
            ),
            Some(*&[0; 1][0] as usize),
            Some(&SurfaceProps::new(
                SurfacePropsFlags::USE_DEVICE_INDEPENDENT_FONTS,
                PixelGeometry::RGBV,
            )),
        )
        .expect("no surface!");
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas().clear(color);
    }

    pub fn text(&mut self, x: f32, y: f32, string: &str) {
        let ctx = self.canvas();

        let mut paint = Paint::default();
        paint
            .set_anti_alias(true)
            .set_color(Color::from_rgb(255, 255, 255))
            .set_style(paint::Style::Fill);

        let blob1 = TextBlob::from_str(
            string,
            &Font::from_typeface_with_params(Typeface::default(), 15.0, 1.5, 0.0),
        )
        .unwrap();

        ctx.draw_text_blob(&blob1, (x, y), &paint);
    }

    pub fn data(&mut self) -> *const c_void {
        match self.surface.image_snapshot().peek_pixels() {
            Some(x) => unsafe { x.addr() },
            None => std::ptr::null(),
        }
    }

    #[inline]
    fn canvas(&mut self) -> &mut skia_safe::Canvas {
        self.surface.canvas()
    }
}
