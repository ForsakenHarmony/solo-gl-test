use gl_generator::{Registry, Api, Profile, Fallbacks, StaticStructGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
	println!("cargo:rerun-if-changed=build.rs");

	let dest = env::var("OUT_DIR").unwrap();
	let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

	Registry::new(Api::Gles2, (2, 0), Profile::Core, Fallbacks::All, [
		// "GL_OES_rgb8_rgba8",
		// "GL_OES_depth24",
		// "GL_OES_vertex_half_float",
		// "GL_OES_texture_float",
		// "GL_OES_texture_half_float",
		// "GL_OES_element_index_uint",
		// "GL_OES_mapbuffer",
		// "GL_OES_fragment_precision_high",
		// "GL_OES_compressed_ETC1_RGB8_texture",
		// "GL_OES_EGL_image",
		// "GL_OES_EGL_image_external",
		// "GL_OES_required_internalformat",
		// "GL_OES_depth_texture",
		// "GL_OES_get_program_binary",
		// "GL_OES_packed_depth_stencil",
		// "GL_OES_standard_derivatives",
		// "GL_OES_vertex_array_object",
		// "GL_OES_egl_sync",
		// "GL_OES_surfaceless_context",
		// "GL_EXT_discard_framebuffer",
		// "GL_EXT_blend_minmax",
		// "GL_EXT_multi_draw_arrays",
		// "GL_EXT_multisampled_render_to_texture",
		// "GL_EXT_shader_texture_lod",
		// "GL_EXT_texture_format_BGRA8888",
		// "GL_EXT_texture_rg",
		// "GL_IMG_shader_binary",
		// "GL_IMG_texture_compression_pvrtc",
		// "GL_IMG_texture_npot",
		// "GL_IMG_texture_format_BGRA8888",
		// "GL_IMG_read_format",
		// "GL_IMG_program_binary",
		// "GL_IMG_uniform_buffer_object",
		// "GL_IMG_multisampled_render_to_texture",
		"GL_KHR_debug"
	])
		.write_bindings(StaticStructGenerator, &mut file)
		.unwrap();
}
