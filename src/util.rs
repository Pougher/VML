#[repr(C)]
union Conversion_union {
    x: f64,
    y: u64,
    z: i64,
}

pub fn to_u64(value: f64) -> u64 {
    let c = Conversion_union { x: value };
    return unsafe { c.y }
}

pub fn to_f64(value: u64) -> f64 {
    let c = Conversion_union { y: value };
    return unsafe { c.x }
}

pub fn i64_bits(value: i64) -> u64 {
    let c = Conversion_union { z: value };
    return unsafe { c.y }
}
