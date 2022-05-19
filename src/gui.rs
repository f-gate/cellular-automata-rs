use eframe::egui;



// CURRNTLY BOILER PLATE CODE FROM EFRAME EXAMPLE:: IGNORE FOR NOW
    
    #[derive(Default)]
    pub struct CellAutomata {}

    impl CellAutomata {
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            //can do cool stuff here
            Self::default()
        }

        fn custom_painting(&mut self, ui: &mut egui::Ui) {
            let (rect, response) =
                ui.allocate_exact_size(egui::Vec2::splat(1000.0), egui::Sense::drag());
    
            let angle = response.drag_delta().x * 0.01;  
    
            let callback = egui::PaintCallback {
                rect,
                callback: std::sync::Arc::new(move |info, render_ctx| {
                    if let Some(painter) = render_ctx.downcast_ref::<egui_glow::Painter>() {

                        with_three_d_context(painter.gl(), |three_d| {
                            paint_with_three_d(three_d, info, angle);

                        });
                    } else {
                        eprintln!("Can't do custom painting because we are not using a glow context");
                    }
                }),
            };
            ui.painter().add(callback);
        }
    }

    impl eframe::App for CellAutomata {
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("3D Cellular Automata");
                
                egui::ScrollArea::both().show(ui, |ui| {
                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        self.custom_painting(ui);
                    });
                    ui.label("Drag to rotate!");
                });

            });

            
        }
    }


    
    /// We get a [`glow::Context`] from `eframe`, but we want a [`three_d::Context`].
///
/// Sadly we can't just create a [`three_d::Context`] in [`MyApp::new`] and pass it
/// to the [`egui::PaintCallback`] because [`three_d::Context`] isn't `Send+Sync`, which
/// [`egui::PaintCallback`] is.
fn with_three_d_context<R>(
    gl: &std::rc::Rc<glow::Context>,
    f: impl FnOnce(&three_d::Context) -> R,
) -> R {
    use std::cell::RefCell;
    thread_local! {
        pub static THREE_D: RefCell<Option<three_d::Context>> = RefCell::new(None);
    }

    THREE_D.with(|three_d| {
        let mut three_d = three_d.borrow_mut();
        let three_d =
            three_d.get_or_insert_with(|| three_d::Context::from_gl_context(gl.clone()).unwrap());
        f(three_d)
    })
}

fn paint_with_three_d(three_d: &three_d::Context, info: &egui::PaintCallbackInfo, angle: f32) {
    use three_d::*;

    // Set where to paint
    let viewport = info.viewport_in_pixels();
    let viewport = Viewport {
        x: viewport.left_px.round() as _,
        y: viewport.from_bottom_px.round() as _,
        width: viewport.width_px.round() as _,
        height: viewport.height_px.round() as _,
    };

    // Respect the egui clip region (e.g. if we are inside an `egui::ScrollArea`).
    let clip_rect = info.clip_rect_in_pixels();
    let render_states = RenderStates {
        clip: Clip::Enabled {
            x: clip_rect.left_px.round() as _,
            y: clip_rect.from_bottom_px.round() as _,
            width: clip_rect.width_px.round() as _,
            height: clip_rect.height_px.round() as _,
        },
        ..Default::default()
    };

    let camera = Camera::new_perspective(
        three_d,
        viewport,
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    )
    .unwrap();

    // Create a CPU-side mesh consisting of a single colored triangle
    let positions = vec![
        vec3(0.5, -0.5, 0.0),  // bottom right
        vec3(-0.5, -0.5, 0.0), // bottom left
        vec3(0.0, 0.5, 0.0),   // top
    ];

    let colors = vec![
        Color::new(255, 0, 0, 255), // bottom right
        Color::new(0, 255, 0, 255), // bottom left
        Color::new(0, 0, 255, 255), // top
    ];

    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    let material = ColorMaterial {
        render_states,
        ..Default::default()
    };
    let mut model = Model::new_with_material(three_d, &cpu_mesh, material).unwrap();

    // Set the current transformation of the triangle
    model.set_transformation(Mat4::from_angle_y(radians(angle)));

    // Render the triangle with the color material which uses the per vertex colors defined at construction
    model.render(&camera, &[]).unwrap();
}
