use anyhow::Result;
use smithay::backend::renderer::utils::on_commit_buffer_handler;
use smithay::desktop::{Space, Window};
use smithay::input::{SeatHandler, SeatState};
use smithay::reexports::calloop::{generic::Generic, Interest, LoopHandle, Mode, PostAction};
use smithay::reexports::wayland_server::{backend::ClientData, Display, DisplayHandle, Resource};
use smithay::wayland::buffer::BufferHandler;
use smithay::wayland::compositor::{
    add_blocker, add_pre_commit_hook, with_states, BufferAssignment, SurfaceAttributes,
};
use smithay::wayland::dmabuf::get_dmabuf;
use smithay::wayland::keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitHandler;
use smithay::wayland::shell::xdg::decoration::XdgDecorationHandler;
use smithay::wayland::shell::xdg::XdgShellHandler;
use smithay::wayland::socket::ListeningSocketSource;
use smithay::wayland::xdg_activation::XdgActivationHandler;
use smithay::wayland::{
    compositor::{CompositorClientState, CompositorHandler, CompositorState},
    fractional_scale::FractionalScaleManagerState,
    keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitState,
    output::{OutputHandler, OutputManagerState},
    presentation::PresentationState,
    selection::data_device::DataDeviceHandler,
    selection::{
        data_device::{ClientDndGrabHandler, DataDeviceState, ServerDndGrabHandler},
        primary_selection::PrimarySelectionState,
        wlr_data_control::DataControlState,
        SelectionHandler,
    },
    shell::{
        wlr_layer::WlrLayerShellState,
        xdg::{decoration::XdgDecorationState, XdgShellState},
    },
    shm::{ShmHandler, ShmState},
    viewporter::ViewporterState,
    xdg_activation::XdgActivationState,
};
use smithay::{
    delegate_compositor, delegate_data_device, delegate_keyboard_shortcuts_inhibit,
    delegate_output, delegate_seat, delegate_shm, delegate_viewporter, delegate_xdg_activation,
    delegate_xdg_decoration, delegate_xdg_shell,
};

use crate::focus::*;
use crate::windowdata::CompostoWindow;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct LoopData<BackendData: BackendDataExt + 'static> {
    pub state: Compostate<BackendData>,
    pub dh: DisplayHandle,
}

pub trait BackendDataExt {
    fn seat_name(&self) -> Option<String>;
}

#[derive(Debug, Default)]
pub struct ClientState {
    compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: smithay::reexports::wayland_server::backend::ClientId) {
        println!("intialized client");
    }

    fn disconnected(
        &self,
        _client_id: smithay::reexports::wayland_server::backend::ClientId,
        _reason: smithay::reexports::wayland_server::backend::DisconnectReason,
    ) {
        println!("disconnected clinet");
    }

    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientData").finish_non_exhaustive()
    }
}

pub struct Compostate<BackendData: BackendDataExt + 'static> {
    // our state
    pub backend_data: BackendData,
    pub display_handle: DisplayHandle,
    pub loop_handle: LoopHandle<'static, LoopData<BackendData>>,
    pub space: Space<CompostoWindow>,

    // wayland state
    pub compositor_state: CompositorState,
    // pub data_device_state: DataDeviceState,
    // pub layer_shell_state: WlrLayerShellState,
    pub output_manager_state: OutputManagerState,
    // pub primary_selection_state: PrimarySelectionState,
    // pub data_control_state: DataControlState,
    // pub keyboard_shortcuts_inhibit_state: KeyboardShortcutsInhibitState,
    //
    pub seat_state: SeatState<Self>,
    pub shm_state: ShmState,
    pub viewporter_state: ViewporterState,
    pub xdg_activation_state: XdgActivationState,
    pub xdg_decoration_state: XdgDecorationState,
    pub xdg_shell_state: XdgShellState,
    // pub presentation_state: PresentationState,
    // pub fractional_scale_manager_state: FractionalScaleManagerState,
    // input: TODO
    //
    pub running: AtomicBool,
}

impl<BackendData: BackendDataExt + 'static> Compostate<BackendData> {
    pub fn new(
        display: Display<Compostate<BackendData>>,
        lh: LoopHandle<'static, LoopData<BackendData>>,
        data: BackendData,
    ) -> Result<Self> {
        let dh = display.handle();

        let source = ListeningSocketSource::new_auto().unwrap();
        let sock_name = source.socket_name().to_string_lossy().into_owned();
        lh.insert_source(source, |client, _, data| {
            if let Err(e) = data
                .dh
                .insert_client(client, Arc::new(ClientState::default()))
            {
                eprintln!("Unable to add wayland client: {e}");
            }
        })?;

        println!("listening on socket: {sock_name}");

        lh.insert_source(
            Generic::new(display, Interest::READ, Mode::Level),
            |_, display, data| {
                unsafe { display.get_mut().dispatch_clients(&mut data.state).unwrap() };
                Ok(PostAction::Continue)
            },
        )
        .expect("wayland source initialization failed");

        Ok(Self {
            backend_data: data,
            display_handle: dh.clone(),
            loop_handle: lh,
            space: Space::default(),
            compositor_state: CompositorState::new::<Self>(&dh),
            // data_device_state: DataDeviceState::new::<Self>(&dh),
            // layer_shell_state: WlrLayerShellState::new::<Self>(&dh),
            output_manager_state: OutputManagerState::new_with_xdg_output::<Self>(&dh),
            // primary_selection_state: todo!(),
            // data_control_state: todo!(),
            // keyboard_shortcuts_inhibit_state: KeyboardShortcutsInhibitState::new::<Self>(&dh),
            seat_state: SeatState::new(),
            shm_state: ShmState::new::<Self>(&dh, Vec::new()),
            viewporter_state: ViewporterState::new::<Self>(&dh),
            xdg_activation_state: XdgActivationState::new::<Self>(&dh),
            xdg_decoration_state: XdgDecorationState::new::<Self>(&dh),
            xdg_shell_state: XdgShellState::new::<Self>(&dh),
            // presentation_state: todo!(),
            // fractional_scale_manager_state: todo!(),
            running: AtomicBool::new(true),
        })
    }
}

