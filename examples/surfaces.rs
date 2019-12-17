use blinds::traits::*;
use blinds::*;
use golem::{Attribute, AttributeType, ColorFormat, Context, GeometryMode, GolemError, Dimension::{D2, D4}, ElementBuffer, NumberType, Surface, Texture, Uniform, UniformType, UniformValue, VertexBuffer, ShaderProgram, ShaderDescription};

async fn app(window: Window, ctx: glow::Context, mut events: EventStream) -> Result<(), GolemError> {
    let ctx = &Context::from_glow(ctx)?;

    // Step 1: Draw a triangle to the surface
    let vertices = [
        // Position         Color
        -0.5, -0.5,         1.0, 0.0, 0.0, 1.0,
        0.5, -0.5,          0.0, 1.0, 0.0, 1.0,
        0.0, 0.5,           0.0, 0.0, 1.0, 1.0
    ];
    let indices = [0, 1, 2];

    let mut shader = ShaderProgram::new(ctx, ShaderDescription {
        vertex_input: &[
            Attribute::new("vert_position", AttributeType::Vector(D2)),
            Attribute::new("vert_color", AttributeType::Vector(D4)),
        ],
        fragment_input: &[
            Attribute::new("frag_color", AttributeType::Vector(D4)),
        ],
        uniforms: &[],
        vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_color = vert_color;
        }"#,
        fragment_shader:
        r#" void main() {
            gl_FragColor = frag_color;
        }"#
    })?;

    let mut vb = VertexBuffer::new(ctx)?;
    let mut eb = ElementBuffer::new(ctx)?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind(&vb);
    let mut surface = Surface::new(ctx)?;
    let mut backing_texture = Texture::new(ctx)?;
    backing_texture.set_image(None, 1024, 768, ColorFormat::RGBA);
    surface.set_texture(Some(backing_texture));

    ctx.clear();
    Surface::bind(ctx, Some(&surface));
    shader.draw(&eb, 0..indices.len(), GeometryMode::Triangles)?;
    Surface::bind(ctx, None);

    Texture::bind(ctx, surface.texture(), 0);

    // Step 2: Draw a few copies of this triangle to the screen
    // Also, for fun, let's rotate them dynamically
    let vertices = [
        // Position         UV
        -0.2, -0.2,         0.0, 0.0,
        0.2, -0.2,          1.0, 0.0,
        0.2, 0.2,           1.0, 1.0,
        -0.2, 0.2,          0.0, 1.0,
    ];
    let indices = [
        0, 1, 2,
        2, 3, 0,
    ];
    let mut shader = ShaderProgram::new(ctx, ShaderDescription {
        vertex_input: &[
            Attribute::new("vert_position", AttributeType::Vector(D2)),
            Attribute::new("vert_uv", AttributeType::Vector(D2)),
        ],
        fragment_input: &[
            Attribute::new("frag_uv", AttributeType::Vector(D2)),
        ],
        uniforms: &[
            Uniform::new("image", UniformType::Sampler2D),
            Uniform::new("rotate", UniformType::Matrix(D2)),
            Uniform::new("translate", UniformType::Vector(NumberType::Float, D2)),
        ],
        vertex_shader: r#" void main() {
            gl_Position = vec4(translate + (rotate * vert_position), 0, 1);
            frag_uv = vert_uv;
        }"#,
        fragment_shader:
        r#" void main() {
            gl_FragColor = texture(image, frag_uv);
        }"#
    })?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind(&vb);
    shader.set_uniform("image", UniformValue::Int(0))?;

    while let Some(_) = events.next().await {
        ctx.clear();
        let rotate = [1.0, 0.0, 0.0, 1.0];
        let translate = [0.0, 0.0];
        shader.set_uniform("rotate", UniformValue::Matrix2(rotate))?;
        shader.set_uniform("translate", UniformValue::Vector2(translate))?;
        shader.draw(&eb, 0..indices.len(), GeometryMode::Triangles)?;
        window.present();
    }

    Ok(())
}

fn main() {
    run_gl(Settings::default(), |window, gfx, events| async move {
        app(window, gfx, events).await.unwrap()
    });
}