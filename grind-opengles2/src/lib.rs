#![allow(non_snake_case)]

extern crate khronos;
extern crate libc;

use khronos::*;
use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void, ssize_t};

// GLOBAL TYPES
pub type GLbyte = khronos_int8_t;
pub type GLclampf = khronos_float_t;
pub type GLfixed = khronos_int32_t;
pub type GLshort = c_short;
pub type GLushort = c_ushort;
pub type GLvoid = c_void;
pub type GLint64 = khronos_int64_t;
pub type GLutin64 = khronos_uint64_t;
pub type GLenum = c_uint;
pub type GLuint = c_uint;
pub type GLchar = c_char;
pub type GLfloat = khronos_float_t;
pub type GLsizeptr = khronos_ssize_t;
pub type GLintptr = khronos_intptr_t;
pub type GLbitfield = c_uint;
pub type GLint = c_int;
pub type GLboolean = c_uchar;
pub type GLsizei = c_int;
pub type GLubyte = khronos_uint8_t;
pub type GLsizeiptr = ssize_t;

// GrindKernel functions
#[link(name = "grindkernel")]
extern "C" {
    fn gk_glActiveTexture(texture: GLenum);
    fn gk_glAttachShader(program: GLuint, shader: GLuint);
    fn gk_glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar);
    fn gk_glBindBuffer(target: GLenum, buffer: GLuint);
    fn gk_glBindFramebuffer(target: GLenum, framebuffer: GLuint);
    fn gk_glBindRenderbuffer(target: GLenum, renderbuffer: GLuint);
    fn gk_glBindTexture(target: GLenum, texture: GLuint);
    fn gk_glBindVertexArray(array: GLuint);
    fn gk_glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn gk_glBlendEquation(mode: GLenum);
    fn gk_glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);
    fn gk_glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    fn gk_glBlendFuncSeparate(srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
    fn gk_glBufferData(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum);
    fn gk_glBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const GLvoid);
    fn gk_glCheckFramebufferStatus(target: GLenum) -> GLenum;
    fn gk_glClear(mask: GLbitfield);
    fn gk_glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn gk_glClearDepthf(depth: GLclampf);
    fn gk_glClearStencil(s: GLint);
    fn gk_glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
    fn gk_glCompileShader(shader: GLuint);
    fn gk_glCompressedTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const GLvoid,
    );
    fn gk_glCompressedTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const GLvoid,
    );
    fn gk_glCopyTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    );
    fn gk_glCopyTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
    fn gk_glCreateProgram() -> GLuint;
    fn gk_glCreateShader(_type: GLenum) -> GLuint;
    fn gk_glCullFace(mode: GLenum);
    fn gk_glDeleteBuffers(n: GLsizei, buffers: *const GLuint);
    fn gk_glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
    fn gk_glDeleteProgram(program: GLuint);
    fn gk_glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint);
    fn gk_glDeleteShader(shader: GLuint);
    fn gk_glDeleteTextures(n: GLsizei, textures: *const GLuint);
    fn gk_glDepthFunc(func: GLenum);
    fn gk_glDepthMask(flag: GLboolean);
    fn gk_glDepthRangef(zNear: GLclampf, zFar: GLclampf);
    fn gk_glDetachShader(program: GLuint, shader: GLuint);
    fn gk_glDisable(cap: GLenum);
    fn gk_glDisableVertexAttribArray(index: GLuint);
    fn gk_glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
    fn gk_glDrawElements(mode: GLenum, count: GLsizei, _type: GLenum, indices: *const GLvoid);
    fn gk_glDrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, primcount: GLsizei);
    fn gk_glDrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        _type: GLenum,
        indices: *const GLvoid,
        primcount: GLsizei,
    );
    fn gk_glEnable(cap: GLenum);
    fn gk_glEnableVertexAttribArray(index: GLuint);
    fn gk_glFinish();
    fn gk_glFlush();
    fn gk_glFramebufferRenderbuffer(
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    );
    fn gk_glFramebufferTexture2D(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    );
    fn gk_glFrontFace(mode: GLenum);
    fn gk_glGenBuffers(n: GLsizei, buffers: *mut GLuint);
    fn gk_glGenerateMipmap(target: GLenum);
    fn gk_glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
    fn gk_glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);
    fn gk_glGenTextures(n: GLsizei, textures: *mut GLuint);
    fn gk_glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);
    fn gk_glGetActiveAttrib(
        program: GLuint,
        index: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        _type: *mut GLenum,
        name: *mut GLchar,
    );
    fn gk_glGetActiveUniform(
        program: GLuint,
        index: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        _type: *mut GLenum,
        name: *mut GLchar,
    );
    fn gk_glGetAttachedShaders(
        program: GLuint,
        maxcount: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint,
    );
    fn gk_glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;
    fn gk_glGetBooleanv(pname: GLenum, params: *mut GLboolean);
    fn gk_glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
    fn gk_glGetError() -> GLenum;
    fn gk_glGetFloatv(pname: GLenum, params: *mut GLfloat);
    fn gk_glGetFramebufferAttachmentParameteriv(
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    );
    fn gk_glGetIntegerv(pname: GLenum, params: *mut GLint);
    fn gk_glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
    fn gk_glGetProgramInfoLog(
        program: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        infolog: *mut GLchar,
    );
    fn gk_glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
    fn gk_glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
    fn gk_glGetShaderInfoLog(
        shader: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        infolog: *mut GLchar,
    );
    fn gk_glGetShaderPrecisionFormat(
        shadertype: GLenum,
        precisiontype: GLenum,
        range: *mut GLint,
        precision: *mut GLint,
    );
    fn gk_glGetShaderSource(
        shader: GLuint,
        bufsize: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar,
    );
    fn gk_glGetString(name: GLenum) -> *const GLubyte;
    fn gk_glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat);
    fn gk_glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
    fn gk_glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat);
    fn gk_glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint);
    fn gk_glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
    fn gk_glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat);
    fn gk_glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint);
    fn gk_glGetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *mut *mut GLvoid);
    fn gk_glHint(target: GLenum, mode: GLenum);
    fn gk_glIsBuffer(buffer: GLuint) -> GLboolean;
    fn gk_glIsEnabled(cap: GLenum) -> GLboolean;
    fn gk_glIsFramebuffer(framebuffer: GLuint) -> GLboolean;
    fn gk_glIsProgram(program: GLuint) -> GLboolean;
    fn gk_glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean;
    fn gk_glIsShader(shader: GLuint) -> GLboolean;
    fn gk_glIsTexture(texture: GLuint) -> GLboolean;
    fn gk_glLineWidth(width: GLfloat);
    fn gk_glLinkProgram(program: GLuint);
    fn gk_glPixelStorei(pname: GLenum, param: GLint);
    fn gk_glPolygonOffset(factor: GLfloat, units: GLfloat);
    fn gk_glPolygonMode(face: GLenum, mode: GLenum);
    fn gk_glReadPixels(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        _type: GLenum,
        pixels: *mut GLvoid,
    );
    fn gk_glReleaseShaderCompiler();
    fn gk_glRenderbufferStorage(
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
    fn gk_glSampleCoverage(value: GLclampf, invert: GLboolean);
    fn gk_glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn gk_glShaderBinary(
        n: GLsizei,
        shaders: *mut GLuint,
        binaryformat: GLenum,
        binary: *mut GLvoid,
        length: GLsizei,
    );
    fn gk_glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    );
    fn gk_glStencilFunc(func: GLenum, reference: GLint, mask: GLuint);
    fn gk_glStencilFuncSeparate(face: GLenum, func: GLenum, reference: GLint, mask: GLuint);
    fn gk_glStencilMask(mask: GLuint);
    fn gk_glStencilMaskSeparate(face: GLenum, mask: GLuint);
    fn gk_glStencilOp(_fail: GLenum, zfail: GLenum, zpass: GLenum);
    fn gk_glStencilOpSeparate(face: GLenum, _fail: GLenum, zfail: GLenum, zpass: GLenum);
    fn gk_glTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        _type: GLenum,
        pixels: *const GLvoid,
    );
    fn gk_glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
    fn gk_glTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat);
    fn gk_glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn gk_glTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
    fn gk_glTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        _type: GLenum,
        pixels: *const GLvoid,
    );
    fn gk_glUniform1f(location: GLint, x: GLfloat);
    fn gk_glUniform1fv(location: GLint, count: GLsizei, v: *mut GLfloat);
    fn gk_glUniform1i(location: GLint, x: GLint);
    fn gk_glUniform1iv(location: GLint, count: GLsizei, v: *mut GLint);
    fn gk_glUniform2f(location: GLint, x: GLfloat, y: GLfloat);
    fn gk_glUniform2fv(location: GLint, count: GLsizei, v: *mut GLfloat);
    fn gk_glUniform2i(location: GLint, x: GLint, y: GLint);
    fn gk_glUniform2iv(location: GLint, count: GLsizei, v: *mut GLint);
    fn gk_glUniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat);
    fn gk_glUniform3fv(location: GLint, count: GLsizei, v: *mut GLfloat);
    fn gk_glUniform3i(location: GLint, x: GLint, y: GLint, z: GLint);
    fn gk_glUniform3iv(location: GLint, count: GLsizei, v: *mut GLint);
    fn gk_glUniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
    fn gk_glUniform4fv(location: GLint, count: GLsizei, v: *mut GLfloat);
    fn gk_glUniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint);
    fn gk_glUniform4iv(location: GLint, count: GLsizei, v: *mut GLint);
    fn gk_glUniformMatrix2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *mut GLfloat,
    );
    fn gk_glUniformMatrix3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *mut GLfloat,
    );
    fn gk_glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *mut GLfloat,
    );
    fn gk_glUseProgram(program: GLuint);
    fn gk_glValidateProgram(program: GLuint);
    fn gk_glVertexAttrib1f(indx: GLuint, x: GLfloat);
    fn gk_glVertexAttrib1fv(indx: GLuint, values: *mut GLfloat);
    fn gk_glVertexAttrib2f(indx: GLuint, x: GLfloat, y: GLfloat);
    fn gk_glVertexAttrib2fv(indx: GLuint, values: *mut GLfloat);
    fn gk_glVertexAttrib3f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
    fn gk_glVertexAttrib3fv(indx: GLuint, values: *mut GLfloat);
    fn gk_glVertexAttrib4f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
    fn gk_glVertexAttrib4fv(indx: GLuint, values: *mut GLfloat);
    fn gk_glVertexAttribPointer(
        indx: GLuint,
        size: GLint,
        _type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        ptr: *const GLvoid,
    );
    fn gk_glVertexAttribDivisor(indx: GLuint, divisor: GLuint);
    fn gk_glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}