impl<BackendData: BackendDataExt + 'static> OutputHandler for Compostate<BackendData> {}
delegate_output!(@<BackendData: BackendDataExt + 'static> Compostate<BackendData>);

impl<BackendData: BackendDataExt + 'static> CompositorHandler for Compostate<BackendData> {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(
        &self,
        client: &'a smithay::reexports::wayland_server::Client,
    ) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(
        &mut self,
        surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        on_commit_buffer_handler::<Self>(surface);
    }

    fn new_surface(
        &mut self,
        surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        add_pre_commit_hook::<Self, _>(surface, move |state, _dh, surface| {
            let maybe_dmabuf = with_states(surface, |surface_data| {
                surface_data
                    .cached_state
                    .pending::<SurfaceAttributes>()
                    .buffer
                    .as_ref()
                    .and_then(|assignment| match assignment {
                        BufferAssignment::NewBuffer(buffer) => get_dmabuf(buffer).ok(),
                        _ => None,
                    })
            });
            if let Some(dmabuf) = maybe_dmabuf {
                if let Ok((blocker, source)) = dmabuf.generate_blocker(Interest::READ) {
                    let client = surface.client().unwrap();
                    let res = state.loop_handle.insert_source(source, move |_, _, data| {
                        data.state
                            .client_compositor_state(&client)
                            .blocker_cleared(&mut data.state, &data.dh);
                        Ok(())
                    });
                    if res.is_ok() {
                        add_blocker(surface, blocker);
                    }
                }
            }
        });
    }
}
delegate_compositor!(@<BackendData: BackendDataExt + 'static> Compostate<BackendData>);

impl<BackendData: BackendDataExt + 'static> BufferHandler for Compostate<BackendData> {
    fn buffer_destroyed(
        &mut self,
        buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    ) {
        todo!()
    }
}

impl<BackendData: BackendDataExt + 'static> ShmHandler for Compostate<BackendData> {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

delegate_shm!(@<BackendData: BackendDataExt + 'static> Compostate<BackendData>);

delegate_viewporter!(@<BackendData: BackendDataExt + 'static> Compostate<BackendData>);

impl<BackendData: BackendDataExt + 'static> XdgActivationHandler for Compostate<BackendData> {
    fn activation_state(&mut self) -> &mut XdgActivationState {
        &mut self.xdg_activation_state
    }

    fn request_activation(
        &mut self,
        token: smithay::wayland::xdg_activation::XdgActivationToken,
        token_data: smithay::wayland::xdg_activation::XdgActivationTokenData,
        surface: smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        // move the window to the top of the space stack?
        unimplemented!()
    }
}

delegate_xdg_activation!(@<BackendData: BackendDataExt + 'static> Compostate<BackendData>);

impl<BackendData: BackendDataExt + 'static> XdgDecorationHandler for Compostate<BackendData> {
    fn new_decoration(&mut self, toplevel: smithay::wayland::shell::xdg::ToplevelSurface) {
        todo!()
    }

    fn request_mode(
        &mut self,
        toplevel: smithay::wayland::shell::xdg::ToplevelSurface,
        mode: smithay::reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode,
    ) {
        todo!()
    }

    fn unset_mode(&mut self, toplevel: smithay::wayland::shell::xdg::ToplevelSurface) {
        todo!()
    }
}

delegate_xdg_decoration!(@<BackendData: BackendDataExt + 'static> Compostate<BackendData>);

impl<BackendData: BackendDataExt + 'static> XdgShellHandler for Compostate<BackendData> {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: smithay::wayland::shell::xdg::ToplevelSurface) {
        let window = CompostoWindow(Window::new_wayland_window(surface));

        if let Some(toplevel) = window.0.toplevel() {
            toplevel.with_pending_state(|state| {
                state.size = Some((800.into(), 600.into()).into());
                state.bounds = Some((800.into(), 800.into()).into());
            });
            toplevel.send_pending_configure();
        }

        self.space.map_element(window.clone(), (10, 100), true);
    }

    fn new_popup(
        &mut self,
        surface: smithay::wayland::shell::xdg::PopupSurface,
        positioner: smithay::wayland::shell::xdg::PositionerState,
    ) {
        todo!()
    }

    fn grab(
        &mut self,
        surface: smithay::wayland::shell::xdg::PopupSurface,
        seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
        serial: smithay::utils::Serial,
    ) {
        todo!()
    }

    fn reposition_request(
        &mut self,
        surface: smithay::wayland::shell::xdg::PopupSurface,
        positioner: smithay::wayland::shell::xdg::PositionerState,
        token: u32,
    ) {
        todo!()
    }
}

delegate_xdg_shell!(@<BackendData: BackendDataExt + 'static> Compostate<BackendData>);

impl<BackendData: BackendDataExt + 'static> SeatHandler for Compostate<BackendData> {
    type KeyboardFocus = CompostorFocus;

    type PointerFocus = CompostorFocus;

    type TouchFocus = CompostorFocus;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

delegate_seat!(@<BackendData: BackendDataExt + 'static> Compostate<BackendData>);
