-- Your SQL goes here
CREATE TABLE brands (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    url_name VARCHAR(255) NOT NULL
);

CREATE TABLE departments (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    url_name VARCHAR(255) NOT NULL,
    color VARCHAR(7) NOT NULL
);

CREATE TABLE sub_departments (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    url_name VARCHAR(255) NOT NULL,
    department_id INTEGER REFERENCES departments(id)
);

CREATE TABLE products (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    url_name VARCHAR(255) NOT NULL,
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