// OpenGL functions
#[no_mangle]
pub extern "C" fn glActiveTexture(texture: GLenum) {
    unsafe { gk_glActiveTexture(texture) }
}

#[no_mangle]
pub extern "C" fn glAttachShader(program: GLuint, shader: GLuint) {
    unsafe { gk_glAttachShader(program, shader) }
}

#[no_mangle]
pub extern "C" fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
    unsafe { gk_glBindAttribLocation(program, index, name) }
}

#[no_mangle]
pub extern "C" fn glBindBuffer(target: GLenum, buffer: GLuint) {
    unsafe { gk_glBindBuffer(target, buffer) }
}

#[no_mangle]
pub extern "C" fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
    unsafe { gk_glBindFramebuffer(target, framebuffer) }
}

#[no_mangle]
pub extern "C" fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
    unsafe { gk_glBindRenderbuffer(target, renderbuffer) }
}

#[no_mangle]
pub extern "C" fn glBindTexture(target: GLenum, texture: GLuint) {
    unsafe { gk_glBindTexture(target, texture) }
}

#[no_mangle]
pub extern "C" fn glBindVertexArray(array: GLuint) {
    unsafe { gk_glBindVertexArray(array) }
}

#[no_mangle]
pub extern "C" fn glBlendColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    unsafe { gk_glBlendColor(red, green, blue, alpha) }
}

