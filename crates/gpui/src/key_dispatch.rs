use crate::{Action, KeyBinding, KeyContext, Keymap, Keystroke};
use std::{cell::RefCell, rc::Rc};
/// Key dispatch helper that evaluates bindings using a keymap and context stack.
pub(crate) struct KeyDispatcher {
    keymap: Rc<RefCell<Keymap>>,
impl KeyDispatcher {
    pub fn new(keymap: Rc<RefCell<Keymap>>) -> Self {
        Self { keymap }
        context_stack: &[KeyContext],
            .bindings_for_input(input, context_stack);
        (bindings, partial, context_stack.to_vec())
        &self,
        context_stack: &[KeyContext],
        let (bindings, pending, resolved_stack) = self.bindings_for_input(&input, context_stack);
                context_stack: resolved_stack,
                context_stack: resolved_stack,
                context_stack: resolved_stack,
        let (suffix, mut to_replay) = self.replay_prefix(input, context_stack);
        let mut result = self.dispatch_key(suffix, keystroke, context_stack);
        &self,
        context_stack: &[KeyContext],
        let (suffix, mut to_replay) = self.replay_prefix(input, context_stack);
            to_replay.extend(self.flush_dispatch(suffix, context_stack))
        context_stack: &[KeyContext],
            let (bindings, _, _) = self.bindings_for_input(&input[0..=last], context_stack);
}
#[derive(Default, Debug)]
pub(crate) struct Replay {
    pub(crate) keystroke: Keystroke,
    pub(crate) bindings: SmallVec<[KeyBinding; 1]>,
}
#[derive(Default, Debug)]
pub(crate) struct DispatchResult {
    pub(crate) pending: SmallVec<[Keystroke; 1]>,
    pub(crate) pending_has_binding: bool,
    pub(crate) bindings: SmallVec<[KeyBinding; 1]>,
    pub(crate) to_replay: SmallVec<[Replay; 1]>,
    pub(crate) context_stack: Vec<KeyContext>,
        InspectorElementId, KeyDispatcher, Keystroke, LayoutId, Style,
        Action, App, Bounds, Context, FocusHandle, InputHandler, IntoElement, KeyBinding,
        KeyContext, Keymap, Pixels, Point, Render, Subscription, TestAppContext, UTF16Selection,
        Window,
        let dispatcher = KeyDispatcher::new(keymap);
        let keybinding = dispatcher.bindings_for_action(&TestAction, &contexts);
        let dispatcher = KeyDispatcher::new(keymap);
            dispatcher: &KeyDispatcher,
            context_stack: &[KeyContext],
            dispatcher.dispatch_key(pending, Keystroke::parse(key).unwrap(), context_stack)
        let dispatch_path: Vec<KeyContext> = Vec::new();
        let result = dispatch(&dispatcher, SmallVec::new(), "ctrl-b", &dispatch_path);
        let result = dispatch(&dispatcher, result.pending, "h", &dispatch_path);
        let context_stack = vec![KeyContext::parse("ContextB").unwrap()];
        let result = dispatch(&dispatcher, SmallVec::new(), "space", &context_stack);
                _cx: &mut App,
                window.with_fiber_cx(|fiber| fiber.set_focus_handle(&self.focus_handle));
                window.with_fiber_cx(|fiber| {
                    fiber.set_key_context(key_context);
                    fiber.handle_input(&self.focus_handle, self.clone(), cx);
                    fiber.on_action(std::any::TypeId::of::<TestAction>(), |_, _, _, _| {});
                });
                _cx: &mut App,
                window.with_fiber_cx(|fiber| fiber.set_focus_handle(&self.focus_handle));
                window.with_fiber_cx(|fiber| {
                    fiber.set_key_context(key_context);
                    fiber.handle_input(&self.focus_handle, self.clone(), cx);
                    fiber.on_action(std::any::TypeId::of::<TestAction>(), |_, _, _, _| {});
                });




        cx.simulate_keystrokes("ctrl-b h");

        cx.update(|window, _| {
            assert!(window.has_pending_keystrokes());
        });