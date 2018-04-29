use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void, int8_t, uint8_t};
use libc::{intptr_t, ssize_t, int32_t};

pub const POINTS: c_uint = 0x0000 as c_uint;
pub const LINES: c_uint = 0x0001 as c_uint;
pub const LINE_LOOP: c_uint = 0x0002 as c_uint;
pub const LINE_STRIP: c_uint = 0x0003 as c_uint;
pub const TRIANGLES: c_uint = 0x0004 as c_uint;
pub const TRIANGLE_STRIP: c_uint = 0x0005 as c_uint;
pub const TRIANGLE_FAN: c_uint = 0x0006 as c_uint;

pub const DEPTH_BUFFER_BIT: c_uint = 0x00000100 as c_uint;
pub const STENCIL_BUFFER_BIT: c_uint = 0x00000400 as c_uint;
pub const COLOR_BUFFER_BIT: c_uint = 0x00004000 as c_uint;

/* BlendingFactorDest */
pub const ZERO: c_uint = 0 as c_uint;
pub const ONE: c_uint = 1 as c_uint;
pub const SRC_COLOR: c_uint = 0x0300 as c_uint;
pub const ONE_MINUS_SRC_COLOR: c_uint = 0x0301 as c_uint;
pub const SRC_ALPHA: c_uint = 0x0302 as c_uint;
pub const ONE_MINUS_SRC_ALPHA: c_uint = 0x0303 as c_uint;
pub const DST_ALPHA: c_uint = 0x0304 as c_uint;
pub const ONE_MINUS_DST_ALPHA: c_uint = 0x0305 as c_uint;

/* BlendingFactorSrc */
pub const DST_COLOR: c_uint = 0x0306 as c_uint;
pub const ONE_MINUS_DST_COLOR: c_uint = 0x0307 as c_uint;
pub const SRC_ALPHA_SATURATE: c_uint = 0x0308 as c_uint;

/* Boolean */
pub const TRUE: c_uchar = 1 as c_uchar;
pub const FALSE: c_uchar = 0 as c_uchar;

/* BlendEquationSeparate */
pub const FUNC_ADD: c_uint = 0x8006 as c_uint;
pub const BLEND_EQUATION: c_uint = 0x8009 as c_uint;
pub const BLEND_EQUATION_RGB: c_uint = 0x8009 as c_uint;
pub const BLEND_EQUATION_ALPHA: c_uint = 0x883D as c_uint;

/* BlendSubtract */
pub const FUNC_SUBTRACT: c_uint = 0x800A as c_uint;
pub const FUNC_REVERSE_SUBTRACT: c_uint = 0x800B as c_uint;

/* Separate Blend Functions */
pub const BLEND_DST_RGB: c_uint = 0x80C8 as c_uint;
pub const BLEND_SRC_RGB: c_uint = 0x80C9 as c_uint;
pub const BLEND_DST_ALPHA: c_uint = 0x80CA as c_uint;
pub const BLEND_SRC_ALPHA: c_uint = 0x80CB as c_uint;
pub const CONSTANT_COLOR: c_uint = 0x8001 as c_uint;
pub const ONE_MINUS_CONSTANT_COLOR: c_uint = 0x8002 as c_uint;
pub const CONSTANT_ALPHA: c_uint = 0x8003 as c_uint;
pub const ONE_MINUS_CONSTANT_ALPHA: c_uint = 0x8004 as c_uint;
pub const BLEND_COLOR: c_uint = 0x8005 as c_uint;

/* Errors. */
pub const NO_ERROR: c_uint = 0 as c_uint;
pub const INVALID_ENUM: c_uint = 0x0500 as c_uint;
pub const INVALID_VALUE: c_uint = 0x0501 as c_uint;
pub const INVALID_OPERATION: c_uint = 0x0502 as c_uint;
pub const STACK_OVERFLOW: c_uint = 0x0503 as c_uint;
pub const STACK_UNDERFLOW: c_uint = 0x0504 as c_uint;
pub const OUT_OF_MEMORY: c_uint = 0x0505 as c_uint;
pub const INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506 as c_uint;