#[no_mangle]
pub extern "C" fn glBlendEquation(mode: GLenum) {
    unsafe { gk_glBlendEquation(mode) }
}

#[no_mangle]
pub extern "C" fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) {
    unsafe { gk_glBlendEquationSeparate(modeRGB, modeAlpha) }
}

#[no_mangle]
pub extern "C" fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) {
    unsafe { gk_glBlendFunc(sfactor, dfactor) }
}

#[no_mangle]
pub extern "C" fn glBlendFuncSeparate(
    srcRGB: GLenum,
    dstRGB: GLenum,
    srcAlpha: GLenum,
    dstAlpha: GLenum,
) {
    unsafe { gk_glBlendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha) }
}

#[no_mangle]
pub extern "C" fn glBufferData(
    target: GLenum,
    size: GLsizeiptr,
    data: *const GLvoid,
    usage: GLenum,
) {
    unsafe { gk_glBufferData(target, size, data, usage) }
}

#[no_mangle]
pub extern "C" fn glBufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const GLvoid,
) {
    unsafe { gk_glBufferSubData(target, offset, size, data) }
}

#[no_mangle]
pub extern "C" fn glCheckFramebufferStatus(target: GLenum) -> GLenum {
    unsafe { gk_glCheckFramebufferStatus(target) }
}

