use nannou::color::Rgba;

pub trait Render<T> {
    fn color(&self) -> Rgba<u8>;
}
