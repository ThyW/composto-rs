use smithay::{
    input::{keyboard::KeyboardTarget, pointer::PointerTarget, touch::TouchTarget},
    utils::IsAlive,
    wayland::seat::WaylandFocus,
};

use crate::state::{BackendDataExt, Compostate};

#[derive(Debug, Clone, PartialEq)]
pub enum CompostorFocus {
    Window(u8),
    Layer(u8),
    Popup(u8),
}

impl<BackendData: BackendDataExt> PointerTarget<Compostate<BackendData>> for CompostorFocus {
    fn enter(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::MotionEvent,
    ) {
        todo!()
    }

    fn motion(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::MotionEvent,
    ) {
        todo!()
    }

    fn relative_motion(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::RelativeMotionEvent,
    ) {
        todo!()
    }

    fn button(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::ButtonEvent,
    ) {
        todo!()
    }

    fn axis(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        frame: smithay::input::pointer::AxisFrame,
    ) {
        todo!()
    }

    fn frame(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
    ) {
        todo!()
    }

    fn gesture_swipe_begin(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::GestureSwipeBeginEvent,
    ) {
        todo!()
    }

    fn gesture_swipe_update(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::GestureSwipeUpdateEvent,
    ) {
        todo!()
    }

    fn gesture_swipe_end(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::GestureSwipeEndEvent,
    ) {
        todo!()
    }

    fn gesture_pinch_begin(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::GesturePinchBeginEvent,
    ) {
        todo!()
    }

    fn gesture_pinch_update(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::GesturePinchUpdateEvent,
    ) {
        todo!()
    }

    fn gesture_pinch_end(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::GesturePinchEndEvent,
    ) {
        todo!()
    }

    fn gesture_hold_begin(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::GestureHoldBeginEvent,
    ) {
        todo!()
    }

    fn gesture_hold_end(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::pointer::GestureHoldEndEvent,
    ) {
        todo!()
    }

    fn leave(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        serial: smithay::utils::Serial,
        time: u32,
    ) {
        todo!()
    }
}

impl<BackendData: BackendDataExt> KeyboardTarget<Compostate<BackendData>> for CompostorFocus {
    fn enter(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        keys: Vec<smithay::input::keyboard::KeysymHandle<'_>>,
        serial: smithay::utils::Serial,
    ) {
        todo!()
    }

    fn leave(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        serial: smithay::utils::Serial,
    ) {
        todo!()
    }

    fn key(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        key: smithay::input::keyboard::KeysymHandle<'_>,
        state: smithay::backend::input::KeyState,
        serial: smithay::utils::Serial,
        time: u32,
    ) {
        todo!()
    }

    fn modifiers(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        modifiers: smithay::input::keyboard::ModifiersState,
        serial: smithay::utils::Serial,
    ) {
        todo!()
    }
}

impl<BackendData: BackendDataExt> TouchTarget<Compostate<BackendData>> for CompostorFocus {
    fn down(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::touch::DownEvent,
        seq: smithay::utils::Serial,
    ) {
        todo!()
    }

    fn up(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::touch::UpEvent,
        seq: smithay::utils::Serial,
    ) {
        todo!()
    }

    fn motion(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::touch::MotionEvent,
        seq: smithay::utils::Serial,
    ) {
        todo!()
    }

    fn frame(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        seq: smithay::utils::Serial,
    ) {
        todo!()
    }

    fn cancel(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        seq: smithay::utils::Serial,
    ) {
        todo!()
    }

    fn shape(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::touch::ShapeEvent,
        seq: smithay::utils::Serial,
    ) {
        todo!()
    }

    fn orientation(
        &self,
        seat: &smithay::input::Seat<Compostate<BackendData>>,
        data: &mut Compostate<BackendData>,
        event: &smithay::input::touch::OrientationEvent,
        seq: smithay::utils::Serial,
    ) {
        todo!()
    }
}

impl IsAlive for CompostorFocus {
    fn alive(&self) -> bool {
        todo!()
    }
}

impl WaylandFocus for CompostorFocus {
    fn wl_surface(
        &self,
    ) -> Option<smithay::reexports::wayland_server::protocol::wl_surface::WlSurface> {
        None
    }
}
