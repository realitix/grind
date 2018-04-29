mod types;

use opengl::types::*;

#[no_mangle]
pub extern "C" fn glActiveTexture(texture: GLenum) {
    println!("GrindKernel: Fn not implemented: glActiveTexture");
}

#[no_mangle]
pub extern "C" fn glAttachShader(program: GLuint, shader: GLuint) {
    println!("GrindKernel: Fn not implemented: glAttachShader");
}

#[no_mangle]
pub extern "C" fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
    println!("GrindKernel: Fn not implemented: glBindAttribLocation");
}

#[no_mangle]
pub extern "C" fn glBindBuffer(target: GLenum, buffer: GLuint) {
    println!("GrindKernel: Fn not implemented: glBindBuffer");
}

#[no_mangle]
pub extern "C" fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
    println!("GrindKernel: Fn not implemented: glBindFramebuffer");
}

#[no_mangle]
pub extern "C" fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
    println!("GrindKernel: Fn not implemented: glBindRenderbuffer");
}

#[no_mangle]
pub extern "C" fn glBindTexture(target: GLenum, texture: GLuint) {
    println!("GrindKernel: Fn not implemented: glBindTexture");
}

#[no_mangle]
pub extern "C" fn glBindVertexArray(array: GLuint) {
    println!("GrindKernel: Fn not implemented: glBindVertexArray");
}

#[no_mangle]
pub extern "C" fn glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    println!("GrindKernel: Fn not implemented: glBlendColor");
}

#[no_mangle]
pub extern "C" fn glBlendEquation(mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glBlendEquation");
}

#[no_mangle]
pub extern "C" fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) {
    println!("GrindKernel: Fn not implemented: glBlendEquationSeparate");
}

#[no_mangle]
pub extern "C" fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) {
    println!("GrindKernel: Fn not implemented: glBlendFunc");
}

#[no_mangle]
pub extern "C" fn glBlendFuncSeparate(
    srcRGB: GLenum,
    dstRGB: GLenum,
    srcAlpha: GLenum,
    dstAlpha: GLenum,
) {
    println!("GrindKernel: Fn not implemented: glBlendFuncSeparate");
}

#[no_mangle]
pub extern "C" fn glBufferData(
    target: GLenum,
    size: GLsizeiptr,
    data: *const GLvoid,
    usage: GLenum,
) {
    println!("GrindKernel: Fn not implemented: glBufferData");
}

#[no_mangle]
pub extern "C" fn glBufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const GLvoid,
) {
    println!("GrindKernel: Fn not implemented: glBufferSubData");
}

#[no_mangle]
pub extern "C" fn glCheckFramebufferStatus(target: GLenum) -> GLenum {
    println!("GrindKernel: Fn not implemented: glCheckFramebufferStatus");
    0
}

#[no_mangle]
pub extern "C" fn glClear(mask: GLbitfield) {
    println!("GrindKernel: Fn not implemented: glClear");
}

#[no_mangle]
pub extern "C" fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    println!("GrindKernel: Fn not implemented: glClearColor");
}

#[no_mangle]
pub extern "C" fn glClearDepthf(depth: GLclampf) {
    println!("GrindKernel: Fn not implemented: glClearDepthf");
}

#[no_mangle]
pub extern "C" fn glClearStencil(s: GLint) {
    println!("GrindKernel: Fn not implemented: glClearStencil");
}

#[no_mangle]
pub extern "C" fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
    println!("GrindKernel: Fn not implemented: glColorMask");
}

#[no_mangle]
pub extern "C" fn glCompileShader(shader: GLuint) {
    println!("GrindKernel: Fn not implemented: glCompileShader");
}

#[no_mangle]
pub extern "C" fn glCompressedTexImage2D(
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
pub extern "C" fn glCompressedTexSubImage2D(
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
pub extern "C" fn glCopyTexImage2D(
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
pub extern "C" fn glCopyTexSubImage2D(
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
pub extern "C" fn glCreateProgram() -> GLuint {
    println!("GrindKernel: Fn not implemented: glCreateProgram");
    0
}

#[no_mangle]
pub extern "C" fn glCreateShader(_type: GLenum) -> GLuint {
    println!("GrindKernel: Fn not implemented: glCreateShader");
    0
}

#[no_mangle]
pub extern "C" fn glCullFace(mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glCullFace");
}

#[no_mangle]
pub extern "C" fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteBuffers");
}

#[no_mangle]
pub extern "C" fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteFramebuffers");
}

#[no_mangle]
pub extern "C" fn glDeleteProgram(program: GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteProgram");
}

#[no_mangle]
pub extern "C" fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteRenderbuffers");
}

#[no_mangle]
pub extern "C" fn glDeleteShader(shader: GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteShader");
}

#[no_mangle]
pub extern "C" fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    println!("GrindKernel: Fn not implemented: glDeleteTextures");
}

