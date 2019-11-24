// TODO: validate vec and matrix dimensions
// TODO: add out-of-memory to GolemError?
// TODO: allow writing to the data of a texture
// TODO: unbinding textures, also should the API be bindless
// TODO: expanding a buffer does not work, actually, so the buffer API needs a re-work

type GlTexture = <glow::Context as glow::HasContext>::Texture;
type GlSampler = <glow::Context as glow::HasContext>::Sampler;
type GlProgram = <glow::Context as glow::HasContext>::Program;
type GlShader = <glow::Context as glow::HasContext>::Shader;
type GlVertexArray = <glow::Context as glow::HasContext>::VertexArray;
type GlFramebuffer = <glow::Context as glow::HasContext>::Framebuffer;
type GlBuffer = <glow::Context as glow::HasContext>::Buffer;
type GlUniformLocation = <glow::Context as glow::HasContext>::UniformLocation;

pub mod buffer;
pub mod objects;
pub mod program;

mod context;
pub use self::context::Context;

#[derive(Debug)]
pub enum GolemError {
    /// The OpenGL Shader compilation failed, with the given error message
    ///
    /// This may be during vertex-time, fragment-time, or link-time
    ShaderCompilationError(String),
    /// Some general error bubbling up from the GL context
    ContextError(String),
    /// An attempt was made to bind to an illegal uniform TODO
    NoSuchUniform(&'static str),
    /// An attempt was made to draw with no shader program bound
    NoBoundProgram,
    /// An attempt was made to set a uniform with a program that isn't bound
    NotCurrentProgram,
}

impl From<String> for GolemError {
    fn from(other: String) -> Self {
        GolemError::ContextError(other)
    }
}
