use tiny_skia::*;

fn main() {
    let mut pixmap = Pixmap::new(1000, 1000).unwrap();

    let now = std::time::Instant::now();

    let paint1 = Paint {
        color: Color::from_rgba8(50, 127, 150, 200),
        anti_aliased: true,
        ..Paint::default()
    };

    let paint2 = Paint {
        color: Color::from_rgba8(220, 140, 75, 180),
        ..Paint::default()
    };

    let path1 = {
        let mut pb = PathBuilder::new();
        pb.move_to(60.0, 60.0);
        pb.line_to(160.0, 940.0);
        pb.cubic_to(380.0, 840.0, 660.0, 800.0, 940.0, 800.0);
        pb.cubic_to(740.0, 460.0, 440.0, 160.0, 60.0, 60.0);
        pb.close();
        pb.finish().unwrap()
    };

    let path2 = {
        let mut pb = PathBuilder::new();
        pb.move_to(940.0, 60.0);
        pb.line_to(840.0, 940.0);
        pb.cubic_to(620.0, 840.0, 340.0, 800.0, 60.0, 800.0);
        pb.cubic_to(260.0, 460.0, 560.0, 160.0, 940.0, 60.0);
        pb.close();
        pb.finish().unwrap()
    };

    pixmap.fill_path(&path1, &paint1);
    pixmap.fill_path(&path2, &paint2);

    println!("Rendered in {:.2}ms", now.elapsed().as_micros() as f64 / 1000.0);

    #[cfg(feature = "png-format")]
    {
        pixmap.save_png("image.png").unwrap();
    }
}