#[no_mangle]
pub extern "C" fn glDepthFunc(func: GLenum) {
    println!("GrindKernel: Fn not implemented: glDepthFunc");
}

#[no_mangle]
pub extern "C" fn glDepthMask(flag: GLboolean) {
    println!("GrindKernel: Fn not implemented: glDepthMask");
}

#[no_mangle]
pub extern "C" fn glDepthRangef(zNear: GLclampf, zFar: GLclampf) {
    println!("GrindKernel: Fn not implemented: glDepthRangef");
}

#[no_mangle]
pub extern "C" fn glDetachShader(program: GLuint, shader: GLuint) {
    println!("GrindKernel: Fn not implemented: glDetachShader");
}

#[no_mangle]
pub extern "C" fn glDisable(cap: GLenum) {
    println!("GrindKernel: Fn not implemented: glDisable");
}

#[no_mangle]
pub extern "C" fn glDisableVertexAttribArray(index: GLuint) {
    println!("GrindKernel: Fn not implemented: glDisableVertexAttribArray");
}

#[no_mangle]
pub extern "C" fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    println!("GrindKernel: Fn not implemented: glDrawArrays");
}

#[no_mangle]
pub extern "C" fn glDrawElements(
    mode: GLenum,
    count: GLsizei,
    _type: GLenum,
    indices: *const GLvoid,
) {
    println!("GrindKernel: Fn not implemented: glDrawElements");
}

#[no_mangle]
pub extern "C" fn glDrawArraysInstanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    primcount: GLsizei,
) {
    println!("GrindKernel: Fn not implemented: glDrawArraysInstanced");
}

#[no_mangle]
pub extern "C" fn glDrawElementsInstanced(
    mode: GLenum,
    count: GLsizei,
    _type: GLenum,
    indices: *const GLvoid,
    primcount: GLsizei,
) {
    println!("GrindKernel: Fn not implemented: glDrawElementsInstanced");
}

#[no_mangle]
pub extern "C" fn glEnable(cap: GLenum) {
    println!("GrindKernel: Fn not implemented: glEnable");
}

#[no_mangle]
pub extern "C" fn glEnableVertexAttribArray(index: GLuint) {
    println!("GrindKernel: Fn not implemented: glEnableVertexAttribArray");
}

#[no_mangle]
pub extern "C" fn glFinish() {
    println!("GrindKernel: Fn not implemented: glFinish");
}

#[no_mangle]
pub extern "C" fn glFlush() {
    println!("GrindKernel: Fn not implemented: glFlush");
}

#[no_mangle]
pub extern "C" fn glFramebufferRenderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    println!("GrindKernel: Fn not implemented: glFramebufferRenderbuffer");
}

#[no_mangle]
pub extern "C" fn glFramebufferTexture2D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    println!("GrindKernel: Fn not implemented: glFramebufferTexture2D");
}

#[no_mangle]
pub extern "C" fn glFrontFace(mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glFrontFace");
}

#[no_mangle]
pub extern "C" fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
    println!("GrindKernel: Fn not implemented: glGenBuffers");
}

#[no_mangle]
pub extern "C" fn glGenerateMipmap(target: GLenum) {
    println!("GrindKernel: Fn not implemented: glGenerateMipmap");
}

#[no_mangle]
pub extern "C" fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    println!("GrindKernel: Fn not implemented: glGenFramebuffers");
}

#[no_mangle]
pub extern "C" fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    println!("GrindKernel: Fn not implemented: glGenRenderbuffers");
}

#[no_mangle]
pub extern "C" fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
    println!("GrindKernel: Fn not implemented: glGenTextures");
}

#[no_mangle]
pub extern "C" fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    println!("GrindKernel: Fn not implemented: glGenVertexArrays");
}

