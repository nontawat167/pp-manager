CREATE TABLE IF NOT EXISTS sku (
    id varchar(40) not null unique primary key,
    createdAt DATETIME not null,
    updatedAt DATETIME not null,
    name text not null,
    price int not null,
    type varchar(10) not null
)