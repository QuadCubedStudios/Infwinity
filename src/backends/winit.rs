use smithay::{
    backend::{
        renderer::{
            damage::OutputDamageTracker, element::surface::WaylandSurfaceRenderElement,
            gles::GlesRenderer,
        },
        winit::{self, WinitEvent},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::winit::{dpi::LogicalSize, window::Window as WinitWindow},
    utils::{Rectangle, Transform},
};

use crate::{EventLoopHandle, Infwinity};

impl Infwinity {
    pub fn init_winit(&mut self, event_loop: &EventLoopHandle) -> anyhow::Result<()> {
        // Initialize the winit window
        let (mut backend, winit) = winit::init_from_attributes(
            WinitWindow::default_attributes()
                .with_inner_size(LogicalSize::new(1280.0, 800.0))
                .with_title("Infwinity")
                .with_visible(true)
                .with_transparent(true),
        )
        .map_err(|x| anyhow::format_err!("Winit init failed: {x:#?}"))?;

        let output = Output::new(
            "winit".to_string(),
            PhysicalProperties {
                size: (0, 0).into(),
                subpixel: Subpixel::Unknown,
                make: "Smithay".into(),
                model: "Winit".into(),
            },
        );
        output.change_current_state(
            Some(Mode {
                size: backend.window_size(),
                refresh: 0,
            }),
            Some(Transform::Flipped180),
            None,
            Some((0, 0).into()),
        );
        output.set_preferred(output.current_mode().expect("Mode went missing?"));

        self.register_output(&output);

        let mut damage_tracker = OutputDamageTracker::from_output(&output);

        event_loop
            .insert_source(winit, move |event, _meta, state| {
                match event {
                    WinitEvent::Resized { size, .. } => {
                        output.change_current_state(
                            Some(Mode { size, refresh: 0 }),
                            None,
                            None,
                            None,
                        );
                    }
                    // WinitEvent::Input(event) => state.process_input_event(event),
                    WinitEvent::Redraw => {
                        let size = backend.window_size();
                        let damage = Rectangle::from_loc_and_size((0, 0), size);

                        backend.bind().unwrap();
                        smithay::desktop::space::render_output::<
                            _,
                            WaylandSurfaceRenderElement<GlesRenderer>,
                            _,
                            _,
                        >(
                            &output,
                            backend.renderer(),
                            1.0,
                            0,
                            [&state.space],
                            &[],
                            &mut damage_tracker,
                            [0.1, 0.1, 0.1, 0.0],
                        )
                        .unwrap();
                        backend.submit(Some(&[damage])).unwrap();

                        state.after_frame_rendered(&output);

                        // Ask for redraw to schedule new frame.
                        backend.window().request_redraw();
                    }
                    WinitEvent::CloseRequested => state.shutdown(),
                    _ => (),
                };
            })
            .map_err(|x| anyhow::format_err!("Winit event loop register failed: {x}"))?;

        Ok(())
    }
}