#[no_mangle]
pub extern "C" fn glClear(mask: GLbitfield) {
    unsafe { gk_glClear(mask) }
}

#[no_mangle]
pub extern "C" fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    unsafe { gk_glClearColor(red, green, blue, alpha) }
}

#[no_mangle]
pub extern "C" fn glClearDepthf(depth: GLclampf) {
    unsafe { gk_glClearDepthf(depth) }
}

#[no_mangle]
pub extern "C" fn glClearStencil(s: GLint) {
    unsafe { gk_glClearStencil(s) }
}

#[no_mangle]
pub extern "C" fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
    unsafe { gk_glColorMask(red, green, blue, alpha) }
}

#[no_mangle]
pub extern "C" fn glCompileShader(shader: GLuint) {
    unsafe { gk_glCompileShader(shader) }
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
    unsafe {
        gk_glCompressedTexImage2D(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            imageSize,
            data,
        )
    }
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
    unsafe {
        gk_glCompressedTexSubImage2D(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            imageSize,
            data,
        )
    }
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
    unsafe { gk_glCopyTexImage2D(target, level, internalformat, x, y, width, height, border) }
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
    unsafe { gk_glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height) }
}

#[no_mangle]
pub extern "C" fn glCreateProgram() -> GLuint {
    unsafe { gk_glCreateProgram() }
}

#[no_mangle]
pub extern "C" fn glCreateShader(_type: GLenum) -> GLuint {
    unsafe { gk_glCreateShader(_type) }
}

#[no_mangle]
pub extern "C" fn glCullFace(mode: GLenum) {
    unsafe { gk_glCullFace(mode) }
}

#[no_mangle]
pub extern "C" fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    unsafe { gk_glDeleteBuffers(n, buffers) }
}

#[no_mangle]
pub extern "C" fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    unsafe { gk_glDeleteFramebuffers(n, framebuffers) }
}

#[no_mangle]
pub extern "C" fn glDeleteProgram(program: GLuint) {
    unsafe { gk_glDeleteProgram(program) }
}

#[no_mangle]
pub extern "C" fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    unsafe { gk_glDeleteRenderbuffers(n, renderbuffers) }
}

#[no_mangle]
pub extern "C" fn glDeleteShader(shader: GLuint) {
    unsafe { gk_glDeleteShader(shader) }
}

#[no_mangle]
pub extern "C" fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    unsafe { gk_glDeleteTextures(n, textures) }
}

#[no_mangle]
pub extern "C" fn glDepthFunc(func: GLenum) {
    unsafe { gk_glDepthFunc(func) }
}

#[no_mangle]
pub extern "C" fn glDepthMask(flag: GLboolean) {
    unsafe { gk_glDepthMask(flag) }
}

#[no_mangle]
pub extern "C" fn glDepthRangef(zNear: GLclampf, zFar: GLclampf) {
    unsafe { gk_glDepthRangef(zNear, zFar) }
}

#[no_mangle]
pub extern "C" fn glDetachShader(program: GLuint, shader: GLuint) {
    unsafe { gk_glDetachShader(program, shader) }
}

#[no_mangle]
pub extern "C" fn glDisable(cap: GLenum) {
    unsafe { gk_glDisable(cap) }
}

