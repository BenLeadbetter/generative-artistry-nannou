use nannou::prelude::*;

pub fn refresh_model_on_space<M, G: FnOnce(&App) -> M>(
    app: &App,
    model: &mut M,
    event: Event,
    gen_model: G,
) {
    if let Event::WindowEvent {
        id: _,
        simple: Some(window_event),
    } = event
    {
        if let WindowEvent::KeyReleased(Key::Space) = window_event {
            *model = gen_model(app);
        }
    }
}

pub fn rotate_about_point(pt: &mut Point2, origin: &Point2, angle: f32) {
    *pt -= *origin;
    *pt = pt.rotate(angle);
    *pt += *origin;
}
