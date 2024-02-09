CREATE TABLE IF NOT EXISTS slangwords (
    id serial primary key,
    word varchar(255) not null,
    description varchar not null
);