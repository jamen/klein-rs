use std::mem::MaybeUninit;

use klein_sys::{
    kln_compose_motors, kln_compose_rotor_translator, kln_compose_rotors,
    kln_compose_translator_rotor, kln_compose_translators, kln_direction, kln_line, kln_line_init,
    kln_motor, kln_motor_line, kln_motor_plane, kln_motor_point, kln_plane, kln_plane_init,
    kln_point, kln_point_init, kln_reflect_plane, kln_reflect_point, kln_rotate_line,
    kln_rotate_plane, kln_rotate_point, kln_rotor, kln_translator, line_exp, motor_log,
};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Plane(kln_plane);

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        unsafe {
            let mut plane: MaybeUninit<kln_plane> = MaybeUninit::zeroed();
            kln_plane_init(plane.as_mut_ptr(), a, b, c, d);
            Self(plane.assume_init())
        }
    }

    #[inline]
    pub fn reflect(&self, plane: &Plane) -> Self {
        unsafe { Self(kln_reflect_plane(plane.0, self.0)) }
    }

    #[inline]
    pub fn rotate(&self, rotor: &Rotor) -> Self {
        unsafe { Self(kln_rotate_plane(rotor.0, self.0)) }
    }

    #[inline]
    pub fn translate(&self, translator: &Translator) -> Self {
        unsafe { Self(kln_translate_plane(translator.0, self.0)) }
    }

    #[inline]
    pub fn motor(&self, motor: &Motor) -> Self {
        unsafe { Self(kln_motor_plane(motor.0, self.0)) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Line(kln_line);

impl Line {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        unsafe {
            let mut line: MaybeUninit<kln_line> = MaybeUninit::zeroed();
            kln_line_init(line.as_mut_ptr(), a, b, c, d, e, f);
            Self(line.assume_init())
        }
    }

    #[inline]
    pub fn exp(&self) -> Motor {
        unsafe { Motor(line_exp(self.0)) }
    }

    #[inline]
    pub fn reflect(&self, plane: &Plane) -> Self {
        unsafe { Self(kln_reflect_line(plane.0, self.0)) }
    }

    #[inline]
    pub fn rotate(&self, rotor: &Rotor) -> Self {
        unsafe { Self(kln_rotate_line(rotor.0, self.0)) }
    }

    #[inline]
    pub fn translate(&self, translator: &Translator) -> Self {
        unsafe { Self(kln_translate_line(translator.0, self.0)) }
    }

    #[inline]
    pub fn motor(&self, motor: &Motor) -> Self {
        unsafe { Self(kln_motor_line(motor.0, self.0)) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Point(kln_point);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        unsafe {
            let mut point: MaybeUninit<kln_point> = MaybeUninit::zeroed();
            kln_point_init(point.as_mut_ptr(), x, y, z);
            Self(point.assume_init())
        }
    }

    #[inline]
    pub fn reflect(&self, plane: &Plane) -> Self {
        unsafe { Self(kln_reflect_point(plane.0, self.0)) }
    }

    #[inline]
    pub fn rotate(&self, rotor: &Rotor) -> Self {
        unsafe { Self(kln_rotate_point(rotor.0, self.0)) }
    }

    #[inline]
    pub fn translate(&self, translator: &Translator) -> Self {
        unsafe { Self(kln_translate_point(translator.0, self.0)) }
    }

    #[inline]
    pub fn motor(&self, motor: &Motor) -> Self {
        unsafe { Self(kln_motor_point(motor.0, self.0)) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Direction(kln_direction);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Translator(kln_translator);

impl Translator {
    #[inline]
    pub fn compose_translators(&self, translator: &Self) -> Self {
        unsafe { Self(kln_compose_translators(rotor.0, self.0)) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Motor(kln_motor);

impl Motor {
    #[inline]
    pub fn log(&self) -> Line {
        unsafe { Line(motor_log(self.0)) }
    }

    #[inline]
    pub fn compose_motors(&self, motor: &Self) -> Self {
        unsafe { Self(kln_compose_motors(motor.0, self.0)) }
    }

    #[inline]
    pub fn compose_translator_rotor(translator: &Translator, rotor: &Rotor) -> Self {
        unsafe { Self(kln_compose_translator_rotor(translator.0, rotor.0)) }
    }

    #[inline]
    pub fn compose_rotor_translator(rotor: &Rotor, translator: &Translator) -> Self {
        unsafe { Self(kln_compose_rotor_translator(rotor.0, translator.0)) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Rotor(kln_rotor);

impl Rotor {
    #[inline]
    fn compose_rotors(&self, rotor: &Self) -> Self {
        unsafe { Self(kln_compose_rotors(rotor.0, self.0)) }
    }
}
