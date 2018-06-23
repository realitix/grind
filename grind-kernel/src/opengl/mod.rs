mod entrypoint;
pub mod gles2;
mod global;
mod types;

use opengl::types::*;

#[no_mangle]
pub extern "C" fn gk_glActiveTexture(texture: GLenum) {
    println!("GrindKernel: Fn not implemented: glActiveTexture");
}

#[no_mangle]
pub extern "C" fn gk_glAttachShader(program: GLuint, shader: GLuint) {
    entrypoint::attach_shader(program, shader)
}

#[no_mangle]
pub extern "C" fn gk_glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
    println!("GrindKernel: Fn not implemented: glBindAttribLocation");
}

#[no_mangle]
pub extern "C" fn gk_glBindBuffer(target: GLenum, buffer: GLuint) {
    entrypoint::bind_buffer(target, buffer)
}

#[no_mangle]
pub extern "C" fn gk_glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
    println!("GrindKernel: Fn not implemented: glBindFramebuffer");
}

#[no_mangle]
pub extern "C" fn gk_glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
    println!("GrindKernel: Fn not implemented: glBindRenderbuffer");
}

#[no_mangle]
pub extern "C" fn gk_glBindTexture(target: GLenum, texture: GLuint) {
    println!("GrindKernel: Fn not implemented: glBindTexture");
}

#[no_mangle]
pub extern "C" fn gk_glBindVertexArray(array: GLuint) {
    println!("GrindKernel: Fn not implemented: glBindVertexArray");
}

#[no_mangle]
pub extern "C" fn gk_glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    println!("GrindKernel: Fn not implemented: glBlendColor");
}

#[no_mangle]
pub extern "C" fn gk_glBlendEquation(mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glBlendEquation");
}

#[no_mangle]
pub extern "C" fn gk_glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) {
    println!("GrindKernel: Fn not implemented: glBlendEquationSeparate");
}

#[no_mangle]
pub extern "C" fn gk_glBlendFunc(sfactor: GLenum, dfactor: GLenum) {
    println!("GrindKernel: Fn not implemented: glBlendFunc");
}

#[no_mangle]
pub extern "C" fn gk_glBlendFuncSeparate(
    srcRGB: GLenum,
    dstRGB: GLenum,
    srcAlpha: GLenum,
    dstAlpha: GLenum,
) {
    println!("GrindKernel: Fn not implemented: glBlendFuncSeparate");
}

#[no_mangle]
pub extern "C" fn gk_glBufferData(
    target: GLenum,
    size: GLsizeiptr,
    data: *const GLvoid,
    usage: GLenum,
) {
    entrypoint::buffer_data(target, size, data, usage)
}

#[no_mangle]
pub extern "C" fn gk_glBufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const GLvoid,
) {
    println!("GrindKernel: Fn not implemented: glBufferSubData");
}

#[no_mangle]
pub extern "C" fn gk_glCheckFramebufferStatus(target: GLenum) -> GLenum {
    println!("GrindKernel: Fn not implemented: glCheckFramebufferStatus");
    0
}

#[no_mangle]
pub extern "C" fn gk_glClear(mask: GLbitfield) {
    entrypoint::clear(mask)
}

#[no_mangle]
pub extern "C" fn gk_glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    entrypoint::clear_color(red, green, blue, alpha)
}

#[no_mangle]
pub extern "C" fn gk_glClearDepthf(depth: GLclampf) {
    println!("GrindKernel: Fn not implemented: glClearDepthf");
}

#[no_mangle]
pub extern "C" fn gk_glClearStencil(s: GLint) {
    println!("GrindKernel: Fn not implemented: glClearStencil");
}

#[no_mangle]
pub extern "C" fn gk_glColorMask(
    red: GLboolean,
    green: GLboolean,
    blue: GLboolean,
    alpha: GLboolean,
) {
    println!("GrindKernel: Fn not implemented: glColorMask");
}

#[no_mangle]
pub extern "C" fn gk_glCompileShader(shader: GLuint) {
    entrypoint::compile_shader(shader)
}

#[no_mangle]
pub extern "C" fn gk_glCompressedTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    println!("GrindKernel: Fn not implemented: gk_glCompressedTexImage2D");
}

#[no_mangle]
pub extern "C" fn gk_glCompressedTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    println!("GrindKernel: Fn not implemented: gk_glCompressedTexSubImage2D");
}

#[no_mangle]
pub extern "C" fn gk_glCopyTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
) {
    println!("GrindKernel: Fn not implemented: glCopyTexImage2D");
}

#[no_mangle]
pub extern "C" fn gk_glCopyTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    println!("GrindKernel: Fn not implemented: glCopyTexSubImage2D");
}

