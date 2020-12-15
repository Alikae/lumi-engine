use std::path::Path;
use luminance::backend::texture::Texture as TextureBackend;
use luminance::context::GraphicsContext;
use luminance::pixel::NormRGBA8UI;
use luminance::texture::{Dim2, GenMipmaps, Sampler, Texture};
use luminance::texture::{Wrap, MinFilter, MagFilter};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

pub fn create_window()-> GlfwSurface {
    let dim = WindowDim::Windowed { width: 1920, height: 1080 };
    GlfwSurface::new_gl33("RUST*eng*INE", WindowOpt::default().set_dim(dim)).unwrap()
}

pub fn read_image(path: &Path) -> Option<image::RgbaImage> {
      image::open(path).map(|img| img.flipv().to_rgba8()).ok()
}

pub fn load_from_disk<B>(surface: &mut B, img: image::RgbaImage) -> Texture<B::Backend, Dim2, NormRGBA8UI>
where
  B: GraphicsContext,
  B::Backend: TextureBackend<Dim2, NormRGBA8UI>,
{
      let (width, height) = img.dimensions();
      let texels = img.into_raw();

    // create the luminance texture; the third argument is the number of mipmaps we want (leave it
    // to 0 for now) and the latest is the sampler to use when sampling the texels in the
    // shader (we’ll just use the default one)
    let mut tex = Texture::new(surface, [width, height], 0,
        Sampler {
            wrap_r: Wrap::ClampToEdge,
            wrap_s: Wrap::ClampToEdge,
            wrap_t: Wrap::ClampToEdge,
            min_filter: MinFilter::Nearest,
            mag_filter: MagFilter::Nearest,
            depth_comparison: None,
        })
        .expect("luminance texture creation");

    // the first argument disables mipmap generation (we don’t care so far)
    tex.upload_raw(GenMipmaps::No, &texels).unwrap();

    tex
}