#[no_mangle]
pub extern "C" fn glDisableVertexAttribArray(index: GLuint) {
    unsafe { gk_glDisableVertexAttribArray(index) }
}

#[no_mangle]
pub extern "C" fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    unsafe { gk_glDrawArrays(mode, first, count) }
}

#[no_mangle]
pub extern "C" fn glDrawElements(
    mode: GLenum,
    count: GLsizei,
    _type: GLenum,
    indices: *const GLvoid,
) {
    unsafe { gk_glDrawElements(mode, count, _type, indices) }
}

#[no_mangle]
pub extern "C" fn glDrawArraysInstanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    primcount: GLsizei,
) {
    unsafe { gk_glDrawArraysInstanced(mode, first, count, primcount) }
}

#[no_mangle]
pub extern "C" fn glDrawElementsInstanced(
    mode: GLenum,
    count: GLsizei,
    _type: GLenum,
    indices: *const GLvoid,
    primcount: GLsizei,
) {
    unsafe { gk_glDrawElementsInstanced(mode, count, _type, indices, primcount) }
}

#[no_mangle]
pub extern "C" fn glEnable(cap: GLenum) {
    unsafe { gk_glEnable(cap) }
}

#[no_mangle]
pub extern "C" fn glEnableVertexAttribArray(index: GLuint) {
    unsafe { gk_glEnableVertexAttribArray(index) }
}

#[no_mangle]
pub extern "C" fn glFinish() {
    unsafe { gk_glFinish() }
}

#[no_mangle]
pub extern "C" fn glFlush() {
    unsafe { gk_glFlush() }
}

#[no_mangle]
pub extern "C" fn glFramebufferRenderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    unsafe { gk_glFramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer) }
}

#[no_mangle]
pub extern "C" fn glFramebufferTexture2D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    unsafe { gk_glFramebufferTexture2D(target, attachment, textarget, texture, level) }
}

#[no_mangle]
pub extern "C" fn glFrontFace(mode: GLenum) {
    unsafe { gk_glFrontFace(mode) }
}

#[no_mangle]
pub extern "C" fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
    unsafe { gk_glGenBuffers(n, buffers) }
}

#[no_mangle]
pub extern "C" fn glGenerateMipmap(target: GLenum) {
    unsafe { gk_glGenerateMipmap(target) }
}

#[no_mangle]
pub extern "C" fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    unsafe { gk_glGenFramebuffers(n, framebuffers) }
}

#[no_mangle]
pub extern "C" fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    unsafe { gk_glGenRenderbuffers(n, renderbuffers) }
}

#[no_mangle]
pub extern "C" fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
    unsafe { gk_glGenTextures(n, textures) }
}

#[no_mangle]
pub extern "C" fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    unsafe { gk_glGenVertexArrays(n, arrays) }
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
    unsafe { gk_glGetActiveAttrib(program, index, bufsize, length, size, _type, name) }
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
    unsafe { gk_glGetActiveUniform(program, index, bufsize, length, size, _type, name) }
}

#[no_mangle]
pub extern "C" fn glGetAttachedShaders(
    program: GLuint,
    maxcount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
) {
    unsafe { gk_glGetAttachedShaders(program, maxcount, count, shaders) }
}

#[no_mangle]
pub extern "C" fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
    unsafe { gk_glGetAttribLocation(program, name) }
}

