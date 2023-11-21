-- Your SQL goes here
CREATE TABLE brands (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL
);

CREATE TABLE departments (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL
);

CREATE TABLE sub_departments (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    department_id INTEGER REFERENCES departments(id)
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    price DOUBLE PRECISION,
    description VARCHAR NOT NULL,
    images TEXT[] NOT NULL,
    brand_id INTEGER REFERENCES brands(id),
    department_id INTEGER REFERENCES departments(id)
);

CREATE TABLE product_sub_departments (
    product_id INTEGER NOT NULL REFERENCES products(id),
    sub_department_id INTEGER NOT NULL REFERENCES sub_departments(id),
    PRIMARY KEY (product_id, sub_department_id)
);