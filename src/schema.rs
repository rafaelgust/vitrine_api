// @generated automatically by Diesel CLI.

diesel::table! {
    brands (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        url_name -> Varchar,
    }
}

diesel::table! {
    departments (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        url_name -> Varchar,
        #[max_length = 20]
        color -> Nullable<Varchar>,
    }
}

diesel::table! {
    product_sub_departments (product_id, sub_department_id) {
        product_id -> Int4,
        sub_department_id -> Int4,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        url_name -> Varchar,
        price -> Nullable<Float8>,
        description -> Varchar,
        images -> Array<Nullable<Text>>,
        brand_id -> Nullable<Int4>,
        department_id -> Nullable<Int4>,
    }
}

diesel::table! {
    sub_departments (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        url_name -> Varchar,
        department_id -> Nullable<Int4>,
    }
}

diesel::joinable!(product_sub_departments -> products (product_id));
diesel::joinable!(product_sub_departments -> sub_departments (sub_department_id));
diesel::joinable!(products -> brands (brand_id));
diesel::joinable!(products -> departments (department_id));
diesel::joinable!(sub_departments -> departments (department_id));

diesel::allow_tables_to_appear_in_same_query!(
    brands,
    departments,
    product_sub_departments,
    products,
    sub_departments,
);
