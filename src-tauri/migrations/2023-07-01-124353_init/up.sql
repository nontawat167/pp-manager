-- Your SQL goes here
CREATE TABLE IF NOT EXISTS skus (
    id varchar(40) not null unique primary key,
    createdAt DATETIME not null,
    updatedAt DATETIME not null,
    name text not null,
    price int not null,
    productType varchar(10) not null
)