#[no_mangle]
pub extern "C" fn gk_glCreateProgram() -> GLuint {
    entrypoint::create_program()
}

#[no_mangle]
pub extern "C" fn gk_glCreateShader(_type: GLenum) -> GLuint {
    entrypoint::create_shader(_type)
}

#[no_mangle]
pub extern "C" fn gk_glCullFace(mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glCullFace");
}

#[no_mangle]
pub extern "C" fn gk_glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteBuffers");
}

#[no_mangle]
pub extern "C" fn gk_glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteFramebuffers");
}

#[no_mangle]
pub extern "C" fn gk_glDeleteProgram(program: GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteProgram");
}

#[no_mangle]
pub extern "C" fn gk_glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteRenderbuffers");
}

#[no_mangle]
pub extern "C" fn gk_glDeleteShader(shader: GLuint) {
    entrypoint::delete_shader(shader)
}

#[no_mangle]
pub extern "C" fn gk_glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteTextures");
}

#[no_mangle]
pub extern "C" fn gk_glDepthFunc(func: GLenum) {
    println!("GrindKernel: Fn not implemented: glDepthFunc");
}

#[no_mangle]
pub extern "C" fn gk_glDepthMask(flag: GLboolean) {
    println!("GrindKernel: Fn not implemented: glDepthMask");
}

#[no_mangle]
pub extern "C" fn gk_glDepthRangef(zNear: GLclampf, zFar: GLclampf) {
    println!("GrindKernel: Fn not implemented: glDepthRangef");
}

#[no_mangle]
pub extern "C" fn gk_glDetachShader(program: GLuint, shader: GLuint) {
    println!("GrindKernel: Fn not implemented: glDetachShader");
}

#[no_mangle]
pub extern "C" fn gk_glDisable(cap: GLenum) {
    println!("GrindKernel: Fn not implemented: glDisable");
}

#[no_mangle]
pub extern "C" fn gk_glDisableVertexAttribArray(index: GLuint) {
    println!("GrindKernel: Fn not implemented: glDisableVertexAttribArray");
}

#[no_mangle]
pub extern "C" fn gk_glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    entrypoint::draw_arrays(mode, first, count)
}

#[no_mangle]
pub extern "C" fn gk_glDrawElements(
    mode: GLenum,
    count: GLsizei,
    _type: GLenum,
    indices: *const GLvoid,
) {
    println!("GrindKernel: Fn not implemented: glDrawElements");
}

#[no_mangle]
pub extern "C" fn gk_glDrawArraysInstanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    primcount: GLsizei,
) {
    println!("GrindKernel: Fn not implemented: glDrawArraysInstanced");
}

#[no_mangle]
pub extern "C" fn gk_glDrawElementsInstanced(
    mode: GLenum,
    count: GLsizei,
    _type: GLenum,
    indices: *const GLvoid,
    primcount: GLsizei,
) {
    println!("GrindKernel: Fn not implemented: glDrawElementsInstanced");
}

#[no_mangle]
pub extern "C" fn gk_glEnable(cap: GLenum) {
    println!("GrindKernel: Fn not implemented: glEnable");
}

#[no_mangle]
pub extern "C" fn gk_glEnableVertexAttribArray(index: GLuint) {
    entrypoint::enable_vertex_attrib_array(index)
}

#[no_mangle]
pub extern "C" fn gk_glFinish() {
    println!("GrindKernel: Fn not implemented: glFinish");
}

#[no_mangle]
pub extern "C" fn gk_glFlush() {
    println!("GrindKernel: Fn not implemented: glFlush");
}

#[no_mangle]
pub extern "C" fn gk_glFramebufferRenderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    println!("GrindKernel: Fn not implemented: glFramebufferRenderbuffer");
}

#[no_mangle]
pub extern "C" fn gk_glFramebufferTexture2D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    println!("GrindKernel: Fn not implemented: glFramebufferTexture2D");
}

#[no_mangle]
pub extern "C" fn gk_glFrontFace(mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glFrontFace");
}

#[no_mangle]
pub extern "C" fn gk_glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
    entrypoint::gen_buffers(n, buffers)
}

#[no_mangle]
pub extern "C" fn gk_glGenerateMipmap(target: GLenum) {
    println!("GrindKernel: Fn not implemented: glGenerateMipmap");
}

#[no_mangle]
pub extern "C" fn gk_glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    println!("GrindKernel: Fn not implemented: glGenFramebuffers");
}

#[no_mangle]
pub extern "C" fn gk_glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    println!("GrindKernel: Fn not implemented: glGenRenderbuffers");
}

