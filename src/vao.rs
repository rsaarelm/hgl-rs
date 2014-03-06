use gl;
use std::libc::c_void;
use gl::types::{GLint, GLuint, GLsizei, GLvoid};

use Program;
use Primitive;

/// A vertex array object
pub struct Vao {
    name: GLuint
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.name); }
    }
}

impl Vao {
    pub fn new() -> Vao {
        let mut vao: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao as *mut GLuint); }
        Vao { name: vao }
    }

    pub fn bind(&self) {
        gl::BindVertexArray(self.name);
    }

    /// Define and enable an array of generic vertex attribute data for `name`
    /// in `program`, in this VAO, using the bound VBO. TODO: Currently
    /// hardcoded to GL_FLOAT.  TODO: Normalize hardcoded to GL_FALSE.
    ///
    /// NOTE: Memory unsafety caused when no bound VBO, or bound VBO does not
    /// have enough data.
    pub fn enable_attrib(&self, program: &Program, name: &str, elts: GLint,
                         stride: GLint, offset: uint) {
        self.bind();
        name.with_c_str(|cstr| {
            unsafe {
                let pos = gl::GetAttribLocation(program.name, cstr);
                gl::EnableVertexAttribArray(pos as GLuint);
                gl::VertexAttribPointer(pos as GLuint, elts, gl::FLOAT,
                                        gl::FALSE, stride, offset as *c_void);
            }
        });
    }

    pub fn disable_attrib(&self, program: &Program, name: &str) {
        self.bind();
        name.with_c_str(|cstr| {
            let pos = unsafe { gl::GetAttribLocation(program.name, cstr) };
            gl::DisableVertexAttribArray(pos as GLuint);
        });
    }

    /// Draw the given primitive, using `count` vertices starting at offset
    /// `first` in the currently bound VBO.
    pub fn draw_array(&self, primitive: Primitive, first: GLint, count: GLsizei) {
        gl::DrawArrays(primitive.to_glenum(), first, count);
    }

    /// Draw the given primitive, using `count` vertices starting at offset
    /// `first` in the currently bound EBO.
    ///
    /// TODO: Hardcoded to GL_UNSIGNED_INT
    pub fn draw_elements(&self, primitive: Primitive, first: GLint, count: GLint) {
        // last argument null; use the bound buffer
        unsafe {
            gl::DrawElements(primitive.to_glenum(), count, gl::UNSIGNED_INT, first as *GLvoid);
        }
    }
}
