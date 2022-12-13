use crate::console::view::FileView;
use crate::debugger::EventHook;
use crate::debugger::Place;
use nix::libc::c_int;
use std::rc::Rc;

pub(super) struct TerminalHook {
    file_view: Rc<FileView>,
}

impl TerminalHook {
    pub(super) fn new(file_view: Rc<FileView>) -> Self {
        Self { file_view }
    }
}

impl EventHook for TerminalHook {
    fn on_trap(&self, pc: usize, mb_place: Option<Place>) -> anyhow::Result<()> {
        println!("Hit breakpoint at address {:#016X}", pc);
        if let Some(place) = mb_place {
            println!("{}:{}", place.file, place.line_number);
            println!("{}", self.file_view.render_source(&place, 1)?);
        }
        Ok(())
    }

    fn on_signal(&self, signo: c_int, code: c_int) {
        println!("Receive signal {signo}, reason: {code}")
    }
}