#[no_mangle]
pub extern "C" fn glGetActiveAttrib(
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
pub extern "C" fn glGetActiveUniform(
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
pub extern "C" fn glGetAttachedShaders(
    program: GLuint,
    maxcount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
) {
    println!("GrindKernel: Fn not implemented: glGetAttachedShaders");
}

#[no_mangle]
pub extern "C" fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
    println!("GrindKernel: Fn not implemented: glGetAttribLocation");
    0
}

#[no_mangle]
pub extern "C" fn glGetBooleanv(pname: GLenum, params: *mut GLboolean) {
    println!("GrindKernel: Fn not implemented: glGetBooleanv");
}

#[no_mangle]
pub extern "C" fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetBufferParameteriv");
}

#[no_mangle]
pub extern "C" fn glGetError() -> GLenum {
    println!("GrindKernel: Fn not implemented: glGetError");
    0
}

#[no_mangle]
pub extern "C" fn glGetFloatv(pname: GLenum, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glGetFloatv");
}

#[no_mangle]
pub extern "C" fn glGetFramebufferAttachmentParameteriv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    println!("GrindKernel: Fn not implemented: glGetFramebufferAttachmentParameteriv");
}

#[no_mangle]
pub extern "C" fn glGetIntegerv(pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetIntegerv");
}

#[no_mangle]
pub extern "C" fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetProgramiv");
}

#[no_mangle]
pub extern "C" fn glGetProgramInfoLog(
    program: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    infolog: *mut GLchar,
) {
    println!("GrindKernel: Fn not implemented: glGetProgramInfoLog");
}

#[no_mangle]
pub extern "C" fn glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetRenderbufferParameteriv");
}

#[no_mangle]
pub extern "C" fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetShaderiv");
}

#[no_mangle]
pub extern "C" fn glGetShaderInfoLog(
    shader: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    infolog: *mut GLchar,
) {
    println!("GrindKernel: Fn not implemented: glGetShaderInfoLog");
}

#[no_mangle]
pub extern "C" fn glGetShaderPrecisionFormat(
    shadertype: GLenum,
    precisiontype: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
) {
    println!("GrindKernel: Fn not implemented: glGetShaderPrecisionFormat");
}

#[no_mangle]
pub extern "C" fn glGetShaderSource(
    shader: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
) {
    println!("GrindKernel: Fn not implemented: glGetShaderSource");
}

#[no_mangle]
pub extern "C" fn glGetString(name: GLenum) -> *const GLubyte {
    println!("GrindKernel: Fn not implemented: glGetString");
    0 as *const u8
}

#[no_mangle]
pub extern "C" fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glGetTexParameterfv");
}

#[no_mangle]
pub extern "C" fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetTexParameteriv");
}

#[no_mangle]
pub extern "C" fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glGetUniformfv");
}

#[no_mangle]
pub extern "C" fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetUniformiv");
}

#[no_mangle]
pub extern "C" fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    println!("GrindKernel: Fn not implemented: glGetUniformLocation");
    0
}

#[no_mangle]
pub extern "C" fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glGetVertexAttribfv");
}

#[no_mangle]
pub extern "C" fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glGetVertexAttribiv");
}

#[no_mangle]
pub extern "C" fn glGetVertexAttribPointerv(
    index: GLuint,
    pname: GLenum,
    pointer: *mut *mut GLvoid,
) {
    println!("GrindKernel: Fn not implemented: glGetVertexAttribPointerv");
}

#[no_mangle]
pub extern "C" fn glHint(target: GLenum, mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glHint");
}

#[no_mangle]
pub extern "C" fn glIsBuffer(buffer: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsBuffer");
    0
}

#[no_mangle]
pub extern "C" fn glIsEnabled(cap: GLenum) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsEnabled");
    0
}

#[no_mangle]
pub extern "C" fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsFramebuffer");
    0
}

#[no_mangle]
pub extern "C" fn glIsProgram(program: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsProgram");
    0
}

#[no_mangle]
pub extern "C" fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsRenderbuffer");
    0
}

#[no_mangle]
pub extern "C" fn glIsShader(shader: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsShader");
    0
}

#[no_mangle]
pub extern "C" fn glIsTexture(texture: GLuint) -> GLboolean {
    println!("GrindKernel: Fn not implemented: glIsTexture");
    0
}