#[no_mangle]
pub extern "C" fn glGetBooleanv(pname: GLenum, params: *mut GLboolean) {
    unsafe { gk_glGetBooleanv(pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { gk_glGetBufferParameteriv(target, pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetError() -> GLenum {
    unsafe { gk_glGetError() }
}

#[no_mangle]
pub extern "C" fn glGetFloatv(pname: GLenum, params: *mut GLfloat) {
    unsafe { gk_glGetFloatv(pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetFramebufferAttachmentParameteriv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe { gk_glGetFramebufferAttachmentParameteriv(target, attachment, pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetIntegerv(pname: GLenum, params: *mut GLint) {
    unsafe { gk_glGetIntegerv(pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { gk_glGetProgramiv(program, pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetProgramInfoLog(
    program: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    infolog: *mut GLchar,
) {
    unsafe { gk_glGetProgramInfoLog(program, bufsize, length, infolog) }
}

#[no_mangle]
pub extern "C" fn glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { gk_glGetRenderbufferParameteriv(target, pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { gk_glGetShaderiv(shader, pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetShaderInfoLog(
    shader: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    infolog: *mut GLchar,
) {
    unsafe { gk_glGetShaderInfoLog(shader, bufsize, length, infolog) }
}

#[no_mangle]
pub extern "C" fn glGetShaderPrecisionFormat(
    shadertype: GLenum,
    precisiontype: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
) {
    unsafe { gk_glGetShaderPrecisionFormat(shadertype, precisiontype, range, precision) }
}

#[no_mangle]
pub extern "C" fn glGetShaderSource(
    shader: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
) {
    unsafe { gk_glGetShaderSource(shader, bufsize, length, source) }
}

#[no_mangle]
pub extern "C" fn glGetString(name: GLenum) -> *const GLubyte {
    unsafe { gk_glGetString(name) }
}

#[no_mangle]
pub extern "C" fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    unsafe { gk_glGetTexParameterfv(target, pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { gk_glGetTexParameteriv(target, pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) {
    unsafe { gk_glGetUniformfv(program, location, params) }
}

#[no_mangle]
pub extern "C" fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint) {
    unsafe { gk_glGetUniformiv(program, location, params) }
}

#[no_mangle]
pub extern "C" fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    unsafe { gk_glGetUniformLocation(program, name) }
}

#[no_mangle]
pub extern "C" fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) {
    unsafe { gk_glGetVertexAttribfv(index, pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe { gk_glGetVertexAttribiv(index, pname, params) }
}

#[no_mangle]
pub extern "C" fn glGetVertexAttribPointerv(
    index: GLuint,
    pname: GLenum,
    pointer: *mut *mut GLvoid,
) {
    unsafe { gk_glGetVertexAttribPointerv(index, pname, pointer) }
}

#[no_mangle]
pub extern "C" fn glHint(target: GLenum, mode: GLenum) {
    unsafe { gk_glHint(target, mode) }
}

#[no_mangle]
pub extern "C" fn glIsBuffer(buffer: GLuint) -> GLboolean {
    unsafe { gk_glIsBuffer(buffer) }
}

#[no_mangle]
pub extern "C" fn glIsEnabled(cap: GLenum) -> GLboolean {
    unsafe { gk_glIsEnabled(cap) }
}

#[no_mangle]
pub extern "C" fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
    unsafe { gk_glIsFramebuffer(framebuffer) }
}

#[no_mangle]
pub extern "C" fn glIsProgram(program: GLuint) -> GLboolean {
    unsafe { gk_glIsProgram(program) }
}

#[no_mangle]
pub extern "C" fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
    unsafe { gk_glIsRenderbuffer(renderbuffer) }
}

#[no_mangle]
pub extern "C" fn glIsShader(shader: GLuint) -> GLboolean {
    unsafe { gk_glIsShader(shader) }
}

#[no_mangle]
pub extern "C" fn glIsTexture(texture: GLuint) -> GLboolean {
    unsafe { gk_glIsTexture(texture) }
}

#[no_mangle]
pub extern "C" fn glLineWidth(width: GLfloat) {
    unsafe { gk_glLineWidth(width) }
}

#[no_mangle]
pub extern "C" fn glLinkProgram(program: GLuint) {
    unsafe { gk_glLinkProgram(program) }
}

#[no_mangle]
pub extern "C" fn glPixelStorei(pname: GLenum, param: GLint) {
    unsafe { gk_glPixelStorei(pname, param) }
}

#[no_mangle]
pub extern "C" fn glPolygonOffset(factor: GLfloat, units: GLfloat) {
    unsafe { gk_glPolygonOffset(factor, units) }
}

#[no_mangle]
pub extern "C" fn glPolygonMode(face: GLenum, mode: GLenum) {
    unsafe { gk_glPolygonMode(face, mode) }
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
    unsafe { gk_glReadPixels(x, y, width, height, format, _type, pixels) }
}

#[no_mangle]
pub extern "C" fn glReleaseShaderCompiler() {
    unsafe { gk_glReleaseShaderCompiler() }
}

#[no_mangle]
pub extern "C" fn glRenderbufferStorage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe { gk_glRenderbufferStorage(target, internalformat, width, height) }
}

#[no_mangle]
pub extern "C" fn glSampleCoverage(value: GLclampf, invert: GLboolean) {
    unsafe { gk_glSampleCoverage(value, invert) }
}

#[no_mangle]
pub extern "C" fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { gk_glScissor(x, y, width, height) }
}

#[no_mangle]
pub extern "C" fn glShaderBinary(
    n: GLsizei,
    shaders: *mut GLuint,
    binaryformat: GLenum,
    binary: *mut GLvoid,
    length: GLsizei,
) {
    unsafe { gk_glShaderBinary(n, shaders, binaryformat, binary, length) }
}

#[no_mangle]
pub extern "C" fn glShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
) {
    unsafe { gk_glShaderSource(shader, count, string, length) }
}

#[no_mangle]
pub extern "C" fn glStencilFunc(func: GLenum, reference: GLint, mask: GLuint) {
    unsafe { gk_glStencilFunc(func, reference, mask) }
}

#[no_mangle]
pub extern "C" fn glStencilFuncSeparate(
    face: GLenum,
    func: GLenum,
    reference: GLint,
    mask: GLuint,
) {
    unsafe { gk_glStencilFuncSeparate(face, func, reference, mask) }
}

#[no_mangle]
pub extern "C" fn glStencilMask(mask: GLuint) {
    unsafe { gk_glStencilMask(mask) }
}

#[no_mangle]
pub extern "C" fn glStencilMaskSeparate(face: GLenum, mask: GLuint) {
    unsafe { gk_glStencilMaskSeparate(face, mask) }
}

#[no_mangle]
pub extern "C" fn glStencilOp(_fail: GLenum, zfail: GLenum, zpass: GLenum) {
    unsafe { gk_glStencilOp(_fail, zfail, zpass) }
}

#[no_mangle]
pub extern "C" fn glStencilOpSeparate(face: GLenum, _fail: GLenum, zfail: GLenum, zpass: GLenum) {
    unsafe { gk_glStencilOpSeparate(face, _fail, zfail, zpass) }
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
    unsafe {
        gk_glTexImage2D(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            _type,
            pixels,
        )
    }
}

#[no_mangle]
pub extern "C" fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    unsafe { gk_glTexParameterf(target, pname, param) }
}

#[no_mangle]
pub extern "C" fn glTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    unsafe { gk_glTexParameterfv(target, pname, params) }
}

#[no_mangle]
pub extern "C" fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe { gk_glTexParameteri(target, pname, param) }
}

#[no_mangle]
pub extern "C" fn glTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe { gk_glTexParameteriv(target, pname, params) }
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
    unsafe {
        gk_glTexSubImage2D(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            _type,
            pixels,
        )
    }
}

#[no_mangle]
pub extern "C" fn glUniform1f(location: GLint, x: GLfloat) {
    unsafe { gk_glUniform1f(location, x) }
}

#[no_mangle]
pub extern "C" fn glUniform1fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    unsafe { gk_glUniform1fv(location, count, v) }
}

#[no_mangle]
pub extern "C" fn glUniform1i(location: GLint, x: GLint) {
    unsafe { gk_glUniform1i(location, x) }
}

#[no_mangle]
pub extern "C" fn glUniform1iv(location: GLint, count: GLsizei, v: *mut GLint) {
    unsafe { gk_glUniform1iv(location, count, v) }
}

#[no_mangle]
pub extern "C" fn glUniform2f(location: GLint, x: GLfloat, y: GLfloat) {
    unsafe { gk_glUniform2f(location, x, y) }
}

#[no_mangle]
pub extern "C" fn glUniform2fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    unsafe { gk_glUniform2fv(location, count, v) }
}

#[no_mangle]
pub extern "C" fn glUniform2i(location: GLint, x: GLint, y: GLint) {
    unsafe { gk_glUniform2i(location, x, y) }
}

#[no_mangle]
pub extern "C" fn glUniform2iv(location: GLint, count: GLsizei, v: *mut GLint) {
    unsafe { gk_glUniform2iv(location, count, v) }
}

#[no_mangle]
pub extern "C" fn glUniform3f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe { gk_glUniform3f(location, x, y, z) }
}

#[no_mangle]
pub extern "C" fn glUniform3fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    unsafe { gk_glUniform3fv(location, count, v) }
}

#[no_mangle]
pub extern "C" fn glUniform3i(location: GLint, x: GLint, y: GLint, z: GLint) {
    unsafe { gk_glUniform3i(location, x, y, z) }
}

#[no_mangle]
pub extern "C" fn glUniform3iv(location: GLint, count: GLsizei, v: *mut GLint) {
    unsafe { gk_glUniform3iv(location, count, v) }
}

#[no_mangle]
pub extern "C" fn glUniform4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe { gk_glUniform4f(location, x, y, z, w) }
}

#[no_mangle]
pub extern "C" fn glUniform4fv(location: GLint, count: GLsizei, v: *mut GLfloat) {
    unsafe { gk_glUniform4fv(location, count, v) }
}

#[no_mangle]
pub extern "C" fn glUniform4i(location: GLint, x: GLint, y: GLint, z: GLint, w: GLint) {
    unsafe { gk_glUniform4i(location, x, y, z, w) }
}

#[no_mangle]
pub extern "C" fn glUniform4iv(location: GLint, count: GLsizei, v: *mut GLint) {
    unsafe { gk_glUniform4iv(location, count, v) }
}

#[no_mangle]
pub extern "C" fn glUniformMatrix2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *mut GLfloat,
) {
    unsafe { gk_glUniformMatrix2fv(location, count, transpose, value) }
}

#[no_mangle]
pub extern "C" fn glUniformMatrix3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *mut GLfloat,
) {
    unsafe { gk_glUniformMatrix3fv(location, count, transpose, value) }
}

#[no_mangle]
pub extern "C" fn glUniformMatrix4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *mut GLfloat,
) {
    unsafe { gk_glUniformMatrix4fv(location, count, transpose, value) }
}

#[no_mangle]
pub extern "C" fn glUseProgram(program: GLuint) {
    unsafe { gk_glUseProgram(program) }
}

#[no_mangle]
pub extern "C" fn glValidateProgram(program: GLuint) {
    unsafe { gk_glValidateProgram(program) }
}

#[no_mangle]
pub extern "C" fn glVertexAttrib1f(indx: GLuint, x: GLfloat) {
    unsafe { gk_glVertexAttrib1f(indx, x) }
}

#[no_mangle]
pub extern "C" fn glVertexAttrib1fv(indx: GLuint, values: *mut GLfloat) {
    unsafe { gk_glVertexAttrib1fv(indx, values) }
}

#[no_mangle]
pub extern "C" fn glVertexAttrib2f(indx: GLuint, x: GLfloat, y: GLfloat) {
    unsafe { gk_glVertexAttrib2f(indx, x, y) }
}

#[no_mangle]
pub extern "C" fn glVertexAttrib2fv(indx: GLuint, values: *mut GLfloat) {
    unsafe { gk_glVertexAttrib2fv(indx, values) }
}

#[no_mangle]
pub extern "C" fn glVertexAttrib3f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe { gk_glVertexAttrib3f(indx, x, y, z) }
}

#[no_mangle]
pub extern "C" fn glVertexAttrib3fv(indx: GLuint, values: *mut GLfloat) {
    unsafe { gk_glVertexAttrib3fv(indx, values) }
}

#[no_mangle]
pub extern "C" fn glVertexAttrib4f(indx: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe { gk_glVertexAttrib4f(indx, x, y, z, w) }
}

#[no_mangle]
pub extern "C" fn glVertexAttrib4fv(indx: GLuint, values: *mut GLfloat) {
    unsafe { gk_glVertexAttrib4fv(indx, values) }
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
    unsafe { gk_glVertexAttribPointer(indx, size, _type, normalized, stride, ptr) }
}

#[no_mangle]
pub extern "C" fn glVertexAttribDivisor(indx: GLuint, divisor: GLuint) {
    unsafe { gk_glVertexAttribDivisor(indx, divisor) }
}

#[no_mangle]
pub extern "C" fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { gk_glViewport(x, y, width, height) }
}
