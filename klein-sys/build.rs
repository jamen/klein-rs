use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("Klein/c_src/klein_c.cpp")
        .cpp(true)
        .flag_if_supported("-msse3")
        .flag_if_supported("-msse4.1")
        .include("Klein/public")
        .compile("klein");

    let bindings = bindgen::Builder::default()
        .header("Klein/c_src/klein.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("kln_plane")
        .allowlist_type("kln_line")
        .allowlist_type("kln_rotor")
        .allowlist_type("kln_direction")
        .allowlist_type("kln_point")
        .allowlist_type("kln_rotor")
        .allowlist_type("kln_translator")
        .allowlist_type("kln_motor")
        .allowlist_function("kln_plane_init")
        .allowlist_function("kln_line_init")
        .allowlist_function("kln_point_init")
        .allowlist_function("kln_reflect_point")
        .allowlist_function("kln_reflect_line")
        .allowlist_function("kln_reflect_plane")
        .allowlist_function("kln_rotate_point")
        .allowlist_function("kln_rotate_line")
        .allowlist_function("kln_rotate_plane")
        .allowlist_function("kln_translate_point")
        .allowlist_function("kln_translate_line")
        .allowlist_function("kln_translate_plane")
        .allowlist_function("kln_motor_point")
        .allowlist_function("kln_motor_line")
        .allowlist_function("kln_motor_plane")
        .allowlist_function("kln_compose_rotors")
        .allowlist_function("kln_compose_translators")
        .allowlist_function("kln_compose_rotor_translator")
        .allowlist_function("kln_compose_translator_rotor")
        .allowlist_function("kln_compose_motors")
        .allowlist_function("motor_log")
        .allowlist_function("line_exp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write the bindings");
}
