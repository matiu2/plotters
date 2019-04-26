/*!
# Plotters - Another Plotting Library in Rust

Plotters is a flexible drawing library for data visualization written in pure Rust.
The library isn't aimed supporting different types of plotting, but a generic platform
that can be extended to support different types of visualization methods.

## Quick Start

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut backend = BitMapBackend::new("examples/outputs/0.png", (640,480));
    backend.open()?;
    let root:DrawingArea<_,_> = backend.into();
    let font = Into::<FontDesc>::into("DejaVu Serif").resize(20.0);
    root.fill(&RGBColor(255,255,255))?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", &font)
        .build_ranged::<RangedCoordf32, RangedCoordf32, _, _>(-1f32..1f32, 0f32..1f32);

    chart.configure_mesh()
        .draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x*x)),
        &RGBColor(255, 0, 0),
    ))?;

    root.close()?;
    return Ok(());
}
```

![](https://raw.githubusercontent.com/38/plotters/master/examples/outputs/0.png)

## Concepts by examples

### Drawing Backends
Plotters can use different drawing backends, such as SVG, BitMap, etc. And even real-time rendering,
such as library. For example a bitmap drawing backend.

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a 800*600 bitmap and start drawing
    let mut backend = BitMapBackend::new("examples/outputs/1.png", (300,200));
    // And if we want SVG backend
    // let backend = SVGBackend::new("output.svg", (800, 600));
    backend.open()?;
    backend.draw_rect((50,50), (200, 150), &RGBColor(255,0,0), true)?;
    backend.close()?;
    return Ok(());
}
```

And this will produce

![](https://raw.githubusercontent.com/38/plotters/master/examples/outputs/1.png)

### Drawing Area
Plotters use a concept called drawing area for layout purpose.
Plotters support multiple plot integrate in a single image.
This is done by craeting sub drawing areas.

Besides that, drawing area also allows customized cooridnate system, by doing so, the coordinate mapping is done by the drawing area automatically.

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = BitMapBackend::new("examples/outputs/2.png", (300, 200));
    // A backend object can be converted into a drawing area
    let root_drawing_area:DrawingArea<_,_> = backend.into();
    // And we can split the drawing area into 3x3 grid
    let child_drawing_areas = root_drawing_area.split_evenly((3,3));
    // Then we fill the drawing area with different color
    for (area,color) in child_drawing_areas.into_iter().zip(0..) {
        area.fill(&Plattle99::pick(color))?;
    }
    root_drawing_area.close()?;
    return Ok(());
}
```

And this will produce

![](https://raw.githubusercontent.com/38/plotters/master/examples/outputs/2.png)

### Elements

In Plotters, elements are build blocks of a image. All elements are able to draw on a drawing area.
There are different types of elements, such as, lines, texts, circles, etc.

You may also combining existing elements to build a complex element.

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = BitMapBackend::new("examples/outputs/3.png", (300, 200));
    // A backend object can be converted into a drawing area
    let root:DrawingArea<_,_> = backend.into();
    root.fill(&RGBColor(255,255,255));
    // Draw an circle on the drawing area
    root.draw(&Circle::new((100,100), 50, Into::<ShapeStyle>::into(&RGBColor(0, 255, 0))))?;
    root.close()?;
    return Ok(());
}
```

And this will produce

![](https://raw.githubusercontent.com/38/plotters/master/examples/outputs/3.png)

### Composable Elements

Besides the basic elements, elements can be composed into a logic group we called composed elements.
When composing new elements, the upper-left conner is given in the target coordinate, and a new pixel
based coordinate which has the upper-left conner defined as `(0,0)` is used for further element composition purpose.

For example, we can have an element which includes a dot and its coordinate.

```rust
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = BitMapBackend::new("examples/outputs/4.png", (640, 480));
    // A backend object can be converted into a drawing area
    let root:DrawingArea<_,_> = backend.into();
    root.fill(&RGBColor(240,200,200))?;

    let root = root.apply_coord_spec(RangedCoord::<RangedCoordf32, RangedCoordf32>::new(0f32..1f32, 0f32..1f32, (0..640, 0..480)));
    let font = Into::<FontDesc>::into("DejaVu Serif").resize(15.0);

    let dot_and_label = |x:f32,y:f32| {
        return EmptyElement::at((x,y))
               + Circle::new((0,0), 3, Into::<ShapeStyle>::into(&RGBColor(0,0,0)).filled())
               + OwnedText::new(format!("({:.2},{:.2})", x, y), (10, 0), &font);
    };

    root.draw(&dot_and_label(0.5, 0.6))?;
    root.draw(&dot_and_label(0.25, 0.33))?;
    root.draw(&dot_and_label(0.8, 0.8))?;
    root.close()?;
    return Ok(());
}
```

![](https://raw.githubusercontent.com/38/plotters/master/examples/outputs/4.png)

### Chart Context

In order to draw a chart, Plotters need an data object build on top of drawing area called `ChartContext`.
The chart context defines even higher level constructs compare to the drawing area.
For example, you can define the label areas, meshs, and put a data series onto the drawing area with the help
of the chart context object.

```rust
use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = BitMapBackend::new("examples/outputs/5.png", (640, 480));
    let root:DrawingArea<_,_> = backend.into();
    root.fill(&RGBColor(255,255,255));
    let root = root.margin(10,10,10,10);
    // After this point, we should be able to draw construct a chart context
    let font:FontDesc = Into::<FontDesc>::into("DejaVu Serif").resize(40.0);
    // Create the chart object
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", &font)
        // Set the size of the label region
        .set_x_label_size(40)
        .set_y_label_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_ranged::<RangedCoordf32, RangedCoordf32, _, _>(0f32..10f32, 0f32..10f32);

    // Then we can draw a mesh
    chart.configure_mesh()
        // We can customize the maxium number of labels allowed for each axis
        .x_labels(30)
        .y_labels(10)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    let smaller_font = font.resize(10.0);
    chart.draw_series(LineSeries::new(vec![(0.0,0.0), (5.0, 5.0), (8.0, 7.0)], &RGBColor(255,0,0)))?;
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(vec![(0.0,0.0), (5.0, 5.0), (8.0, 7.0)], 5, &RGBColor(255,0,0), &|c,s,st| {
        return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + OwnedText::new(format!("{:?}", c), (10, 0), &smaller_font);
    }))?;
    root.close()?;
    return Ok(());
}
```

![](https://raw.githubusercontent.com/38/plotters/master/examples/outputs/5.png)

*/
pub mod chart;
pub mod coord;
pub mod data;
pub mod drawing;
pub mod element;
pub mod series;
pub mod style;

pub mod prelude {
    pub use crate::chart::{ChartBuilder, ChartContext};
    pub use crate::coord::{
        CoordTranslate, Ranged, RangedCoord, RangedCoordf32, RangedCoordf64, RangedCoordi32,
        RangedCoordi64, RangedCoordu32, RangedCoordu64, RangedDate, RangedDateTime,
    };
    pub use crate::drawing::{backend::DrawingBackend, DrawingArea};
    pub use crate::series::{Histogram, LineSeries, PointSeries};
    pub use crate::style::{
        Color, FontDesc, Mixable, Plattle, Plattle100, Plattle99, Plattle9999, RGBColor,
        ShapeStyle, TextStyle,
    };

    pub use crate::drawing::{BitMapBackend, SVGBackend};

    pub use crate::element::{
        CandleStick, Circle, Cross, EmptyElement, OwnedText, Path, Rectangle, Text,
    };
}
