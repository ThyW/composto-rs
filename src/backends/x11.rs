use std::time::Duration;

use smithay::delegate_dmabuf;
use smithay::desktop::space::{render_output, space_render_elements};
use smithay::output::{Mode, Output, PhysicalProperties, Subpixel};
use smithay::reexports::ash::vk::ExtPhysicalDeviceDrmFn;
use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;
use smithay::utils::{DeviceFd, Transform};
use smithay::wayland::dmabuf::{
    DmabufFeedback, DmabufFeedbackBuilder, DmabufGlobal, DmabufHandler, DmabufState,
};
use smithay::{
    backend::renderer::{
        element::{AsRenderElements, RenderElement},
        Frame, Renderer,
    },
    utils::Rectangle,
};
use smithay::{
    backend::{
        allocator::{
            dmabuf::DmabufAllocator,
            gbm::{GbmAllocator, GbmBufferFlags, GbmDevice},
            vulkan::{ImageUsageFlags, VulkanAllocator},
        },
        egl::{EGLContext, EGLDisplay},
        renderer::{damage::OutputDamageTracker, gles::GlesRenderer, Bind, ImportDma, ImportMemWl},
        vulkan::*,
        x11::*,
    },
    wayland::shm::with_buffer_contents_mut,
};

use crate::state::{BackendDataExt, Compostate, LoopData};
use crate::windowdata::WindowRenderElement;

pub struct BackendDataX11 {
    mode: Mode,
    renderer: GlesRenderer,
    damage_tracker: OutputDamageTracker,
    surface: X11Surface,
    dmabuf_state: DmabufState,
    _dmabuf_global: DmabufGlobal,
    _dmabuf_default_feedback: DmabufFeedback,
    render: bool,
}

impl BackendDataExt for BackendDataX11 {
    fn seat_name(&self) -> Option<String> {
        None
    }
}

/* impl BufferHandler for Compostate<BackendDataX11> {
    fn buffer_destroyed(
        &mut self,
        _buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    ) {
    }
} */

impl DmabufHandler for Compostate<BackendDataX11> {
    fn dmabuf_state(&mut self) -> &mut DmabufState {
        &mut self.backend_data.dmabuf_state
    }

    fn dmabuf_imported(
        &mut self,
        _global: &smithay::wayland::dmabuf::DmabufGlobal,
        dmabuf: smithay::backend::allocator::dmabuf::Dmabuf,
        notifier: smithay::wayland::dmabuf::ImportNotifier,
    ) {
        if self
            .backend_data
            .renderer
            .import_dmabuf(&dmabuf, None)
            .is_ok()
        {
            let _ = notifier.successful::<Compostate<BackendDataX11>>();
        } else {
            notifier.failed();
        }
    }
}

delegate_dmabuf!(Compostate<BackendDataX11>);