/* DataType */
pub const BYTE: c_uint = 0x1400 as c_uint;
pub const UNSIGNED_BYTE: c_uint = 0x1401 as c_uint;
pub const SHORT: c_uint = 0x1402 as c_uint;
pub const UNSIGNED_SHORT: c_uint = 0x1403 as c_uint;
pub const INT: c_uint = 0x1404 as c_uint;
pub const UNSIGNED_INT: c_uint = 0x1405 as c_uint;
pub const FLOAT: c_uint = 0x1406 as c_uint;
pub const FIXED: c_uint = 0x140C as c_uint;

/* EnableCap */
pub const TEXTURE_2D: c_uint = 0x0DE1 as c_uint;
pub const CULL_FACE: c_uint = 0x0B44 as c_uint;
pub const BLEND: c_uint = 0x0BE2 as c_uint;
pub const DITHER: c_uint = 0x0BD0 as c_uint;
pub const STENCIL_TEST: c_uint = 0x0B90 as c_uint;
pub const DEPTH_TEST: c_uint = 0x0B71 as c_uint;
pub const SCISSOR_TEST: c_uint = 0x0C11 as c_uint;
pub const POLYGON_OFFSET_FILL: c_uint = 0x8037 as c_uint;
pub const SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E as c_uint;
pub const SAMPLE_COVERAGE: c_uint = 0x80A0 as c_uint;

/* Polygons */
pub const POINT: c_uint = 0x1B00 as c_uint;
pub const LINE: c_uint = 0x1B01 as c_uint;
pub const FILL: c_uint = 0x1B02 as c_uint;
pub const CW: c_uint = 0x0900 as c_uint;
pub const CCW: c_uint = 0x0901 as c_uint;
pub const POLYGON_MODE: c_uint = 0x0B40 as c_uint;
pub const POLYGON_SMOOTH: c_uint = 0x0B41 as c_uint;
pub const POLYGON_STIPPLE: c_uint = 0x0B42 as c_uint;
pub const EDGE_FLAG: c_uint = 0x0B43 as c_uint;

