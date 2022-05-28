#[link(name = "EGL")]
#[link(name = "GLESv2")]
extern {}

use std::ffi::{CString};
use std::mem;
// use std::sync::Arc;

use khronos_egl as egl;

mod gl {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() -> Result<(), egl::Error> {
	let egl = egl::Instance::new(egl::Static);
	egl.bind_api(egl::OPENGL_ES_API)?;

	let display = egl.get_display(egl::DEFAULT_DISPLAY).expect("there should be a display");
	let version = egl.initialize(display)?;

	// egl.get_proc_address("eglGetDisplay").unwrap();

	let vendor = egl.query_string(Some(display), egl::VENDOR).unwrap();
	let display_extensions = egl
		.query_string(Some(display), egl::EXTENSIONS)
		.unwrap()
		.to_string_lossy();
	println!(
		"Display vendor {:?}, version {:?}, extensions: {:?}",
		vendor,
		version,
		display_extensions
	);

	let attributes = [
		// egl::RED_SIZE, 8,
		// egl::GREEN_SIZE, 8,
		// egl::BLUE_SIZE, 8,
		egl::RENDERABLE_TYPE, egl::OPENGL_ES2_BIT,
		egl::NONE
	];
	let config = egl.choose_first_config(display, &attributes)?.expect("unable to find an appropriate EGL configuration");

	println!("creating surface");
	let surface = unsafe {
		egl.create_window_surface(display, config, 0 as *mut core::ffi::c_void, None)?
	};

	println!("creating context");
	let context_attributes = [
		egl::CONTEXT_MAJOR_VERSION, 2,
		egl::CONTEXT_MINOR_VERSION, 0,
		egl::NONE
	];
	let context = egl.create_context(display, config, None, &context_attributes)?;

	println!("connecting context");
	egl.make_current(display, Some(surface), Some(surface), Some(context))?;
	egl.swap_interval(display, 1)?;

	println!("create GL context");
	let gl = gl::Gles2::load_with(|_| 0 as *const core::ffi::c_void);

	unsafe {
		println!("GL stuff");
		let program = gl.CreateProgram();

		let shader_version = "#version 100";
		let (vertex_shader_source, fragment_shader_source) = (
			r#"
				attribute vec3 aPos;
				void main() {
					gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
				}
			"#,
			r#"
				precision mediump float;
				void main() {
					gl_FragColor = vec4(1.0, 0.5, 0.2, 1.0);
				}
			"#,
		);

		let shader_sources = [
			(gl::VERTEX_SHADER, vertex_shader_source),
			(gl::FRAGMENT_SHADER, fragment_shader_source),
		];

		let mut shaders = Vec::with_capacity(shader_sources.len());
		for (shader_type, shader_source) in shader_sources.iter() {
			let shader = gl
				.CreateShader(*shader_type);

			let shader_source = &format!("{}\n{}", shader_version, shader_source);
			gl.ShaderSource(
				shader,
				1,
				&(shader_source.as_ptr() as *const gl::types::GLchar),
				&(shader_source.len() as gl::types::GLint),
			);
			gl.CompileShader(shader);

			let mut success = 0;
			gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
			if success != 1 {
				let mut length = 0;
				gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
				let log = if length > 0 {
					let mut log = String::with_capacity(length as usize);
					log.extend(std::iter::repeat('\0').take(length as usize));
					gl.GetShaderInfoLog(
						shader,
						length,
						&mut length,
						(&log[..]).as_ptr() as *mut gl::types::GLchar,
					);
					log.truncate(length as usize);
					log
				} else {
					String::from("")
				};
				panic!("failed to compile shader:\n{}", log);
			}
			gl.AttachShader(program, shader);
			shaders.push(shader);
		}

		let attrib_name = CString::new("aPos").unwrap();
		gl.BindAttribLocation(program, 0, attrib_name.as_ptr());

		gl.LinkProgram(program);

		let mut status = 0;
		gl.GetProgramiv(program, gl::LINK_STATUS, &mut status);
		if status != 1 {
			let mut length = 0;
			gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut length);
			let log = if length > 0 {
				let mut log = String::with_capacity(length as usize);
				log.extend(std::iter::repeat('\0').take(length as usize));
				gl.GetProgramInfoLog(
					program,
					length,
					&mut length,
					(&log[..]).as_ptr() as *mut gl::types::GLchar,
				);
				log.truncate(length as usize);
				log
			} else {
				String::from("")
			};
			panic!("failed to link program:\n{}", log);
		}

		for shader in shaders {
			gl.DetachShader(program, shader);
			gl.DeleteShader(shader);
		}

		let vertices: [f32; 9] = [
			0.0, 0.5, 0.0,
			-0.5, -0.5, 0.0,
			0.5, -0.5, 0.0
		];
		let vertices_u8: &[u8] = core::slice::from_raw_parts(
			vertices.as_ptr() as *const u8,
			vertices.len() * mem::size_of::<f32>(),
		);

		let mut vbo = 0;
		gl.GenBuffers(1, &mut vbo);
		gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl.BufferData(gl::ARRAY_BUFFER, vertices_u8.len() as gl::types::GLsizeiptr, vertices.as_ptr() as *const core::ffi::c_void, gl::STATIC_DRAW);

		let mut vao = 0;
		gl.GenBuffers(1, &mut vao);
		gl.BindBuffer(gl::VERTEX_ARRAY, vao);
		gl.EnableVertexAttribArray(0);
		gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8, 0 as *const core::ffi::c_void);

		// unbind
		gl.BindBuffer(gl::ARRAY_BUFFER, 0);
		gl.BindBuffer(gl::VERTEX_ARRAY, 0);

		// gl.Viewport(0, 0, 240, 240);

		loop {
			gl.ClearColor(0.1, 0.2, 0.3, 1.0);
			gl.Clear(gl::COLOR_BUFFER_BIT);

			gl.UseProgram(program);
			gl.BindBuffer(gl::VERTEX_ARRAY, vao);
			gl.DrawArrays(gl::TRIANGLES, 0, 3);

			egl.swap_buffers(display, surface)?;
		}
	}
}