#[no_mangle]
pub extern "C" fn gk_glGenTextures(n: GLsizei, textures: *mut GLuint) {
    println!("GrindKernel: Fn not implemented: glGenTextures");
}

#[no_mangle]
pub extern "C" fn gk_glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    println!("GrindKernel: Fn not implemented: glGenVertexArrays");
}

#[no_mangle]
pub extern "C" fn gk_glGetActiveAttrib(
    program: GLuint,
    index: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    _type: *mut GLenum,
    name: *mut GLchar,
) {
    println!("GrindKernel: Fn not implemented: glGetActiveAttrib");
}

#[no_mangle]
pub extern "C" fn gk_glGetActiveUniform(
    program: GLuint,
    index: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    _type: *mut GLenum,
    name: *mut GLchar,
) {
    println!("GrindKernel: Fn not implemented: glGetActiveUniform");
}

#[no_mangle]
pub extern "C" fn gk_glGetAttachedShaders(
    program: GLuint,
    maxcount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
) {
    println!("GrindKernel: Fn not implemented: glGetAttachedShaders");
}

#[no_mangle]
pub extern "C" fn gk_glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
    entrypoint::get_attrib_location(program, name)
}

#[no_mangle]
pub extern "C" fn gk_glGetBooleanv(pname: GLenum, params: *mut GLboolean) {
    println!("GrindKernel: Fn not implemented: glGetBooleanv");
}

#[no_mangle]
pub extern "C" fn gk_glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetBufferParameteriv");
}

#[no_mangle]
pub extern "C" fn gk_glGetError() -> GLenum {
    entrypoint::get_error()
}

#[no_mangle]
pub extern "C" fn gk_glGetFloatv(pname: GLenum, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glGetFloatv");
}

#[no_mangle]
pub extern "C" fn gk_glGetFramebufferAttachmentParameteriv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    println!("GrindKernel: Fn not implemented: glGetFramebufferAttachmentParameteriv");
}

#[no_mangle]
pub extern "C" fn gk_glGetIntegerv(pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetIntegerv");
}

#[no_mangle]
pub extern "C" fn gk_glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    entrypoint::get_programiv(program, pname, params);
}

#[no_mangle]
pub extern "C" fn gk_glGetProgramInfoLog(
    program: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    infolog: *mut GLchar,
) {
    println!("GrindKernel: Fn not implemented: glGetProgramInfoLog");
}

#[no_mangle]
pub extern "C" fn gk_glGetRenderbufferParameteriv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    println!("GrindKernel: Fn not implemented: glGetRenderbufferParameteriv");
}

#[no_mangle]
pub extern "C" fn gk_glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    entrypoint::get_shaderiv(shader, pname, params)
}

#[no_mangle]
pub extern "C" fn gk_glGetShaderInfoLog(
    shader: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    infolog: *mut GLchar,
) {
    println!("GrindKernel: Fn not implemented: glGetShaderInfoLog");
}

#[no_mangle]
pub extern "C" fn gk_glGetShaderPrecisionFormat(
    shadertype: GLenum,
    precisiontype: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
) {
    println!("GrindKernel: Fn not implemented: glGetShaderPrecisionFormat");
}

#[no_mangle]
pub extern "C" fn gk_glGetShaderSource(
    shader: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
) {
    println!("GrindKernel: Fn not implemented: glGetShaderSource");
}

#[no_mangle]
pub extern "C" fn gk_glGetString(name: GLenum) -> *const GLubyte {
    println!("GrindKernel: Fn not implemented: glGetString");
    0 as *const u8
}

#[no_mangle]
pub extern "C" fn gk_glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glGetTexParameterfv");
}

#[no_mangle]
pub extern "C" fn gk_glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetTexParameteriv");
}

#[no_mangle]
pub extern "C" fn gk_glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glGetUniformfv");
}

#[no_mangle]
pub extern "C" fn gk_glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetUniformiv");
}

#[no_mangle]
pub extern "C" fn gk_glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    println!("GrindKernel: Fn not implemented: glGetUniformLocation");
    0
}

#[no_mangle]
pub extern "C" fn gk_glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glGetVertexAttribfv");
}

#[no_mangle]
pub extern "C" fn gk_glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetVertexAttribiv");
}

#[no_mangle]
pub extern "C" fn gk_glGetVertexAttribPointerv(
    index: GLuint,
    pname: GLenum,
    pointer: *mut *mut GLvoid,
) {
    println!("GrindKernel: Fn not implemented: glGetVertexAttribPointerv");
}

#[no_mangle]
pub extern "C" fn gk_glHint(target: GLenum, mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glHint");
}

