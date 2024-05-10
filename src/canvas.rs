use crate::window;

type CanvasMargin = (i16, i16, i16, i16);
type CurrentWorkspace = u8;
type WindowGap = i16;
type CanvasSize = (i16, i16);
type Canvas = Vec<window::Window>;
pub struct CanvasList(
    [Option<Canvas>; 10],
    CurrentWorkspace,
    WindowGap,
    CanvasMargin,
    CanvasSize,
);

impl CanvasList {
    pub fn new(gap: WindowGap, margin: CanvasMargin, size: CanvasSize) -> Self {
        Self(
            [None, None, None, None, None, None, None, None, None, None],
            1,
            gap,
            margin,
            size,
        )
    }
    fn add_canvas(&mut self, index: usize) {
        if let Some(canvas) = self.0.get_mut(index) {
            *canvas = Some(Vec::new());
        }
    }
    fn remove_canvas(&mut self, index: usize) {
        if let Some(canvas) = self.0.get_mut(index) {
            *canvas = None;
        }
    }
    fn add_window(&mut self, window: window::Window) {
        let index = self.1 as usize;
        let canvas = if let Some(canvas) = self.0.get_mut(index) {
            canvas
        } else {
            return;
        };
        if let Some(canvas) = canvas {
            canvas.push(window);
        }
    }
    fn remove_window(&mut self, addr: usize) {
        let index = self.1 as usize;
        let canvas = if let Some(canvas) = self.0.get_mut(index) {
            canvas
        } else {
            return;
        };
        if let Some(canvas) = canvas {
            canvas.retain(|w| w.get().0 != addr);
        }
    }
    fn arrange_windows(&mut self) {
        let index = self.1 as usize;
        let canvas = if let Some(canvas) = self.0.get_mut(index) {
            canvas
        } else {
            return;
        };
        if let Some(canvas) = canvas {
            unimplemented!()
        };
    }
}