/* GetPName */
pub const LINE_WIDTH: c_uint = 0x0B21 as c_uint;
pub const ALIASED_POINT_SIZE_RANGE: c_uint = 0x846D as c_uint;
pub const ALIASED_LINE_WIDTH_RANGE: c_uint = 0x846E as c_uint;
pub const CULL_FACE_MODE: c_uint = 0x0B45 as c_uint;
pub const FRONT_FACE: c_uint = 0x0B46 as c_uint;
pub const DEPTH_RANGE: c_uint = 0x0B70 as c_uint;
pub const DEPTH_WRITEMASK: c_uint = 0x0B72 as c_uint;
pub const DEPTH_CLEAR_VALUE: c_uint = 0x0B73 as c_uint;
pub const DEPTH_FUNC: c_uint = 0x0B74 as c_uint;
pub const STENCIL_CLEAR_VALUE: c_uint = 0x0B91 as c_uint;
pub const STENCIL_FUNC: c_uint = 0x0B92 as c_uint;
pub const STENCIL_FAIL: c_uint = 0x0B94 as c_uint;
pub const STENCIL_PASS_DEPTH_FAIL: c_uint = 0x0B95 as c_uint;
pub const STENCIL_PASS_DEPTH_PASS: c_uint = 0x0B96 as c_uint;
pub const STENCIL_REF: c_uint = 0x0B97 as c_uint;
pub const STENCIL_VALUE_MASK: c_uint = 0x0B93 as c_uint;
pub const STENCIL_WRITEMASK: c_uint = 0x0B98 as c_uint;
pub const STENCIL_BACK_FUNC: c_uint = 0x8800 as c_uint;
pub const STENCIL_BACK_FAIL: c_uint = 0x8801 as c_uint;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: c_uint = 0x8802 as c_uint;
pub const STENCIL_BACK_PASS_DEPTH_PASS: c_uint = 0x8803 as c_uint;
pub const STENCIL_BACK_REF: c_uint = 0x8CA3 as c_uint;
pub const STENCIL_BACK_VALUE_MASK: c_uint = 0x8CA4 as c_uint;
pub const STENCIL_BACK_WRITEMASK: c_uint = 0x8CA5 as c_uint;
pub const VIEWPORT: c_uint = 0x0BA2 as c_uint;
pub const SCISSOR_BOX: c_uint = 0x0C10 as c_uint;
/*      SCISSOR_TEST */
pub const COLOR_CLEAR_VALUE: c_uint = 0x0C22 as c_uint;
pub const COLOR_WRITEMASK: c_uint = 0x0C23 as c_uint;
pub const UNPACK_ALIGNMENT: c_uint = 0x0CF5 as c_uint;
pub const PACK_ALIGNMENT: c_uint = 0x0D05 as c_uint;
pub const MAX_TEXTURE_SIZE: c_uint = 0x0D33 as c_uint;
pub const MAX_VIEWPORT_DIMS: c_uint = 0x0D3A as c_uint;
pub const SUBPIXEL_BITS: c_uint = 0x0D50 as c_uint;
pub const RED_BITS: c_uint = 0x0D52 as c_uint;
pub const GREEN_BITS: c_uint = 0x0D53 as c_uint;
pub const BLUE_BITS: c_uint = 0x0D54 as c_uint;
pub const ALPHA_BITS: c_uint = 0x0D55 as c_uint;
pub const DEPTH_BITS: c_uint = 0x0D56 as c_uint;
pub const STENCIL_BITS: c_uint = 0x0D57 as c_uint;
pub const POLYGON_OFFSET_UNITS: c_uint = 0x2A00 as c_uint;
/*      POLYGON_OFFSET_FILL */
pub const POLYGON_OFFSET_FACTOR: c_uint = 0x8038 as c_uint;
pub const TEXTURE_BINDING_2D: c_uint = 0x8069 as c_uint;
pub const SAMPLE_BUFFERS: c_uint = 0x80A8 as c_uint;
pub const SAMPLES: c_uint = 0x80A9 as c_uint;
pub const SAMPLE_COVERAGE_VALUE: c_uint = 0x80AA as c_uint;
pub const SAMPLE_COVERAGE_INVERT: c_uint = 0x80AB as c_uint;

/* GetTarget */
pub const UNPACK_ROW_LENGTH: c_uint = 0x0CF2 as c_uint;

/* PixelFormat */
pub const DEPTH_COMPONENT: c_uint = 0x1902 as c_uint;
pub const RED: c_uint = 0x1903 as c_uint;
pub const GREEN: c_uint = 0x1904 as c_uint;
pub const BLUE: c_uint = 0x1905 as c_uint;
pub const ALPHA: c_uint = 0x1906 as c_uint;
pub const RGB: c_uint = 0x1907 as c_uint;
pub const RGBA: c_uint = 0x1908 as c_uint;

pub const BGRA: c_uint = 0x80e1 as c_uint; // NB: Not OpenGL ES!
pub const RGBA8: c_uint = 0x8058 as c_uint; // NB: Not OpenGL ES!

/* Packed Pixels */
pub const UNSIGNED_INT_8_8_8_8_REV: c_uint = 0x8367 as c_uint; // NB: Not OpenGL ES!