pub fn run_x11() -> anyhow::Result<()> {
    let mut event_loop = EventLoop::try_new()?;
    let backend = X11Backend::new()?;
    let display: Display<Compostate<BackendDataX11>> = Display::new()?;
    let mut display_handle = display.handle();

    let handle = backend.handle();
    let window = WindowBuilder::new()
        .title("composto: x11")
        .build(&handle)
        .expect("Unable to create the X window");

    let (node, drm_fd) = handle.drm_node()?;
    let gbm_device = GbmDevice::new(DeviceFd::from(drm_fd))?;

    let egl_display =
        unsafe { EGLDisplay::new(gbm_device.clone()).expect("egl display has been closed") };
    let egl_context = EGLContext::new(&egl_display).expect("unable to create egl context");
    let egl_modifiers = egl_context
        .dmabuf_render_formats()
        .iter()
        .map(|f| f.modifier)
        .collect::<std::collections::HashSet<_>>();

    let vulkan_allocator = Instance::new(version::Version::VERSION_1_2, None)
        .ok()
        .and_then(|instance| {
            PhysicalDevice::enumerate(&instance)
                .ok()
                .and_then(|devices| {
                    devices
                        .filter(|phd| phd.has_device_extension(ExtPhysicalDeviceDrmFn::name()))
                        .find(|phd| {
                            phd.primary_node().unwrap() == Some(node)
                                || phd.render_node().unwrap() == Some(node)
                        })
                })
        })
        .and_then(|physical_device| {
            VulkanAllocator::new(
                &physical_device,
                ImageUsageFlags::COLOR_ATTACHMENT | ImageUsageFlags::SAMPLED,
            )
            .ok()
        })
        .expect("cant create vulkan allocator");
    /* let dmabuf_allocator =
    DmabufAllocator(GbmAllocator::new(gbm_device, GbmBufferFlags::RENDERING)); */
    let dmabuf_allocator = DmabufAllocator(vulkan_allocator);

    let surface = handle.create_surface(&window, dmabuf_allocator, egl_modifiers.into_iter())?;

    let gles_renderer =
        unsafe { GlesRenderer::new(egl_context) }.expect("unable to construct a gles renderer");

    let dmabuf_formats = gles_renderer.dmabuf_formats().collect::<Vec<_>>();
    let dmabuf_feedback = DmabufFeedbackBuilder::new(node.dev_id(), dmabuf_formats).build()?;
    let mut dmabuf_state = DmabufState::new();

    let dmabuf_global = dmabuf_state
        .create_global_with_default_feedback::<Compostate<BackendDataX11>>(
            &display_handle,
            &dmabuf_feedback,
        );

    let size = {
        let s = window.size();
        (s.w as i32, s.h as i32).into()
    };

    let mode = Mode {
        size,
        refresh: 60_000,
    };
    let output = Output::new(
        "Composto".to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Smithay".into(),
            model: "X11".into(),
        },
    );

    let _global = output.create_global::<Compostate<BackendDataX11>>(&display.handle());
    output.change_current_state(Some(mode), None, None, Some((0, 0).into()));
    output.set_preferred(mode);

    let damage_tracker = OutputDamageTracker::from_output(&output);

    let backend_data = BackendDataX11 {
        mode,
        renderer: gles_renderer,
        dmabuf_state,
        damage_tracker,
        surface,
        _dmabuf_global: dmabuf_global,
        _dmabuf_default_feedback: dmabuf_feedback,
        render: true,
    };

    let mut state = Compostate::new(display, event_loop.handle(), backend_data)?;
    state
        .shm_state
        .update_formats(state.backend_data.renderer.shm_formats());

    state.space.map_output(&output, (0, 0));

    // map output
    let output_clone = output.clone();

    event_loop
        .handle()
        .insert_source(backend, move |event, _, data| match event {
            X11Event::CloseRequested { .. } => data
                .state
                .running
                .store(false, std::sync::atomic::Ordering::SeqCst),
            X11Event::Resized { new_size, .. } => {
                let output = &output_clone;
                let size = { (new_size.w as i32, new_size.h as i32).into() };

                data.state.backend_data.mode = Mode {
                    size,
                    refresh: 60_000,
                };

                output.delete_mode(output.current_mode().unwrap());
                output.change_current_state(Some(data.state.backend_data.mode), None, None, None);
                output.set_preferred(data.state.backend_data.mode);
                println!("x11: resized to new size: {new_size:#?}");
            }
            X11Event::Refresh { .. } | X11Event::PresentCompleted { .. } => {
                // println!("refresh event");
                state.backend_data.render = true;
            }
            X11Event::Input(event) => {
                // println!("X11 input event: {:#?}", event);
            }
            X11Event::Focus(false) => {
                println!("Lost focus");
            }
            _ => {}
        })
        .expect("cant handle backend sources");

    while state.running.load(std::sync::atomic::Ordering::SeqCst) {
        if state.backend_data.render {
            // println!("do rendering here");
            let backend_data = &mut state.backend_data;
            backend_data.surface.reset_buffers();
            let (buffer, dmabuf_age) = backend_data
                .surface
                .buffer()
                .expect("cant get surface buffer");

            if let Err(e) = backend_data.renderer.bind(buffer) {
                eprintln!("error binding buffer: {e}");
                continue;
            }

            /* let damage = Rectangle::from_loc_and_size((0, 0), size);
            let mut frame = backend_data.renderer.render(size, Transform::Normal)?;
            frame.clear([0.9, 0.9, 0.3, 1.0], &[damage])?;
            let _ = frame.finish().unwrap();

            println!("submitting surface");
            backend_data.surface.submit()?; */

            let output_clone = output.clone();

            let spaces = vec![&state.space];
            let custom_elems: Vec<WindowRenderElement<GlesRenderer>> = Vec::new();

            let render_res = render_output(
                &output_clone,
                &mut backend_data.renderer,
                1.0,
                dmabuf_age.into(),
                spaces,
                &custom_elems,
                &mut backend_data.damage_tracker,
                [1.0, 1.0, 1.0, 1.0],
            );

            match render_res {
                Ok(_) => {
                    if let Err(e) = backend_data.surface.submit() {
                        eprintln!("Error submiting surface buffers {e}");
                        backend_data.surface.reset_buffers();
                    } else {
                        // backend_data.render = false;
                        // std::thread::sleep(std::time::Duration::from_millis(16));
                    }
                }
                Err(e) => {
                    eprintln!("Rendering error {e}");
                    backend_data.surface.reset_buffers();
                }
            }
        }

        let mut calloop_data = LoopData {
            state,
            dh: display_handle.clone(),
        };
        let dispatch_result =
            event_loop.dispatch(Some(Duration::from_millis(16)), &mut calloop_data);

        LoopData {
            state,
            dh: display_handle,
        } = calloop_data;

        if dispatch_result.is_err() {
            state
                .running
                .store(false, std::sync::atomic::Ordering::SeqCst);
        } else {
            state.space.refresh();
            display_handle.flush_clients().unwrap();
        }
    }
    Ok(())
}
