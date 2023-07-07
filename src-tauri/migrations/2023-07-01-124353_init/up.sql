-- Your SQL goes here
CREATE TABLE IF NOT EXISTS skus (
    id varchar(40) not null unique primary key,
    created_at DATETIME not null,
    updated_at DATETIME not null,
    deleted_at DATETIME default null,
    name text not null,
    price int not null default 0,
    product_type varchar(10) not null
)