/* Shaders */
pub const FRAGMENT_SHADER: c_uint = 0x8B30 as c_uint;
pub const VERTEX_SHADER: c_uint = 0x8B31 as c_uint;
pub const MAX_VERTEX_ATTRIBS: c_uint = 0x8869 as c_uint;
pub const MAX_VERTEX_UNIFORM_VECTORS: c_uint = 0x8DFB as c_uint;
pub const MAX_VARYING_VECTORS: c_uint = 0x8DFC as c_uint;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D as c_uint;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4C as c_uint;
pub const MAX_TEXTURE_IMAGE_UNITS: c_uint = 0x8872 as c_uint;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: c_uint = 0x8DFD as c_uint;
pub const SHADER_TYPE: c_uint = 0x8B4F as c_uint;
pub const DELETE_STATUS: c_uint = 0x8B80 as c_uint;
pub const LINK_STATUS: c_uint = 0x8B82 as c_uint;
pub const VALIDATE_STATUS: c_uint = 0x8B83 as c_uint;
pub const ATTACHED_SHADERS: c_uint = 0x8B85 as c_uint;
pub const ACTIVE_UNIFORMS: c_uint = 0x8B86 as c_uint;
pub const ACTIVE_UNIFORM_MAX_LENGTH: c_uint = 0x8B87 as c_uint;
pub const ACTIVE_ATTRIBUTES: c_uint = 0x8B89 as c_uint;
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: c_uint = 0x8B8A as c_uint;
pub const SHADING_LANGUAGE_VERSION: c_uint = 0x8B8C as c_uint;
pub const CURRENT_PROGRAM: c_uint = 0x8B8D as c_uint;

/* StencilFunction */
pub const NEVER: c_uint = 0x0200 as c_uint;
pub const LESS: c_uint = 0x0201 as c_uint;
pub const EQUAL: c_uint = 0x0202 as c_uint;
pub const LEQUAL: c_uint = 0x0203 as c_uint;
pub const GREATER: c_uint = 0x0204 as c_uint;
pub const NOTEQUAL: c_uint = 0x0205 as c_uint;
pub const GEQUAL: c_uint = 0x0206 as c_uint;
pub const ALWAYS: c_uint = 0x0207 as c_uint;

pub const VENDOR: c_uint = 0x1F00 as c_uint;
pub const RENDERER: c_uint = 0x1F01 as c_uint;
pub const VERSION: c_uint = 0x1F02 as c_uint;
pub const EXTENSIONS: c_uint = 0x1F03 as c_uint;

/* Shader Source */
pub const COMPILE_STATUS: c_uint = 0x8B81 as c_uint;
pub const INFO_LOG_LENGTH: c_uint = 0x8B84 as c_uint;
pub const SHADER_SOURCE_LENGTH: c_uint = 0x8B88 as c_uint;
pub const SHADER_COMPILER: c_uint = 0x8DFA as c_uint;

/* Buffer Objects */
pub const ARRAY_BUFFER: c_uint = 0x8892 as c_uint;
pub const ELEMENT_ARRAY_BUFFER: c_uint = 0x8893 as c_uint;
pub const ARRAY_BUFFER_BINDING: c_uint = 0x8894 as c_uint;
pub const ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895 as c_uint;

pub const STREAM_DRAW: c_uint = 0x88E0 as c_uint;
pub const STATIC_DRAW: c_uint = 0x88E4 as c_uint;
pub const DYNAMIC_DRAW: c_uint = 0x88E8 as c_uint;

/* CullFaceMode */
pub const FRONT: c_uint = 0x0404 as c_uint;
pub const BACK: c_uint = 0x0405 as c_uint;
pub const FRONT_AND_BACK: c_uint = 0x0408 as c_uint;

/* TextureMagFilter */
pub const NEAREST: c_uint = 0x2600 as c_uint;
pub const LINEAR: c_uint = 0x2601 as c_uint;

/* TextureParameterName */
pub const TEXTURE_MAG_FILTER: c_uint = 0x2800 as c_uint;
pub const TEXTURE_MIN_FILTER: c_uint = 0x2801 as c_uint;
pub const TEXTURE_WRAP_S: c_uint = 0x2802 as c_uint;
pub const TEXTURE_WRAP_T: c_uint = 0x2803 as c_uint;