#[no_mangle]
pub extern "C" fn gk_glIsBuffer(buffer: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsBuffer");
    0
}

#[no_mangle]
pub extern "C" fn gk_glIsEnabled(cap: GLenum) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsEnabled");
    0
}

#[no_mangle]
pub extern "C" fn gk_glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsFramebuffer");
    0
}

#[no_mangle]
pub extern "C" fn gk_glIsProgram(program: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsProgram");
    0
}

#[no_mangle]
pub extern "C" fn gk_glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsRenderbuffer");
    0
}

#[no_mangle]
pub extern "C" fn gk_glIsShader(shader: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsShader");
    0
}

#[no_mangle]
pub extern "C" fn gk_glIsTexture(texture: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsTexture");
    0
}

#[no_mangle]
pub extern "C" fn gk_glLineWidth(width: GLfloat) {
    println!("GrindKernel: Fn not implemented: glLineWidth");
}

#[no_mangle]
pub extern "C" fn gk_glLinkProgram(program: GLuint) {
    entrypoint::link_program(program)
}

#[no_mangle]
pub extern "C" fn gk_glPixelStorei(pname: GLenum, param: GLint) {
    println!("GrindKernel: Fn not implemented: glPixelStorei");
}

#[no_mangle]
pub extern "C" fn gk_glPolygonOffset(factor: GLfloat, units: GLfloat) {
    println!("GrindKernel: Fn not implemented: glPolygonOffset");
}

#[no_mangle]
pub extern "C" fn gk_glPolygonMode(face: GLenum, mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glPolygonMode");
}

#[no_mangle]
pub extern "C" fn gk_glReadPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    _type: GLenum,
    pixels: *mut GLvoid,
) {
    entrypoint::read_pixels(x, y, width, height, format, _type, pixels)
}

#[no_mangle]
pub extern "C" fn gk_glReleaseShaderCompiler() {
    println!("GrindKernel: Fn not implemented: glReleaseShaderCompiler");
}

#[no_mangle]
pub extern "C" fn gk_glRenderbufferStorage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    println!("GrindKernel: Fn not implemented: glRenderbufferStorage");
}

#[no_mangle]
pub extern "C" fn gk_glSampleCoverage(value: GLclampf, invert: GLboolean) {
    println!("GrindKernel: Fn not implemented: glSampleCoverage");
}

#[no_mangle]
pub extern "C" fn gk_glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    println!("GrindKernel: Fn not implemented: glScissor");
}

#[no_mangle]
pub extern "C" fn gk_glShaderBinary(
    n: GLsizei,
    shaders: *mut GLuint,
    binaryformat: GLenum,
    binary: *mut GLvoid,
    length: GLsizei,
) {
    println!("GrindKernel: Fn not implemented: glShaderBinary");
}

#[no_mangle]
pub extern "C" fn gk_glShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
) {
    entrypoint::shader_source(shader, count, string, length)
}

#[no_mangle]
pub extern "C" fn gk_glStencilFunc(func: GLenum, reference: GLint, mask: GLuint) {
    println!("GrindKernel: Fn not implemented: glStencilFunc");
}

#[no_mangle]
pub extern "C" fn gk_glStencilFuncSeparate(
    face: GLenum,
    func: GLenum,
    reference: GLint,
    mask: GLuint,
) {
    println!("GrindKernel: Fn not implemented: glStencilFuncSeparate");
}

#[no_mangle]
pub extern "C" fn gk_glStencilMask(mask: GLuint) {
    println!("GrindKernel: Fn not implemented: glStencilMask");
}

#[no_mangle]
pub extern "C" fn gk_glStencilMaskSeparate(face: GLenum, mask: GLuint) {
    println!("GrindKernel: Fn not implemented: glStencilMaskSeparate");
}

#[no_mangle]
pub extern "C" fn gk_glStencilOp(_fail: GLenum, zfail: GLenum, zpass: GLenum) {
    println!("GrindKernel: Fn not implemented: glStencilOp");
}

#[no_mangle]
pub extern "C" fn gk_glStencilOpSeparate(
    face: GLenum,
    _fail: GLenum,
    zfail: GLenum,
    zpass: GLenum,
) {
    println!("GrindKernel: Fn not implemented: glStencilOpSeparate");
}

#[no_mangle]
pub extern "C" fn gk_glTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    _type: GLenum,
    pixels: *const GLvoid,
) {
    println!("GrindKernel: Fn not implemented: gk_glTexImage2D");
}

#[no_mangle]
pub extern "C" fn gk_glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    println!("GrindKernel: Fn not implemented: glTexParameterf");
}

