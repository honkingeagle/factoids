CREATE TABLE IF NOT EXISTS slangwords (
    id serial primary key,
    word varchar(50) not null,
    synonym varchar(50) not null,
    description varchar not null
);