/* TextureUnit */
pub const TEXTURE0: c_uint = 0x84C0 as c_uint;
pub const TEXTURE1: c_uint = 0x84C1 as c_uint;
pub const TEXTURE2: c_uint = 0x84C2 as c_uint;
pub const TEXTURE3: c_uint = 0x84C3 as c_uint;
pub const TEXTURE4: c_uint = 0x84C4 as c_uint;
pub const TEXTURE5: c_uint = 0x84C5 as c_uint;
pub const TEXTURE6: c_uint = 0x84C6 as c_uint;
pub const TEXTURE7: c_uint = 0x84C7 as c_uint;
pub const TEXTURE8: c_uint = 0x84C8 as c_uint;
pub const TEXTURE9: c_uint = 0x84C9 as c_uint;
pub const TEXTURE10: c_uint = 0x84CA as c_uint;
pub const TEXTURE11: c_uint = 0x84CB as c_uint;
pub const TEXTURE12: c_uint = 0x84CC as c_uint;
pub const TEXTURE13: c_uint = 0x84CD as c_uint;
pub const TEXTURE14: c_uint = 0x84CE as c_uint;
pub const TEXTURE15: c_uint = 0x84CF as c_uint;
pub const TEXTURE16: c_uint = 0x84D0 as c_uint;
pub const TEXTURE17: c_uint = 0x84D1 as c_uint;
pub const TEXTURE18: c_uint = 0x84D2 as c_uint;
pub const TEXTURE19: c_uint = 0x84D3 as c_uint;
pub const TEXTURE20: c_uint = 0x84D4 as c_uint;
pub const TEXTURE21: c_uint = 0x84D5 as c_uint;
pub const TEXTURE22: c_uint = 0x84D6 as c_uint;
pub const TEXTURE23: c_uint = 0x84D7 as c_uint;
pub const TEXTURE24: c_uint = 0x84D8 as c_uint;
pub const TEXTURE25: c_uint = 0x84D9 as c_uint;
pub const TEXTURE26: c_uint = 0x84DA as c_uint;
pub const TEXTURE27: c_uint = 0x84DB as c_uint;
pub const TEXTURE28: c_uint = 0x84DC as c_uint;
pub const TEXTURE29: c_uint = 0x84DD as c_uint;
pub const TEXTURE30: c_uint = 0x84DE as c_uint;
pub const TEXTURE31: c_uint = 0x84DF as c_uint;
pub const ACTIVE_TEXTURE: c_uint = 0x84E0 as c_uint;

/* TextureWrapMode */
pub const REPEAT: c_uint = 0x2901 as c_uint;
pub const CLAMP_TO_EDGE: c_uint = 0x812F as c_uint;
pub const MIRRORED_REPEAT: c_uint = 0x8370 as c_uint;

pub const COLOR_ATTACHMENT0: c_uint = 0x8CE0 as c_uint;

pub const FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5 as c_uint;

// Framebuffer Object
pub const FRAMEBUFFER: c_uint = 0x8D40 as c_uint;
pub const RENDERBUFFER: c_uint = 0x8D41 as c_uint;

// Extensions
pub const TEXTURE_RECTANGLE_ARB: c_uint = 0x84F5 as c_uint; // NB: Not OpenGL ES!

pub const UNPACK_CLIENT_STORAGE_APPLE: c_uint = 0x85B2 as c_uint; // NB: Not OpenGL ES!
pub const TEXTURE_STORAGE_HINT_APPLE: c_uint = 0x85BC as c_uint; // NB: Not OpenGL ES!
pub const STORAGE_CACHED_APPLE: c_uint = 0x85BE as c_uint; // NB: Not OpenGL ES!
pub const STORAGE_SHARED_APPLE: c_uint = 0x85BF as c_uint; // NB: Not OpenGL ES!

// Types
pub type GLvoid = c_void;
pub type GLchar = c_char;
pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLbyte = int8_t;
pub type GLshort = c_short;
pub type GLint = c_int;
pub type GLsizei = c_int;
pub type GLubyte = uint8_t;
pub type GLushort = c_ushort;
pub type GLuint = c_uint;
pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLfixed = int32_t;
pub type GLintptr = intptr_t;
pub type GLsizeiptr = ssize_t;