#[no_mangle]
pub extern "C" fn glLineWidth(width: GLfloat) {
    println!("GrindKernel: Fn not implemented: glLineWidth");
}

#[no_mangle]
pub extern "C" fn glLinkProgram(program: GLuint) {
    println!("GrindKernel: Fn not implemented: glLinkProgram");
}

#[no_mangle]
pub extern "C" fn glPixelStorei(pname: GLenum, param: GLint) {
    println!("GrindKernel: Fn not implemented: glPixelStorei");
}

#[no_mangle]
pub extern "C" fn glPolygonOffset(factor: GLfloat, units: GLfloat) {
    println!("GrindKernel: Fn not implemented: glPolygonOffset");
}

#[no_mangle]
pub extern "C" fn glPolygonMode(face: GLenum, mode: GLenum) {
    println!("GrindKernel: Fn not implemented: glPolygonMode");
}

#[no_mangle]
pub extern "C" fn glReadPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    _type: GLenum,
    pixels: *mut GLvoid,
) {
    println!("GrindKernel: Fn not implemented: glReadPixels");
}

#[no_mangle]
pub extern "C" fn glReleaseShaderCompiler() {
    println!("GrindKernel: Fn not implemented: glReleaseShaderCompiler");
}

#[no_mangle]
pub extern "C" fn glRenderbufferStorage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    println!("GrindKernel: Fn not implemented: glRenderbufferStorage");
}

#[no_mangle]
pub extern "C" fn glSampleCoverage(value: GLclampf, invert: GLboolean) {
    println!("GrindKernel: Fn not implemented: glSampleCoverage");
}

#[no_mangle]
pub extern "C" fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    println!("GrindKernel: Fn not implemented: glScissor");
}

#[no_mangle]
pub extern "C" fn glShaderBinary(
    n: GLsizei,
    shaders: *mut GLuint,
    binaryformat: GLenum,
    binary: *mut GLvoid,
    length: GLsizei,
) {
    println!("GrindKernel: Fn not implemented: glShaderBinary");
}

#[no_mangle]
pub extern "C" fn glShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
) {
    println!("GrindKernel: Fn not implemented: glShaderSource");
}

#[no_mangle]
pub extern "C" fn glStencilFunc(func: GLenum, reference: GLint, mask: GLuint) {
    println!("GrindKernel: Fn not implemented: glStencilFunc");
}

#[no_mangle]
pub extern "C" fn glStencilFuncSeparate(
    face: GLenum,
    func: GLenum,
    reference: GLint,
    mask: GLuint,
) {
    println!("GrindKernel: Fn not implemented: glStencilFuncSeparate");
}

#[no_mangle]
pub extern "C" fn glStencilMask(mask: GLuint) {
    println!("GrindKernel: Fn not implemented: glStencilMask");
}

#[no_mangle]
pub extern "C" fn glStencilMaskSeparate(face: GLenum, mask: GLuint) {
    println!("GrindKernel: Fn not implemented: glStencilMaskSeparate");
}

#[no_mangle]
pub extern "C" fn glStencilOp(_fail: GLenum, zfail: GLenum, zpass: GLenum) {
    println!("GrindKernel: Fn not implemented: glStencilOp");
}

#[no_mangle]
pub extern "C" fn glStencilOpSeparate(face: GLenum, _fail: GLenum, zfail: GLenum, zpass: GLenum) {
    println!("GrindKernel: Fn not implemented: glStencilOpSeparate");
}

#[no_mangle]
pub extern "C" fn glTexImage2D(
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
pub extern "C" fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    println!("GrindKernel: Fn not implemented: glTexParameterf");
}

#[no_mangle]
pub extern "C" fn glTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glTexParameterfv");
}

#[no_mangle]
pub extern "C" fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    println!("GrindKernel: Fn not implemented: glTexParameteri");
}

#[no_mangle]
pub extern "C" fn glTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glTexParameteriv");
}

#[no_mangle]
pub extern "C" fn glTexSubImage2D(
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
pub extern "C" fn glUniform1f(location: GLint, x: GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform1f");
}

#[no_mangle]
pub extern "C" fn glUniform1fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform1fv");
}

