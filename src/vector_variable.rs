struct EducationBackground {
    education_name: &'static str,
    education_level: &'static str,
    graduated_at: i32,
    score: u32,
}

struct Address {
    country: &'static str,
    province: &'static str,
    city: &'static str,
    district: &'static str,
    sub_district: &'static str,
    address_detal: &'static str,
}

struct Employe {
    name: &'static str,
    education_background: EducationBackground,
    age: i32,
    address: Address,
}
