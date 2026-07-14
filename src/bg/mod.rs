//! The site background: the GrainGradient GLSL, re-driven through WebGL2 via
//! `web-sys`. No JavaScript involved.
//!
//! The React site rendered this with `@paper-design/shaders-react`. That package
//! ships its GLSL unminified, so `webgl.rs` embeds the real shader rather than
//! approximating it. The npm dependency is gone, not wrapped.

mod webgl;
pub use webgl::GrainGradient;