#[no_mangle]
pub extern "C" fn glUniform1i(location: GLint, x: GLint) {
    println!("GrindKernel: Fn not implemented: glUniform1i");
}

#[no_mangle]
pub extern "C" fn glUniform1iv(location: GLint, count: GLsizei, v: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glUniform1iv");
}

#[no_mangle]
pub extern "C" fn glUniform2f(location: GLint, x: GLfloat, y: GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform2f");
}

#[no_mangle]
pub extern "C" fn glUniform2fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform2fv");
}

#[no_mangle]
pub extern "C" fn glUniform2i(location: GLint, x: GLint, y: GLint) {
    println!("GrindKernel: Fn not implemented: glUniform2i");
}

#[no_mangle]
pub extern "C" fn glUniform2iv(location: GLint, count: GLsizei, v: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glUniform2iv");
}

#[no_mangle]
pub extern "C" fn glUniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform3f");
}

#[no_mangle]
pub extern "C" fn glUniform3fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform3fv");
}

#[no_mangle]
pub extern "C" fn glUniform3i(location: GLint, x: GLint, y: GLint, z: GLint) {
    println!("GrindKernel: Fn not implemented: glUniform3i");
}

#[no_mangle]
pub extern "C" fn glUniform3iv(location: GLint, count: GLsizei, v: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glUniform3iv");
}

#[no_mangle]
pub extern "C" fn glUniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform4f");
}

#[no_mangle]
pub extern "C" fn glUniform4fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glUniform4fv");
}

#[no_mangle]
pub extern "C" fn glUniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint) {
    println!("GrindKernel: Fn not implemented: glUniform4i");
}

#[no_mangle]
pub extern "C" fn glUniform4iv(location: GLint, count: GLsizei, v: *mut GLint) {
    println!("GrindKernel: Fn not implemented: glUniform4iv");
}

#[no_mangle]
pub extern "C" fn glUniformMatrix2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *mut GLfloat,
) {
    println!("GrindKernel: Fn not implemented: glUniformMatrix2fv");
}

#[no_mangle]
pub extern "C" fn glUniformMatrix3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *mut GLfloat,
) {
    println!("GrindKernel: Fn not implemented: glUniformMatrix3fv");
}

#[no_mangle]
pub extern "C" fn glUniformMatrix4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *mut GLfloat,
) {
    println!("GrindKernel: Fn not implemented: glUniformMatrix4fv");
}

#[no_mangle]
pub extern "C" fn glUseProgram(program: GLuint) {
    println!("GrindKernel: Fn not implemented: glUseProgram");
}

#[no_mangle]
pub extern "C" fn glValidateProgram(program: GLuint) {
    println!("GrindKernel: Fn not implemented: glValidateProgram");
}

#[no_mangle]
pub extern "C" fn glVertexAttrib1f(indx: GLuint, x: GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib1f");
}

#[no_mangle]
pub extern "C" fn glVertexAttrib1fv(indx: GLuint, values: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib1fv");
}

#[no_mangle]
pub extern "C" fn glVertexAttrib2f(indx: GLuint, x: GLfloat, y: GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib2f");
}

#[no_mangle]
pub extern "C" fn glVertexAttrib2fv(indx: GLuint, values: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib2fv");
}

#[no_mangle]
pub extern "C" fn glVertexAttrib3f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib3f");
}

#[no_mangle]
pub extern "C" fn glVertexAttrib3fv(indx: GLuint, values: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib3fv");
}

#[no_mangle]
pub extern "C" fn glVertexAttrib4f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib4f");
}

#[no_mangle]
pub extern "C" fn glVertexAttrib4fv(indx: GLuint, values: *mut GLfloat) {
    println!("GrindKernel: Fn not implemented: glVertexAttrib4fv");
}

#[no_mangle]
pub extern "C" fn glVertexAttribPointer(
    indx: GLuint,
    size: GLint,
    _type: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    ptr: *const GLvoid,
) {
    println!("GrindKernel: Fn not implemented: glVertexAttribPointer");
}

#[no_mangle]
pub extern "C" fn glVertexAttribDivisor(indx: GLuint, divisor: GLuint) {
    println!("GrindKernel: Fn not implemented: glVertexAttribDivisor");
}

#[no_mangle]
pub extern "C" fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    println!("GrindKernel: Fn not implemented: glViewport");
}
