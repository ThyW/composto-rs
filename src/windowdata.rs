use smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement;
use smithay::backend::renderer::{element::AsRenderElements, gles::GlesRenderer};
use smithay::backend::renderer::{ImportAll, ImportMem, Renderer, Texture};
use smithay::desktop::{space::SpaceElement, Window};
use smithay::render_elements;
use smithay::utils::{IsAlive, Physical, Point, Scale};

#[derive(PartialEq, Clone)]
pub struct CompostoWindow(pub Window);

impl IsAlive for CompostoWindow {
    fn alive(&self) -> bool {
        self.0.alive()
    }
}

render_elements!(
    pub WindowRenderElement<R> where R: ImportAll + ImportMem;
    Window=WaylandSurfaceRenderElement<R>,
);

impl<R> AsRenderElements<R> for CompostoWindow
where
    R: Renderer + ImportAll + ImportMem,
    <R as Renderer>::TextureId: Texture + 'static,
{
    type RenderElement = WindowRenderElement<R>;

    fn render_elements<C: From<Self::RenderElement>>(
        &self,
        renderer: &mut R,
        location: Point<i32, Physical>,
        scale: Scale<f64>,
        alpha: f32,
    ) -> Vec<C> {
        // let window_bbox = SpaceElement::bbox(&self.0);
        AsRenderElements::render_elements(&self.0, renderer, location, scale, alpha)
            .into_iter()
            .map(C::from)
            .collect()
    }
}

impl SpaceElement for CompostoWindow {
    fn bbox(&self) -> smithay::utils::Rectangle<i32, smithay::utils::Logical> {
        self.0.bbox()
    }

    fn is_in_input_region(
        &self,
        point: &smithay::utils::Point<f64, smithay::utils::Logical>,
    ) -> bool {
        self.0.is_in_input_region(point)
    }

    fn set_activate(&self, activated: bool) {
        self.0.set_activate(activated)
    }

    fn output_enter(
        &self,
        output: &smithay::output::Output,
        overlap: smithay::utils::Rectangle<i32, smithay::utils::Logical>,
    ) {
        // self.0.output_enter(output, overlap)
    }

    fn output_leave(&self, output: &smithay::output::Output) {
        todo!()
    }
}
