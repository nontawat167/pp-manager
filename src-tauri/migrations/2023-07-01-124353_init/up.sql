-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tags (
    id varchar(30) not null unique primary key,
    name varchar(30) not null unique,
    kind varchar(30) not null,
    color varchar(30) not null default 'primary'
);

CREATE TABLE IF NOT EXISTS skus (
    id varchar(30) not null unique primary key,
    created_at DATETIME not null,
    updated_at DATETIME not null,
    deleted_at DATETIME default null,
    name text not null,
    price int not null default 0,
    product_type varchar(30) not null,
    FOREIGN KEY(product_type) REFERENCES tags(id)
);

INSERT INTO
    tags (id, name, kind, color)
VALUES
    (
        '466c7a12-eb2a-4b1a-8298-a78f410c63af',
        'ยาง',
        'PRODUCT_TYPE',
        'primary'
    );

INSERT INTO
    tags (id, name, kind, color)
VALUES
    (
        'ba803a3a-92dc-4bd9-9f64-180b2bea5161',
        'แม็ก',
        'PRODUCT_TYPE',
        'primary'
    );