#[no_mangle]
pub extern "C" fn gk_glTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glTexParameterfv");
}

#[no_mangle]
pub extern "C" fn gk_glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    println!("GrindKernel: Fn not implemented: glTexParameteri");
}

#[no_mangle]
pub extern "C" fn gk_glTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glTexParameteriv");
}

#[no_mangle]
pub extern "C" fn gk_glTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    _type: GLenum,
    pixels: *const GLvoid,
) {
    println!("GrindKernel: Fn not implemented: gk_glTexSubImage2D");
}

#[no_mangle]
pub extern "C" fn gk_glUniform1f(location: GLint, x: GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform1f");
}

#[no_mangle]
pub extern "C" fn gk_glUniform1fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform1fv");
}

#[no_mangle]
pub extern "C" fn gk_glUniform1i(location: GLint, x: GLint) {
    println!("GrindKernel: Fn not implemented: glUniform1i");
}

#[no_mangle]
pub extern "C" fn gk_glUniform1iv(location: GLint, count: GLsizei, v: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glUniform1iv");
}

#[no_mangle]
pub extern "C" fn gk_glUniform2f(location: GLint, x: GLfloat, y: GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform2f");
}

#[no_mangle]
pub extern "C" fn gk_glUniform2fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform2fv");
}

#[no_mangle]
pub extern "C" fn gk_glUniform2i(location: GLint, x: GLint, y: GLint) {
    println!("GrindKernel: Fn not implemented: glUniform2i");
}

#[no_mangle]
pub extern "C" fn gk_glUniform2iv(location: GLint, count: GLsizei, v: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glUniform2iv");
}

#[no_mangle]
pub extern "C" fn gk_glUniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform3f");
}

#[no_mangle]
pub extern "C" fn gk_glUniform3fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform3fv");
}

#[no_mangle]
pub extern "C" fn gk_glUniform3i(location: GLint, x: GLint, y: GLint, z: GLint) {
    println!("GrindKernel: Fn not implemented: glUniform3i");
}

#[no_mangle]
pub extern "C" fn gk_glUniform3iv(location: GLint, count: GLsizei, v: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glUniform3iv");
}

#[no_mangle]
pub extern "C" fn gk_glUniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform4f");
}

#[no_mangle]
pub extern "C" fn gk_glUniform4fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform4fv");
}

#[no_mangle]
pub extern "C" fn gk_glUniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint) {
    println!("GrindKernel: Fn not implemented: glUniform4i");
}

#[no_mangle]
pub extern "C" fn gk_glUniform4iv(location: GLint, count: GLsizei, v: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glUniform4iv");
}

#[no_mangle]
pub extern "C" fn gk_glUniformMatrix2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *mut GLfloat,
) {
    println!("GrindKernel: Fn not implemented: glUniformMatrix2fv");
}

#[no_mangle]
pub extern "C" fn gk_glUniformMatrix3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *mut GLfloat,
) {
    println!("GrindKernel: Fn not implemented: glUniformMatrix3fv");
}

#[no_mangle]
pub extern "C" fn gk_glUniformMatrix4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *mut GLfloat,
) {
    println!("GrindKernel: Fn not implemented: glUniformMatrix4fv");
}

#[no_mangle]
pub extern "C" fn gk_glUseProgram(program: GLuint) {
    entrypoint::use_program(program)
}

#[no_mangle]
pub extern "C" fn gk_glValidateProgram(program: GLuint) {
    println!("GrindKernel: Fn not implemented: glValidateProgram");
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttrib1f(indx: GLuint, x: GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib1f");
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttrib1fv(indx: GLuint, values: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib1fv");
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttrib2f(indx: GLuint, x: GLfloat, y: GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib2f");
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttrib2fv(indx: GLuint, values: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib2fv");
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttrib3f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib3f");
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttrib3fv(indx: GLuint, values: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib3fv");
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttrib4f(
    indx: GLuint,
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    w: GLfloat,
) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib4f");
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttrib4fv(indx: GLuint, values: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib4fv");
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttribPointer(
    index: GLuint,
    size: GLint,
    _type: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    ptr: *const GLvoid,
) {
    entrypoint::vertex_attrib_pointer(index, size, _type, normalized, stride, ptr)
}

#[no_mangle]
pub extern "C" fn gk_glVertexAttribDivisor(indx: GLuint, divisor: GLuint) {
    println!("GrindKernel: Fn not implemented: glVertexAttribDivisor");
}

#[no_mangle]
pub extern "C" fn gk_glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    println!("GrindKernel: Fn not implemented: glViewport");
}
